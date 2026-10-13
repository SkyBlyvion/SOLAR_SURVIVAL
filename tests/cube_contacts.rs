use bevy::{
    math::{DQuat, DVec3},
    prelude::*,
};
use solar_survival::{
    physics::surface::{self, HALF_WIDTH_M},
    planets::terrain::MarsTerrain,
    player::{Explorer, surface::SurfaceState},
    universe::{catalog::SolarSystem, celestial_body::BodyId, units::J2000},
};

fn terrain() -> MarsTerrain {
    let mut terrain = MarsTerrain {
        radius_m: 3389500.0,
        seed: 42,
        orientation: DQuat::IDENTITY,
        revision: 0,
        cube_enabled: true,
        migration_enabled: false,
        offcenter_enabled: false,
        migration: None,
        site_obstacles: None,
        cube: None,
    };
    terrain.set_site(DVec3::Y);
    terrain
}
fn mu() -> f64 {
    SolarSystem::default()
        .body(BodyId::MARS)
        .gravitational_parameter
}

#[test]
fn walk_run_both_ways_rest_and_jump_across_seam_without_contact_loss() {
    let terrain = terrain();
    for speed in [surface::WALK_MPS, surface::RUN_MPS] {
        for z in [-40.0, 0.0, 40.0] {
            let mut feet = DVec3::new(-8.0, 0.0, z);
            feet.y = terrain.support_height(feet, HALF_WIDTH_M) + 0.002;
            let mut velocity = DVec3::ZERO;
            let mut grounded = true;
            for direction in [1.0, -1.0, 1.0, -1.0] {
                for _ in 0..480 {
                    surface::step(
                        &terrain,
                        &mut feet,
                        &mut velocity,
                        &mut grounded,
                        DVec3::X * speed * direction,
                        false,
                        mu(),
                    );
                    assert!(grounded);
                    assert!(
                        (feet.y - terrain.support_height(feet, HALF_WIDTH_M) - 0.002).abs() < 1e-9
                    );
                }
                assert!(feet.x * direction > 7.9);
            }
            feet.x = 0.0;
            feet.y = terrain.support_height(feet, HALF_WIDTH_M) + 0.002;
            let rest = feet;
            for _ in 0..1200 {
                surface::step(
                    &terrain,
                    &mut feet,
                    &mut velocity,
                    &mut grounded,
                    DVec3::ZERO,
                    false,
                    mu(),
                );
            }
            assert!(feet.distance(rest) < 1e-9 && velocity.length() < 1e-9);
            feet.x = -4.0;
            let mut peak: f64 = 0.0;
            for tick in 0..480 {
                surface::step(
                    &terrain,
                    &mut feet,
                    &mut velocity,
                    &mut grounded,
                    DVec3::X * speed,
                    tick == 0,
                    mu(),
                );
                let floor = terrain.support_height(feet, HALF_WIDTH_M);
                assert!(feet.y >= floor);
                peak = peak.max(feet.y - floor);
            }
            assert!((2.0..2.2).contains(&peak) && grounded && feet.x > 13.9);
        }
    }
}

#[test]
fn all_four_loaded_boundaries_keep_full_footprint_supported_even_airborne() {
    let terrain = terrain();
    let cube = terrain.cube.as_ref().unwrap();
    for (start, desired) in [
        (DVec3::new(cube.max.x - 0.31, 20.0, 0.0), DVec3::X),
        (DVec3::new(cube.min.x + 0.31, 20.0, 0.0), DVec3::NEG_X),
        (DVec3::new(0.0, 20.0, cube.max.z - 0.31), DVec3::Z),
        (DVec3::new(0.0, 20.0, cube.min.z + 0.31), DVec3::NEG_Z),
    ] {
        let mut feet = start;
        let mut velocity = DVec3::ZERO;
        let mut grounded = false;
        for _ in 0..600 {
            surface::step(
                &terrain,
                &mut feet,
                &mut velocity,
                &mut grounded,
                desired * surface::RUN_MPS,
                false,
                mu(),
            );
            assert!(cube.support_height(feet, HALF_WIDTH_M).is_some());
        }
        assert!(grounded && feet.is_finite());
        assert!(velocity.dot(desired).abs() < 1e-8);
    }
}

#[test]
fn cube_walk_jump_frame_rates_and_rotating_publication_without_substep() {
    let solar = SolarSystem::default();
    let mut results = Vec::new();
    for fps in [30, 60, 144] {
        let mut terrain = terrain();
        let mut pilot = Explorer::default();
        let mut state = SurfaceState::default();
        state.enter(&mut pilot, &mut terrain, &solar, J2000);
        state.yaw = 0.0;
        pilot.captured = true;
        let mut keys = ButtonInput::default();
        keys.press(KeyCode::KeyD);
        keys.press(KeyCode::Space);
        for _ in 0..fps * 4 {
            state.advance(&pilot, &mut terrain, &keys, 1.0 / fps as f64, mu());
            keys.clear();
        }
        assert!(state.grounded && state.feet.x > 9.9);
        results.push(state.feet);
        let feet = state.feet;
        state.accumulator = 0.0;
        state.advance(&pilot, &mut terrain, &keys, 0.001, mu());
        let rotation = solar.body(BodyId::MARS).rotation(12000.0);
        state.sync(&mut pilot, &terrain, rotation);
        assert_eq!(state.feet, feet);
        assert!(
            pilot
                .local_m
                .distance(rotation * terrain.body_relative(state.eye(&terrain)))
                < 2e-9
        );
    }
    assert!(results[0].distance(results[1]) < 1e-8 && results[1].distance(results[2]) < 1e-8);
}

#[test]
fn central_face_entry_is_lit_at_different_epochs() {
    let solar = SolarSystem::default();
    for jd in [J2000, J2000 + 0.37, 2461325.5] {
        let mut terrain = terrain();
        let mut pilot = Explorer::default();
        let mut state = SurfaceState::default();
        state.enter(&mut pilot, &mut terrain, &solar, jd);
        let sun = -solar.position(BodyId::MARS, jd).normalize();
        assert!(pilot.local_m.normalize().dot(sun) > 0.577);
        assert!(state.grounded);
    }
}
