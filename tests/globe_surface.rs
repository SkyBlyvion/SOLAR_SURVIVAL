use bevy::{
    math::{DQuat, DVec3},
    prelude::*,
};
use solar_survival::{
    physics::surface::{self, HALF_WIDTH_M},
    planets::{
        CubeFace, PatchEdge,
        globe_terrain::{self, GlobeTerrain, MAX_PATCHES, REBASE_M},
        terrain::MarsTerrain,
    },
    player::{Explorer, MovementMode, surface::SurfaceState},
    universe::{catalog::SolarSystem, celestial_body::BodyId},
};
const R: f64 = 3_389_500.0;
fn terrain(up: DVec3) -> MarsTerrain {
    let mut terrain = MarsTerrain {
        building_solids: Vec::new(),
        radius_m: R,
        seed: 42,
        orientation: DQuat::IDENTITY,
        revision: 0,
        cube_enabled: false,
        migration_enabled: false,
        offcenter_enabled: false,
        migration: None,
        site_obstacles: None,
        cube: None,
        globe_enabled: true,
        globe: None,
    };
    terrain.set_site(up);
    terrain
}
fn mu() -> f64 {
    SolarSystem::default()
        .body(BodyId::MARS)
        .gravitational_parameter
}

#[test]
fn six_faces_edges_and_corners_cover_operational_disk_with_full_footprints() {
    let mut maximum = (0, 0, 0);
    for face in CubeFace::ALL {
        for u in [-1.0, -0.5, 0.0, 0.5, 1.0] {
            for v in [-1.0, -0.5, 0.0, 0.5, 1.0] {
                let up = face.direction(u, v).unwrap();
                let q = DQuat::from_rotation_arc(DVec3::Y, up) * DQuat::from_rotation_y(0.73);
                let globe = GlobeTerrain::new(up, q, R);
                assert!(globe.support_height(DVec3::ZERO, 1000.0).is_none());
                assert!(globe.patches.len() <= MAX_PATCHES);
                let counts = globe.index_counts();
                maximum = (
                    maximum.0.max(counts.0),
                    maximum.1.max(counts.1),
                    maximum.2.max(counts.2),
                );
                for ix in -8..=8 {
                    for iz in -8..=8 {
                        let feet = DVec3::new(ix as f64 * 8.0, 0.0, iz as f64 * 8.0);
                        if feet.x.hypot(feet.z) > REBASE_M + 1.0 {
                            continue;
                        }
                        let h = globe
                            .support_height(feet, HALF_WIDTH_M)
                            .expect("Toute empreinte couverte");
                        assert!(h.is_finite() && (-0.01..0.001).contains(&h));
                        // Les quatre coins de l'empreinte ont aussi un support.
                        for x in [-HALF_WIDTH_M, HALF_WIDTH_M] {
                            for z in [-HALF_WIDTH_M, HALF_WIDTH_M] {
                                assert!(
                                    globe
                                        .support_height(feet + DVec3::new(x, 0.0, z), 0.0)
                                        .is_some()
                                );
                            }
                        }
                    }
                }
                for patch in globe.patches.values() {
                    for &ids in patch.indices.as_chunks::<3>().0 {
                        let [a, b, c] = ids.map(|i| patch.positions_m[i as usize]);
                        assert!((b - a).cross(c - a).dot(a) > 0.0);
                    }
                }
            }
        }
    }
    println!("global index maximum {maximum:?}");
}

