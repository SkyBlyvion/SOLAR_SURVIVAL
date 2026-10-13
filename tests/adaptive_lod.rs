use bevy::math::{DQuat, DVec3};
use solar_survival::planets::{
    CubeFace, PatchEdge, PatchKey,
    globe_terrain::{DETAILS, GlobeTerrain, SurfacePatch},
};
use std::collections::BTreeMap;
const R: f64 = 3_389_500.0;
fn boundary(patch: &SurfacePatch) -> Vec<(DVec3, DVec3)> {
    let mut edges = BTreeMap::new();
    for ids in patch.indices.as_chunks::<3>().0 {
        let [a, b, c] = ids.map(|i| patch.positions_m[i as usize]);
        assert!((b - a).cross(c - a).dot(a) > 0.0);
        for (a, b) in [(ids[0], ids[1]), (ids[1], ids[2]), (ids[2], ids[0])] {
            *edges.entry((a.min(b), a.max(b))).or_insert(0) += 1;
        }
    }
    assert!(edges.values().all(|&n| n == 1 || n == 2));
    assert_eq!(
        patch.positions_m.len() as i32 - edges.len() as i32 + (patch.indices.len() / 3) as i32,
        1
    );
    let boundary: Vec<_> = edges
        .into_iter()
        .filter(|(_, n)| *n == 1)
        .map(|((a, b), _)| (patch.positions_m[a as usize], patch.positions_m[b as usize]))
        .collect();
    assert_eq!(boundary.len(), 128);
    boundary
}
#[test]
fn every_mixed_resolution_edge_is_the_same_complete_polyline_on_faces_and_corners() {
    for face in CubeFace::ALL {
        for (x, y) in [
            (16384, 16384),
            (0, 0),
            (32767, 0),
            (0, 32767),
            (32767, 32767),
        ] {
            let key = PatchKey {
                face,
                level: 15,
                x,
                y,
            };
            for edge in PatchEdge::ALL {
                let neighbor = key.neighbor(edge).unwrap();
                for source in DETAILS {
                    let a = SurfacePatch::with_segments(key, R, source);
                    let ab = boundary(&a);
                    for target in DETAILS {
                        let b = SurfacePatch::with_segments(neighbor.key, R, target);
                        let bb = boundary(&b);
                        for k in 0..32 {
                            let (i, j) = edge.sample(k, 32).unwrap();
                            let (ii, jj) = edge.sample(k + 1, 32).unwrap();
                            let p = key.vertex(i, j, 32, R).unwrap();
                            let q = key.vertex(ii, jj, 32, R).unwrap();
                            let contains = |edges: &Vec<(DVec3, DVec3)>| {
                                edges
                                    .iter()
                                    .any(|&(a, b)| (a == p && b == q) || (a == q && b == p))
                            };
                            assert!(contains(&ab) && contains(&bb));
                        }
                    }
                }
            }
        }
    }
}
#[test]
fn selection_reduces_residency_and_keeps_all_valid_contacts_and_construction_fine() {
    let mut saw = [false; 3];
    let mut maximum = (0, 0, 0);
    for face in CubeFace::ALL {
        for u in [-1.0, -0.5, 0.0, 0.5, 1.0] {
            for v in [-1.0, -0.5, 0.0, 0.5, 1.0] {
                let up = face.direction(u, v).unwrap();
                let q = DQuat::from_rotation_arc(DVec3::Y, up) * DQuat::from_rotation_y(0.73);
                let fine = GlobeTerrain::with_lod(up, q, R, false);
                let adaptive = GlobeTerrain::with_lod(up, q, R, true);
                let (details, vertices, triangles) = adaptive.detail_counts();
                assert!(vertices <= 14_157 && triangles <= 26_624);
                let indexed = adaptive.index_counts();
                maximum = (
                    maximum.0.max(indexed.0),
                    maximum.1.max(indexed.1),
                    maximum.2.max(indexed.2),
                );
                assert!(indexed.0 <= 400 && indexed.1 <= 24_000 && indexed.2 <= 6_000);
                for i in 0..3 {
                    saw[i] |= details[i] > 0;
                }
                for ix in -8..=8 {
                    for iz in -8..=8 {
                        let feet = DVec3::new(ix as f64 * 8.0, 0.0, iz as f64 * 8.0);
                        if feet.x.hypot(feet.z) > 66.0 {
                            continue;
                        }
                        for half in [0.0, 0.3, 1.0, 4.5, 5.0] {
                            let a = adaptive.height_range(feet, half).unwrap();
                            let b = fine.height_range(feet, half).unwrap();
                            assert!(
                                (a.0 - b.0).abs() < 1e-9 && (a.1 - b.1).abs() < 1e-9,
                                "{face:?} {feet:?} half={half}"
                            );
                        }
                    }
                }
            }
        }
    }
    println!("LOD index maxima {maximum:?}");
    assert!(saw.into_iter().all(|v| v));
}
#[test]
fn coarse_sphere_transition_error_is_below_a_tenth_of_a_millimeter() {
    for face in CubeFace::ALL {
        for (x, y) in [(16384, 16384), (0, 0), (32767, 32767)] {
            let key = PatchKey {
                face,
                level: 15,
                x,
                y,
            };
            for detail in DETAILS {
                let patch = SurfacePatch::with_segments(key, R, detail);
                for triangle in patch.indices.as_chunks::<3>().0 {
                    let [a, b, c] = triangle.map(|i| patch.positions_m[i as usize]);
                    // Le minimum du rayon sur le triangle est >= distance
                    // du plan au centre (borne conservative même hors triangle).
                    let n = (b - a).cross(c - a).normalize();
                    assert!(R - n.dot(a) < 0.0001);
                }
            }
        }
    }
}
#[test]
fn moving_stream_keeps_bounded_geometry_and_changes_actual_resolution() {
    let q = DQuat::IDENTITY;
    let mut g = GlobeTerrain::with_lod(DVec3::Y, q, R, true);
    let mut saw_change = false;
    for x in (64..=1280).step_by(64) {
        let up = (DVec3::Y * R + DVec3::X * x as f64).normalize();
        let q = DQuat::from_rotation_arc(DVec3::Y, up);
        let next = g.rebased(up, q, R);
        let (d, v, t) = next.detail_counts();
        assert!(d.iter().sum::<usize>() <= 13 && v <= 14_157 && t <= 26_624);
        assert!(g.detail_counts().1 + v <= 28_314);
        assert!(g.detail_counts().2 + t <= 53_248);
        for (key, p) in &g.patches {
            if let Some(new) = next.patches.get(key) {
                if p.segments == new.segments {
                    assert!(std::sync::Arc::ptr_eq(p, new));
                } else {
                    saw_change = true;
                }
            }
        }
        g = next;
    }
    assert!(saw_change && g.lod_changes > 0);
    println!(
        "LOD streaming constructions {}, detail changes {}",
        g.constructions, g.lod_changes
    );
}
