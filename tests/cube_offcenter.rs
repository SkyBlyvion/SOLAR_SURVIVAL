use bevy::{
    math::{DQuat, DVec3},
    prelude::*,
};
use solar_survival::{
    physics::{
        radial_gravity,
        surface::{self, HALF_WIDTH_M},
    },
    planets::{
        CubeFace, PatchKey,
        cube_migration::{BOX_DRIFT_LIMIT_M, CubeMigration},
        terrain::MarsTerrain,
    },
    player::{Explorer, MovementMode, surface::SurfaceState},
    universe::{catalog::SolarSystem, celestial_body::BodyId},
};
const R: f64 = 3389500.0;
fn mu() -> f64 {
    SolarSystem::default()
        .body(BodyId::MARS)
        .gravitational_parameter
}
fn terrain(face: CubeFace) -> MarsTerrain {
    let mut terrain = MarsTerrain {
        building_solids: Vec::new(),
        radius_m: R,
        seed: 42,
        orientation: DQuat::IDENTITY,
        revision: 0,
        cube_enabled: true,
        migration_enabled: true,
        offcenter_enabled: true,
        migration: None,
        site_obstacles: None,
        cube: None,
        globe_enabled: false,
        globe: None,
    };
    terrain.set_site(face.direction(0.0, 0.0).unwrap());
    terrain
}
#[test]
fn certified_overlap_contains_entire_box_at_extreme_altitudes_on_six_faces() {
    for face in CubeFace::ALL {
        let terrain = terrain(face);
        let plan = terrain.migration.as_ref().unwrap();
        println!(
            "{face:?} charts {:?} center {} z {} angle {} drift {} index {:?}",
            plan.charts,
            plan.center_x,
            plan.half_z,
            plan.rotation_rad,
            plan.box_drift_m,
            terrain.cube.as_ref().unwrap().index_counts()
        );
        assert!(plan.box_drift_m <= BOX_DRIFT_LIMIT_M);
        assert!((0.00004..0.00007).contains(&plan.rotation_rad));
        for chart in plan.charts {
            for x in [-8.0, 8.0] {
                for y in [-100.0, 100.0] {
                    for z in [-plan.half_z, plan.half_z] {
                        let p =
                            chart.convert(DVec3::new(plan.center_x + x, y, z), plan.charts[0], R);
                        let half = HALF_WIDTH_M + BOX_DRIFT_LIMIT_M;
                        assert!(p.x - half >= chart.min.x && p.x + half <= chart.max.x);
                        assert!(p.z - half >= chart.min.z && p.z + half <= chart.max.z);
                    }
                }
            }
        }
        let (_, next) =
            solar_survival::planets::cube_terrain::CubeTerrain::from_key(plan.charts[1].key, R)
                .unwrap();
        for i in 0..=8 {
            for j in 0..=8 {
                for y in [-100.0, 0.0, 100.0] {
                    let entry = DVec3::new(
                        plan.center_x - 8.0 + 2.0 * i as f64,
                        y,
                        plan.half_z * (j as f64 / 4.0 - 1.0),
                    );
                    assert!(
                        terrain
                            .cube
                            .as_ref()
                            .unwrap()
                            .support_height(entry, HALF_WIDTH_M + BOX_DRIFT_LIMIT_M)
                            .is_some()
                    );
                    let p = plan.charts[1].convert(entry, plan.charts[0], R);
                    assert!(
                        next.support_height(p, HALF_WIDTH_M + BOX_DRIFT_LIMIT_M)
                            .is_some()
                    );
                }
            }
        }
        assert_eq!(plan.constructions, 2);
        // Une clé près d'un coin peut ne plus offrir de recouvrement suffisant.
        // Aucun plan n'est forcé si la boîte d'admission ne tient pas.
        let invalid = PatchKey {
            face,
            level: 15,
            x: 32767,
            y: 32767,
        };
        assert!(CubeMigration::prepare(invalid, R).is_none());
    }
}
#[test]
fn offcenter_transaction_and_return_preserve_pose_motion_support_and_static_objects() {
    for face in CubeFace::ALL {
        for altitude in [0.0, 2.0, 20.0, 99.0] {
            let mut terrain = terrain(face);
            let mut state = SurfaceState {
                velocity: DVec3::new(8.0, 2.4, -1.0),
                yaw: 0.7,
                pitch: -0.4,
                accumulator: 0.004,
                jump_pending: true,
                grounded: altitude == 0.0,
                ..default()
            };
            for direction in 0..2 {
                let plan = terrain.migration.as_ref().unwrap();
                state.feet = plan.preparation(terrain.cube.as_ref().unwrap().pair.keys[0]);
                state.feet.y = terrain.support_height(state.feet, HALF_WIDTH_M) + 0.002 + altitude;
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
                let obstacle_poses: Vec<_> = (0..3)
                    .map(|i| {
                        let (p, size, q) = surface::obstacle_pose(&terrain, i);
                        (terrain.body_relative(p), size, terrain.orientation * q)
                    })
                    .collect();
                let old_q = terrain.orientation;
                assert!(state.migrate(&mut terrain));
                assert_eq!(
                    terrain.cube.as_ref().unwrap().pair.keys[0].x,
                    if direction == 0 { 24576 } else { 24575 }
                );
                assert!(feet.distance(terrain.body_relative(state.feet)) < 4e-9);
                assert!(eye.distance(terrain.body_relative(state.eye(&terrain))) < 4e-9);
                assert!(velocity.distance(terrain.orientation * state.velocity) < 1e-13);
                assert!(
                    gravity.distance(
                        terrain.orientation
                            * radial_gravity(terrain.site_relative(state.feet), mu())
                    ) < 1e-13
                );
                for axis in [DVec3::X, DVec3::Y, DVec3::Z] {
                    assert!(
                        (view * axis)
                            .distance(terrain.orientation * state.local_rotation(&terrain) * axis)
                            < 1e-13
                    );
                }
                let new_floor = terrain.body_relative(DVec3::new(
                    state.feet.x,
                    terrain.support_height(state.feet, HALF_WIDTH_M),
                    state.feet.z,
                ));
                assert!((floor.length() - new_floor.length()).abs() < 2e-6);
                assert!(
                    (state.feet.y
                        - terrain.support_height(state.feet, HALF_WIDTH_M)
                        - 0.002
                        - altitude)
                        .abs()
                        < 2e-5
                );
                for x in [-0.3, 0.3] {
                    for y in [0.0, 1.8] {
                        for z in [-0.3, 0.3] {
                            let corner = DVec3::new(x, y, z);
                            assert!(
                                (old_q * corner).distance(terrain.orientation * corner)
                                    <= BOX_DRIFT_LIMIT_M
                            );
                        }
                    }
                }
                for (i, (p, size, q)) in obstacle_poses.into_iter().enumerate() {
                    let (p2, size2, q2) = surface::obstacle_pose(&terrain, i);
                    assert!(p.distance(terrain.body_relative(p2)) < 4e-9);
                    assert_eq!(size, size2);
                    let solid = surface::obstacles(&terrain)[i];
                    assert!((solid.max - solid.min - size).max_element() < 0.0005);
                    for x in [-0.5, 0.5] {
                        for y in [-0.5, 0.5] {
                            for z in [-0.5, 0.5] {
                                let corner = DVec3::new(x, y, z) * size;
                                let point = p2 + q2 * corner;
                                assert!((point + DVec3::splat(1e-9)).cmpge(solid.min).all());
                                assert!((point - DVec3::splat(1e-9)).cmple(solid.max).all());
                                assert!(
                                    (q * corner).distance(terrain.orientation * q2 * corner)
                                        < 1e-13
                                );
                            }
                        }
                    }
                }
                assert_eq!(state.accumulator, 0.004);
                assert!(state.jump_pending);
                assert_eq!(state.grounded, altitude == 0.0);
                assert!(!state.migrate(&mut terrain));
            }
            assert_eq!(terrain.migration.as_ref().unwrap().constructions, 4);
        }
    }
}
#[test]
fn refusals_and_hysteresis_do_not_build_candidates_or_mutate_state() {
    let mut terrain = terrain(CubeFace::PositiveY);
    let plan = terrain.migration.as_ref().unwrap();
    let center = plan.center_x;
    let hz = plan.half_z;
    let mut state = SurfaceState::default();
    for p in [
        DVec3::new(center, 0.0, 0.0),
        DVec3::new(center + 1.1, 0.0, hz + 1e-6),
        DVec3::new(center + 1.1, 100.00001, 0.0),
        DVec3::new(center + 8.00001, 0.0, 0.0),
        DVec3::splat(f64::NAN),
    ] {
        state.feet = p;
        let revision = terrain.revision;
        for _ in 0..1200 {
            assert!(!state.migrate(&mut terrain));
        }
        assert_eq!(terrain.revision, revision);
        assert_eq!(terrain.migration.as_ref().unwrap().constructions, 2);
    }
    state.feet = DVec3::new(center + 1.1, 2.0, 0.0);
    assert!(state.migrate(&mut terrain));
    for _ in 0..1200 {
        assert!(!state.migrate(&mut terrain));
    }
    assert_eq!(terrain.migration.as_ref().unwrap().constructions, 3);
    // Refus à proximité d'un objet : pas de reboîtage pendant un contact.
    state.feet = terrain
        .migration
        .as_ref()
        .unwrap()
        .preparation(terrain.cube.as_ref().unwrap().pair.keys[0]);
    let frame = terrain.orientation;
    let p = state.feet;
    terrain.site_obstacles = Some((
        frame,
        [surface::SolidBox {
            min: p - DVec3::ONE,
            max: p + DVec3::ONE,
        }; 3],
    ));
    for _ in 0..1200 {
        assert!(!state.migrate(&mut terrain));
    }
    assert_eq!(terrain.migration.as_ref().unwrap().constructions, 3);
}
#[test]
fn offcenter_walk_run_and_jump_return_at_three_frame_rates_keep_terminal_bounds() {
    for face in CubeFace::ALL {
        for fps in [30, 60, 144] {
            for boosted in [false, true] {
                let mut terrain = terrain(face);
                let mut state = SurfaceState {
                    feet: terrain
                        .migration
                        .as_ref()
                        .unwrap()
                        .approach(terrain.cube.as_ref().unwrap().pair.keys[0]),
                    ..default()
                };
                state.feet.y = terrain.support_height(state.feet, HALF_WIDTH_M) + 0.002;
                state.grounded = true;
                let pilot = Explorer {
                    captured: true,
                    boosted,
                    mode: MovementMode::Surface,
                    ..default()
                };
                let mut keys = ButtonInput::default();
                let revision = terrain.revision;
                for (leg, key) in [KeyCode::KeyD, KeyCode::KeyA, KeyCode::KeyD, KeyCode::KeyA]
                    .into_iter()
                    .enumerate()
                {
                    keys.reset_all();
                    keys.press(key);
                    if leg == 2 {
                        keys.press(KeyCode::Space);
                    }
                    let mut airborne = false;
                    let mut apex: f64 = 0.0;
                    for _ in 0..fps * 4 {
                        let old_revision = terrain.revision;
                        let old_feet = terrain.body_relative(state.feet);
                        state.advance(&pilot, &mut terrain, &keys, 1.0 / fps as f64, mu());
                        keys.clear();
                        let floor = terrain.support_height(state.feet, HALF_WIDTH_M);
                        assert!(state.feet.y >= floor - 1e-8);
                        assert!(old_feet.distance(terrain.body_relative(state.feet)) < 0.31);
                        airborne |= old_revision != terrain.revision && !state.grounded;
                        apex = apex.max(state.feet.y - floor);
                    }
                    if leg == 2 {
                        assert!(airborne);
                        assert!((2.0..2.2).contains(&apex));
                    }
                    assert!(state.grounded);
                    assert_eq!(
                        terrain.cube.as_ref().unwrap().pair.keys[0].x,
                        if leg % 2 == 0 { 24576 } else { 24575 }
                    );
                }
                assert_eq!(terrain.revision, revision + 4);
                assert_eq!(terrain.migration.as_ref().unwrap().constructions, 6);
                keys.reset_all();
                let rest = terrain.body_relative(state.feet);
                for _ in 0..fps * 2 {
                    state.advance(&pilot, &mut terrain, &keys, 1.0 / fps as f64, mu());
                }
                assert!(rest.distance(terrain.body_relative(state.feet)) < 1e-7);
                // Quatre bornes, empreinte en l'air puis réception ; aucun plan
                // n'admet ces extrémités (le couloir n'est pas une ouverture mondiale).
                for direction in [DVec3::X, DVec3::NEG_X, DVec3::Z, DVec3::NEG_Z] {
                    let cube = terrain.cube.as_ref().unwrap();
                    state.feet = DVec3::new(
                        direction.x * (cube.max.x - 0.31),
                        2.0,
                        direction.z * (cube.max.z - 0.31),
                    );
                    state.grounded = false;
                    state.velocity = DVec3::ZERO;
                    for _ in 0..600 {
                        surface::step(
                            &terrain,
                            &mut state.feet,
                            &mut state.velocity,
                            &mut state.grounded,
                            direction * 8.0,
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
                    assert!(state.grounded);
                    assert!(state.velocity.dot(direction).abs() < 1e-8);
                }
            }
        }
    }
}