#[test]
fn all_directed_face_edges_walk_run_jump_return_without_bounds_or_pose_jump() {
    let mut crossings = 0;
    let mut rebases = 0;
    for face in CubeFace::ALL {
        for edge in PatchEdge::ALL {
            let (u, v) = match edge {
                PatchEdge::NegativeU => (-1.0, 0.23),
                PatchEdge::PositiveU => (1.0, 0.23),
                PatchEdge::NegativeV => (0.23, -1.0),
                PatchEdge::PositiveV => (0.23, 1.0),
            };
            let seam = face.direction(u, v).unwrap();
            let inside = face.direction(u * 0.99999, v * 0.99999).unwrap();
            let tangent = (seam - inside - seam * (seam - inside).dot(seam)).normalize();
            for fps in [30, 60, 144] {
                for boosted in [false, true] {
                    let mut terrain = terrain((seam - tangent * 12.0 / R).normalize());
                    // Isoler le raccord ; objets exacts fixes mais loin du trajet.
                    let solid = surface::SolidBox {
                        min: DVec3::splat(1000.0),
                        max: DVec3::splat(1001.0),
                    };
                    terrain.site_obstacles = Some((terrain.orientation, [solid; 3]));
                    let mut state = SurfaceState::default();
                    state.feet.y = terrain.support_height(state.feet, HALF_WIDTH_M) + 0.002;
                    state.grounded = true;
                    let local = terrain.orientation.inverse() * tangent;
                    state.yaw = (-local.x).atan2(-local.z);
                    let pilot = Explorer {
                        captured: true,
                        boosted,
                        mode: MovementMode::Surface,
                        ..default()
                    };
                    let mut keys = ButtonInput::default();
                    keys.press(KeyCode::KeyW);
                    let mut jumped = false;
                    let mut airborne_crossing = false;
                    let mut previous_face = face;
                    // 100 m aller puis 100 m retour : plusieurs origines/patches.
                    let frames =
                        (100.0 / if boosted { 8.0 } else { 4.5 } * fps as f64).ceil() as usize;
                    for leg in 0..2 {
                        if leg == 1 {
                            keys.reset_all();
                            keys.press(KeyCode::KeyS);
                        }
                        for _ in 0..frames {
                            let before = terrain.body_relative(state.feet);
                            let face_before = globe_terrain::key_at(before).face;
                            if leg == 0 && !jumped && (before - seam * R).dot(tangent) > -1.0 {
                                keys.press(KeyCode::Space);
                                jumped = true;
                            }
                            state.advance(&pilot, &mut terrain, &keys, 1.0 / fps as f64, mu());
                            keys.clear();
                            let after = terrain.body_relative(state.feet);
                            assert!(before.distance(after) < 0.31);
                            assert!(
                                state.feet.y
                                    >= terrain.support_height(state.feet, HALF_WIDTH_M) - 1e-8
                            );
                            let current = globe_terrain::key_at(after).face;
                            if current != previous_face {
                                crossings += 1;
                                previous_face = current;
                            }
                            airborne_crossing |= current != face_before && !state.grounded;
                        }
                        assert_eq!(
                            globe_terrain::key_at(terrain.body_relative(state.feet)).face == face,
                            leg == 1
                        );
                        assert!(state.grounded);
                    }
                    assert!(jumped && airborne_crossing);
                    let globe = terrain.globe.as_ref().unwrap();
                    assert!(globe.rebases >= 2 && globe.patches.len() <= MAX_PATCHES);
                    rebases += globe.rebases;
                    let construction_count = globe.constructions;
                    keys.reset_all();
                    let before_rest = terrain.body_relative(state.feet);
                    // Un remplacement conserve les pieds ; le support sous la
                    // boîte reboîtée peut changer de quelques micromètres.
                    state.advance(&pilot, &mut terrain, &keys, 2.0 / fps as f64, mu());
                    let rest = terrain.body_relative(state.feet);
                    assert!(before_rest.distance(rest) < 0.00005);
                    for _ in 0..fps {
                        state.advance(&pilot, &mut terrain, &keys, 1.0 / fps as f64, mu());
                    }
                    assert!(
                        rest.distance(terrain.body_relative(state.feet)) < 1e-7,
                        "{face:?} {edge:?} fps={fps} boosted={boosted} rest drift={} velocity={:?}",
                        rest.distance(terrain.body_relative(state.feet)),
                        state.velocity
                    );
                    assert_eq!(
                        terrain.globe.as_ref().unwrap().constructions,
                        construction_count
                    );
                }
            }
        }
    }
    println!("global directed crossings {crossings}, rebases {rebases}");
    assert_eq!(crossings, 288);
}

