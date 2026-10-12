use bevy::math::{DQuat, DVec3};
use solar_survival::{
    physics::{radial_gravity, stop_before_sphere},
    planets::{CubeFace, PatchKey},
    player::Explorer,
    universe::{
        MAX_JD, MIN_JD, SimulationClock,
        catalog::SolarSystem,
        celestial_body::{BodyId, BodyKind},
        coordinates::{ReferenceFrame, local_to_render, render_proxy},
        orbital_mechanics::{eccentric_anomaly, kepler_period, kepler_position, moon_relative},
        units::{AU, DAY, J2000, SUN_MU, au_to_m, m_to_au},
    },
};
fn near(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "actual={actual}, expected={expected}, tolerance={tolerance}"
    );
}
#[test]
fn astronomical_unit_is_exact_iau_definition() {
    assert_eq!(au_to_m(1.0), 149_597_870_700.0);
    near(m_to_au(au_to_m(30.06992276)), 30.06992276, 1e-14);
}
#[test]
fn interplanetary_distance_retains_small_offsets() {
    let a = DVec3::new(4.5e12, 1e12, -3e12);
    near(a.distance(a + DVec3::new(3.0, 4.0, 0.0)), 5.0, 1e-12);
}
#[test]
fn circular_orbit_quarter_turn_and_inclination() {
    let p = kepler_position(AU, 0.0, 0.0, 0.0, 0.0, std::f64::consts::FRAC_PI_2);
    near(p.x, 0.0, 0.0001);
    near(p.y, 0.0, 0.0001);
    near(p.z, -AU, 0.0001);
    let tilted = kepler_position(
        AU,
        0.0,
        std::f64::consts::FRAC_PI_2,
        0.0,
        0.0,
        std::f64::consts::FRAC_PI_2,
    );
    near(tilted.y, AU, 0.0001);
}
#[test]
fn elliptical_orbit_has_correct_perihelion_and_aphelion() {
    near(
        kepler_position(AU, 0.2, 0.0, 0.0, 0.0, 0.0).length(),
        AU * 0.8,
        0.0001,
    );
    near(
        kepler_position(AU, 0.2, 0.0, 0.0, 0.0, std::f64::consts::PI).length(),
        AU * 1.2,
        0.0001,
    );
}
#[test]
fn kepler_solver_residual_for_all_catalogue_eccentricities() {
    for eccentricity in [0.0, 0.0068, 0.0549, 0.0934, 0.2056, 0.9, 0.999] {
        for mean in [-3.0, -1.7, -0.0001, 0.0, 0.0001, 0.9, 3.0] {
            let e = eccentric_anomaly(mean, eccentricity);
            near(e - eccentricity * e.sin(), mean, 1e-12);
        }
    }
}
#[test]
fn solar_gravity_predicts_earth_year() {
    near(kepler_period(AU, SUN_MU) / DAY, 365.2569, 0.001);
}
#[test]
fn periods_match_jpl_sidereal_years_approximately() {
    let solar = SolarSystem::default();
    let days = [
        87.969, 224.701, 365.256, 686.980, 4332.589, 10759.22, 30685.4, 60189.0,
    ];
    for (body, expected) in solar.bodies[1..9].iter().zip(days) {
        // Taux des éléments ajustés → période anomalistique, distincte de la période sidérale.
        near(
            body.orbit.unwrap().period_seconds() / DAY,
            expected,
            expected * 0.015,
        );
    }
}
#[test]
fn catalogue_contains_ten_unique_bodies_and_real_dimensions() {
    let solar = SolarSystem::default();
    assert_eq!(solar.bodies.len(), 10);
    for (i, body) in solar.bodies.iter().enumerate() {
        assert_eq!(body.id.0, i);
        assert!(body.mass_kg > 0.0);
        assert!(body.gravitational_parameter > 0.0);
    }
    near(solar.body(BodyId::EARTH).radius_m, 6_371_008.4, 1e-8);
    assert_eq!(solar.body(BodyId::MARS).radius_m, 3_389_500.0);
    assert_eq!(solar.body(BodyId(5)).kind, BodyKind::GasGiant);
    let neptune = solar.position(BodyId(8), J2000).length();
    assert!((4.4e12..4.6e12).contains(&neptune));
}
#[test]
fn earth_moon_barycentre_preserved() {
    let solar = SolarSystem::default();
    let earth = solar.position(BodyId::EARTH, J2000);
    let moon = solar.position(BodyId::MOON, J2000);
    let earth_mass = solar.body(BodyId::EARTH).mass_kg;
    let moon_mass = solar.body(BodyId::MOON).mass_kg;
    let weighted = (earth * earth_mass + moon * moon_mass) / (earth_mass + moon_mass);
    let jpl_bary = solar.body(BodyId::EARTH).orbit.unwrap().position(J2000);
    assert!(weighted.distance(jpl_bary) < 0.0001);
    near(earth.distance(moon), 384_400_000.0 * (1.0 - 0.0549), 0.0001);
}
#[test]
fn moon_returns_after_sidereal_period() {
    assert!(moon_relative(J2000).distance(moon_relative(J2000 + 27.3217)) < 0.03);
}
#[test]
fn positions_finite_through_valid_jpl_interval() {
    let solar = SolarSystem::default();
    for jd in [MIN_JD, J2000, 2_461_325.5, MAX_JD] {
        for body in &solar.bodies {
            let p = solar.position(body.id, jd);
            assert!(p.is_finite());
            assert!(p.length() < 5e12);
        }
    }
}
#[test]
fn orbit_changes_with_time_without_rendering() {
    let solar = SolarSystem::default();
    let movement = solar
        .position(BodyId::EARTH, J2000 + 1.0)
        .distance(solar.position(BodyId::EARTH, J2000));
    assert!((2.4e9..2.7e9).contains(&movement));
}
#[test]
fn reference_frame_roundtrip_near_neptune_preserves_submillimetres() {
    let frame = ReferenceFrame {
        origin: DVec3::new(4.5e12, 0.0, 1e12),
        orientation: DQuat::from_rotation_y(0.71),
    };
    let local = DVec3::new(3.123, 1.73, -12.8);
    assert!(frame.to_local(frame.to_global(local)).distance(local) < 0.001);
}
#[test]
fn frame_composition_and_conversion() {
    let planet = ReferenceFrame {
        origin: DVec3::new(AU, 0.0, 0.0),
        orientation: DQuat::from_rotation_z(0.2),
    };
    let zone = ReferenceFrame {
        origin: DVec3::new(1000.0, 0.0, 0.0),
        orientation: DQuat::from_rotation_x(0.4),
    };
    let local = DVec3::new(1.0, 2.0, 3.0);
    assert!(
        planet
            .compose(zone)
            .to_global(local)
            .distance(planet.to_global(zone.to_global(local)))
            < 1e-4
    );
    let converted = ReferenceFrame::convert(local, planet.compose(zone), planet);
    assert!(converted.distance(zone.to_global(local)) < 1e-4);
}
#[test]
fn render_proxies_preserve_angular_size_and_bound_coordinates() {
    for distance in [7e6, 1e9, AU, 4.5e12, 1e16] {
        let (p, r) = render_proxy(DVec3::new(distance, 0.0, 0.0), 1e6);
        assert!(p.is_finite());
        assert!(p.length() <= 10_001.0);
        near(
            (r / p.length()) as f64,
            1e6 / distance,
            (1e6 / distance) * 2e-7,
        );
    }
    assert!(local_to_render(DVec3::new(0.001, 1.0, 99999.0)).is_some());
    assert!(local_to_render(DVec3::splat(1e12)).is_none());
    assert!(local_to_render(DVec3::splat(f64::NAN)).is_none());
}
#[test]
fn simulation_clock_is_independent_of_frame_rate_and_pauses() {
    let mut a = SimulationClock {
        jd: J2000,
        speed_index: 3,
        paused: false,
    };
    let mut b = SimulationClock {
        jd: J2000,
        speed_index: 3,
        paused: false,
    };
    for _ in 0..60 {
        a.advance(1.0 / 60.0);
    }
    for _ in 0..144 {
        b.advance(1.0 / 144.0);
    }
    near(a.jd, b.jd, 1e-7);
    near((a.jd - J2000) * DAY, 1000.0, 0.005);
    a.paused = true;
    let before = a.jd;
    a.advance(500.0);
    assert_eq!(a.jd, before);
    a.jd = MAX_JD - 1e-6;
    a.paused = false;
    a.advance(1000.0);
    assert_eq!(a.jd, MAX_JD);
    assert!(a.paused);
}
#[test]
fn swept_collision_prevents_hypervelocity_tunnelling() {
    let radius = 3_389_500.0 + 1000.0;
    let start = DVec3::new(-1e12, 0.0, 0.0);
    let delta = DVec3::new(2e12, 0.0, 0.0);
    let hit = stop_before_sphere(start, delta, DVec3::ZERO, radius).unwrap();
    assert!((start + delta * hit).length() >= radius);
    assert!(
        stop_before_sphere(start + DVec3::Y * radius * 2.0, delta, DVec3::ZERO, radius).is_none()
    );
    assert!(stop_before_sphere(start, -delta, DVec3::ZERO, radius).is_none());
}
#[test]
fn gravity_points_toward_mars_in_any_orientation() {
    let solar = SolarSystem::default();
    let mars = solar.body(BodyId::MARS);
    for axis in [DVec3::X, DVec3::Y, -DVec3::Z] {
        let g = radial_gravity(axis * mars.radius_m, mars.gravitational_parameter);
        assert!(g.normalize().distance(-axis) < 1e-12);
        near(g.length(), 3.728, 0.01);
    }
}
#[test]
fn visits_and_frame_detach_preserve_global_location() {
    let solar = SolarSystem::default();
    let mut pilot = Explorer::default();
    for body in &solar.bodies {
        pilot.selected = body.id;
        pilot.visit(&solar, J2000);
        let global = pilot.global(&solar, J2000);
        near(
            global.distance(solar.position(body.id, J2000)),
            body.radius_m * if body.id == BodyId(6) { 5.5 } else { 3.5 },
            0.001,
        );
        let forward = (pilot.rotation * bevy::math::Vec3::NEG_Z).as_dvec3();
        assert!(forward.dot((solar.position(body.id, J2000) - global).normalize()) > 0.99999);
        pilot.anchor = None;
        pilot.local_m = global;
        assert_eq!(pilot.global(&solar, J2000), global);
    }
}
#[test]
fn cube_sphere_patches_are_valid_reproducible_and_bounded() {
    let patch = PatchKey {
        face: CubeFace::PositiveY,
        level: 8,
        x: 12,
        y: 140,
    };
    near(patch.center_direction().unwrap().length(), 1.0, 1e-15);
    assert_eq!(patch.deterministic_seed(42), patch.deterministic_seed(42));
    assert_ne!(patch.deterministic_seed(42), patch.deterministic_seed(43));
    assert!(PatchKey { x: 256, ..patch }.center_direction().is_none());
    assert!(
        PatchKey {
            level: 255,
            ..patch
        }
        .center_direction()
        .is_none()
    );
}

#[test]
fn jpl_validity_boundaries_match_gregorian_dates() {
    // Entre 2000-01-01 et 2050-01-01 : 50 années, dont 13 bissextiles.
    assert_eq!(MAX_JD, J2000 + (50.0 * 365.0 + 13.0) - 0.5);
    // Entre 1800-01-01 et 2000-01-01 : 200 années, dont 48 bissextiles.
    assert_eq!(MIN_JD, J2000 - (200.0 * 365.0 + 48.0) - 0.5);
}
