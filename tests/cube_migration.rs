use bevy::{
    math::{DQuat, DVec3},
    prelude::*,
};
use solar_survival::{
    physics::{
        radial_gravity,
        surface::{self, HALF_WIDTH_M},
    },
    planets::{CubeFace, terrain::MarsTerrain},
    player::{Explorer, surface::SurfaceState},
    universe::{catalog::SolarSystem, celestial_body::BodyId},
};
fn terrain(face: CubeFace) -> MarsTerrain {
    let mut terrain = MarsTerrain {
        radius_m: 3389500.0,
        seed: 42,
        orientation: DQuat::IDENTITY,
        revision: 0,
        cube_enabled: true,
        migration_enabled: true,
        site_obstacles: None,
        cube: None,
    };
    terrain.set_site(face.direction(0.0, 0.0).unwrap());
    terrain
}
fn mu() -> f64 {
    SolarSystem::default()
        .body(BodyId::MARS)
        .gravitational_parameter
}
#[test]
fn transaction_preserves_body_feet_eyes_view_velocity_gravity_and_obstacles() {
    for face in CubeFace::ALL {
        for altitude in [0.0, 2.0, 20.0] {
            let mut terrain = terrain(face);
            let mut state = SurfaceState {
                feet: DVec3::new(100.01, 0.0, 30.0),
                velocity: DVec3::new(8.0, 2.4, -1.0),
                yaw: 0.7,
                pitch: -0.4,
                accumulator: 0.004,
                jump_pending: true,
                ..default()
            };
            state.feet.y = terrain.support_height(state.feet, HALF_WIDTH_M) + 0.002 + altitude;
            state.grounded = altitude == 0.0;
            let feet = terrain.body_relative(state.feet);
            let eye = terrain.body_relative(state.eye(&terrain));
            let view = terrain.orientation * state.local_rotation(&terrain);
            let velocity = terrain.orientation * state.velocity;
            let gravity =
                terrain.orientation * radial_gravity(terrain.site_relative(state.feet), mu());
            let floor = terrain.body_relative(DVec3::new(
                state.feet.x,
                terrain.support_height(state.feet, HALF_WIDTH_M),
                state.feet.z,
            ));
            let obstacles: Vec<_> = (0..3)
                .map(|i| {
                    let (p, size, q) = surface::obstacle_pose(&terrain, i);
                    (terrain.body_relative(p), size, terrain.orientation * q)
                })
                .collect();
            let revision = terrain.revision;
            assert!(state.migrate(&mut terrain));
            assert_eq!(terrain.revision, revision + 1);
            assert_eq!(terrain.cube.as_ref().unwrap().pair.keys[0].x, 16384);
            assert!(feet.distance(terrain.body_relative(state.feet)) < 3e-9);
            assert!(eye.distance(terrain.body_relative(state.eye(&terrain))) < 3e-9);
            assert!(velocity.distance(terrain.orientation * state.velocity) < 1e-13);
            assert!(
                gravity.distance(
                    terrain.orientation * radial_gravity(terrain.site_relative(state.feet), mu())
                ) < 1e-13
            );
            for axis in [DVec3::X, DVec3::Y, DVec3::Z] {
                assert!(
                    (view * axis)
                        .distance(terrain.orientation * state.local_rotation(&terrain) * axis)
                        < 1e-13
                );
            }
            let next_floor = terrain.body_relative(DVec3::new(
                state.feet.x,
                terrain.support_height(state.feet, HALF_WIDTH_M),
                state.feet.z,
            ));
            // En l'air, la projection verticale change horizontalement avec la carte.
            // Comparer le rayon du support et l'écart vertical, pas ces deux projections.
            assert!((floor.length() - next_floor.length()).abs() < 0.000001);
            if altitude == 0.0 {
                assert!(floor.distance(next_floor) < 0.00001);
            }
            assert!(
                (state.feet.y
                    - terrain.support_height(state.feet, HALF_WIDTH_M)
                    - 0.002
                    - altitude)
                    .abs()
                    < 0.00001
            );
            assert_eq!(state.accumulator, 0.004);
            assert!(state.jump_pending);
            assert_eq!(state.grounded, altitude == 0.0);
            for (i, (p, size, q)) in obstacles.into_iter().enumerate() {
                let (p2, size2, q2) = surface::obstacle_pose(&terrain, i);
                assert!(p.distance(terrain.body_relative(p2)) < 3e-9);
                assert_eq!(size, size2);
                for axis in [DVec3::X, DVec3::Y, DVec3::Z] {
                    assert!((q * axis).distance(terrain.orientation * q2 * axis) < 1e-13);
                }
                let solid = surface::obstacles(&terrain)[i];
                assert!((solid.max - solid.min - size).max_element() < 0.00031);
            }
            assert!(!state.migrate(&mut terrain)); // hystérésis après remplacement
        }
    }
}
#[test]
fn real_walk_run_return_and_airborne_migrations_at_three_frame_rates() {
    for fps in [30, 60, 144] {
        for speed in [surface::WALK_MPS, surface::RUN_MPS] {
            let mut terrain = terrain(CubeFace::PositiveY);
            let mut state = SurfaceState {
                feet: DVec3::new(96.0, 0.0, 30.0),
                ..default()
            };
            state.feet.y = terrain.support_height(state.feet, HALF_WIDTH_M) + 0.002;
            state.grounded = true;
            let pilot = Explorer {
                captured: true,
                boosted: speed == surface::RUN_MPS,
                ..default()
            };
            // Le mode surface détermine la vitesse.
            let pilot = Explorer {
                mode: solar_survival::player::MovementMode::Surface,
                ..pilot
            };
            let mut keys = ButtonInput::default();
            let first = terrain.revision;
            for (leg, key) in [KeyCode::KeyD, KeyCode::KeyA, KeyCode::KeyD, KeyCode::KeyA]
                .into_iter()
                .enumerate()
            {
                keys.reset_all();
                keys.press(key);
                if leg == 2 {
                    keys.press(KeyCode::Space);
                }
                let mut air_migration = false;
                let mut apex: f64 = 0.0;
                for _ in 0..fps * 4 {
                    let old_revision = terrain.revision;
                    let old_body = terrain.body_relative(state.feet);
                    state.advance(&pilot, &mut terrain, &keys, 1.0 / fps as f64, mu());
                    keys.clear();
                    let floor = terrain.support_height(state.feet, HALF_WIDTH_M);
                    assert!(state.feet.y >= floor - 1e-8);
                    assert!(
                        old_body.distance(terrain.body_relative(state.feet))
                            < (speed * speed + 16.0).sqrt() * (120.0 / fps as f64).ceil() / 120.0
                                + 0.002
                    );
                    if terrain.revision != old_revision {
                        air_migration |= !state.grounded;
                    }
                    apex = apex.max(state.feet.y - floor);
                    if leg != 2 {
                        assert!(state.grounded);
                    }
                }
                if leg == 2 {
                    assert!(air_migration);
                    assert!((2.0..2.2).contains(&apex));
                }
                assert!(state.grounded);
                assert_eq!(
                    terrain.cube.as_ref().unwrap().pair.keys[0].x,
                    if leg % 2 == 0 { 16384 } else { 16383 }
                );
            }
            assert_eq!(terrain.revision, first + 4);
            keys.reset_all();
            let rest = terrain.body_relative(state.feet);
            for _ in 0..fps * 2 {
                state.advance(&pilot, &mut terrain, &keys, 1.0 / fps as f64, mu());
            }
            assert!(rest.distance(terrain.body_relative(state.feet)) < 1e-7);
            let solar = SolarSystem::default();
            let mut pilot = pilot;
            state.accumulator = 0.0;
            let feet = state.feet;
            state.advance(&pilot, &mut terrain, &keys, 0.001, mu());
            state.sync(
                &mut pilot,
                &terrain,
                solar.body(BodyId::MARS).rotation(12000.0),
            );
            assert_eq!(feet, state.feet);
            assert!(
                pilot.local_m.distance(
                    solar.body(BodyId::MARS).rotation(12000.0)
                        * terrain.body_relative(state.eye(&terrain))
                ) < 3e-9
            );
        }
    }
}
#[test]
fn migration_corridor_and_terminal_bounds_keep_airborne_footprint_supported() {
    let mut terrain = terrain(CubeFace::NegativeZ);
    let mut state = SurfaceState {
        feet: DVec3::new(101.0, 20.0, 60.0),
        ..default()
    };
    assert!(!state.migrate(&mut terrain));
    state.feet.z = 30.0;
    assert!(state.migrate(&mut terrain));
    let cube = terrain.cube.as_ref().unwrap();
    state.feet.x = cube.max.x - 0.31;
    let mut grounded = false;
    let mut velocity = DVec3::ZERO;
    for _ in 0..600 {
        surface::step(
            &terrain,
            &mut state.feet,
            &mut velocity,
            &mut grounded,
            DVec3::X * 8.0,
            false,
            mu(),
        );
        assert!(
            terrain
                .cube
                .as_ref()
                .unwrap()
                .support_height(state.feet, HALF_WIDTH_M)
                .is_some()
        );
        assert!(!state.migrate(&mut terrain));
    }
    assert!(grounded);
    assert!(velocity.x.abs() < 1e-8);
}