#[test]
fn rebase_preserves_f64_pose_velocity_view_gravity_and_reuses_geometry() {
    for up in [
        DVec3::X,
        DVec3::NEG_Y,
        DVec3::new(1.0, 1.0, 1.0).normalize(),
    ] {
        let mut terrain = terrain(up);
        let mut state = SurfaceState {
            feet: DVec3::new(64.1, 2.0, 3.0),
            velocity: DVec3::new(8.0, 2.4, -1.0),
            yaw: 0.7,
            pitch: -0.4,
            accumulator: 0.004,
            jump_pending: true,
            ..default()
        };
        let body = terrain.body_relative(state.feet);
        let eye = terrain.body_relative(state.eye(&terrain));
        let view = terrain.orientation * state.local_rotation(&terrain);
        let velocity = terrain.orientation * state.velocity;
        let saved: Vec<_> = terrain
            .globe
            .as_ref()
            .unwrap()
            .patches
            .iter()
            .map(|(key, p)| (*key, p.clone()))
            .collect();
        assert!(state.migrate(&mut terrain));
        assert!(body.distance(terrain.body_relative(state.feet)) < 4e-9);
        assert!(eye.distance(terrain.body_relative(state.eye(&terrain))) < 4e-9);
        assert!(velocity.distance(terrain.orientation * state.velocity) < 1e-13);
        for axis in [DVec3::X, DVec3::Y, DVec3::Z] {
            assert!(
                (view * axis).distance(terrain.orientation * state.local_rotation(&terrain) * axis)
                    < 1e-13
            );
        }
        assert_eq!(state.accumulator, 0.004);
        assert!(state.jump_pending);
        assert!(!state.migrate(&mut terrain));
        let next = terrain.globe.as_ref().unwrap();
        for (key, patch) in saved {
            if let Some(new) = next.patches.get(&key) {
                if patch.segments == new.segments {
                    assert!(std::sync::Arc::ptr_eq(&patch, new));
                } else {
                    assert!(!std::sync::Arc::ptr_eq(&patch, new));
                }
            }
        }
    }
}

#[test]
fn every_cube_corner_is_walkable_through_face_changes_and_dynamic_streaming() {
    for sx in [-1.0, 1.0] {
        for sy in [-1.0, 1.0] {
            for sz in [-1.0, 1.0] {
                let corner = DVec3::new(sx, sy, sz).normalize();
                for tangent in [
                    DVec3::new(sx, -sy, 0.0),
                    DVec3::new(0.0, sy, -sz),
                    DVec3::new(-sx, 0.0, sz),
                ] {
                    let tangent = tangent.normalize();
                    let mut terrain = terrain((corner - tangent * 12.0 / R).normalize());
                    let solid = surface::SolidBox {
                        min: DVec3::splat(1000.0),
                        max: DVec3::splat(1001.0),
                    };
                    terrain.site_obstacles = Some((terrain.orientation, [solid; 3]));
                    let start = globe_terrain::key_at(terrain.body_relative(DVec3::ZERO)).face;
                    let local = terrain.orientation.inverse() * tangent;
                    let mut state = SurfaceState {
                        yaw: (-local.x).atan2(-local.z),
                        grounded: true,
                        ..default()
                    };
                    state.feet.y = terrain.support_height(state.feet, HALF_WIDTH_M) + 0.002;
                    let mut keys = ButtonInput::default();
                    keys.press(KeyCode::KeyW);
                    let pilot = Explorer {
                        captured: true,
                        boosted: true,
                        mode: MovementMode::Surface,
                        ..default()
                    };
                    for _ in 0..720 {
                        state.advance(&pilot, &mut terrain, &keys, 1.0 / 60.0, mu());
                        keys.clear();
                        assert!(
                            state.feet.y >= terrain.support_height(state.feet, HALF_WIDTH_M) - 1e-8
                        );
                    }
                    assert_ne!(
                        start,
                        globe_terrain::key_at(terrain.body_relative(state.feet)).face
                    );
                    assert!(terrain.globe.as_ref().unwrap().rebases >= 1);
                }
            }
        }
    }
}
