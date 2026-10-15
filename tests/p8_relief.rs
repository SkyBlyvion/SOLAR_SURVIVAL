use bevy::math::{DQuat, DVec3};
use solar_survival::planets::{
    CubeFace, PatchEdge, PatchKey,
    globe_terrain::{GlobeTerrain, SurfacePatch},
    relief,
};
use std::collections::BTreeMap;
const R: f64 = 3_389_500.0;
type Point = [u64; 3];
fn point(p: DVec3) -> Point {
    p.to_array().map(f64::to_bits)
}
fn boundary<'a>(
    patches: impl IntoIterator<Item = &'a SurfacePatch>,
) -> BTreeMap<(Point, Point), usize> {
    let mut edges = BTreeMap::new();
    for patch in patches {
        for ids in patch.indices.as_chunks::<3>().0 {
            let [a, b, c] = ids.map(|i| patch.positions_m[i as usize]);
            assert!((b - a).cross(c - a).dot(a) > 0.0, "Triangle inversé");
            for (a, b) in [(a, b), (b, c), (c, a)] {
                let (a, b) = (point(a), point(b));
                *edges.entry((a.min(b), a.max(b))).or_insert(0) += 1;
            }
        }
    }
    edges
}
#[test]
fn relief_seams_match_bit_for_bit_between_faces_and_mixed_lods() {
    for face in CubeFace::ALL {
        for (x, y) in [
            (0, 0),
            (32767, 0),
            (32767, 32767),
            (0, 32767),
            (16384, 16384),
        ] {
            let key = PatchKey {
                face,
                level: 15,
                x,
                y,
            };
            for edge in PatchEdge::ALL {
                let neighbor = key.neighbor(edge).unwrap();
                let a = SurfacePatch::with_relief(key, R, 32, 42);
                for detail in [32, 16, 8] {
                    let b = SurfacePatch::with_relief(neighbor.key, R, detail, 42);
                    // Tous les segments de la couture doivent être des arêtes dans les deux meshes.
                    let counts = boundary([&a, &b]);
                    for i in 0..32 {
                        let (u, v) = edge.sample(i, 32).unwrap();
                        let (uu, vv) = edge.sample(i + 1, 32).unwrap();
                        let p = point(relief::displace(key.vertex(u, v, 32, R).unwrap(), R, 42));
                        let q = point(relief::displace(key.vertex(uu, vv, 32, R).unwrap(), R, 42));
                        assert_eq!(counts[&(p.min(q), p.max(q))], 2);
                    }
                }
            }
        }
    }
}
#[test]
fn landscape_is_a_single_joined_mesh_without_internal_cracks_and_has_bounded_size() {
    for face in CubeFace::ALL {
        for (u, v) in [
            (-1.0, -1.0),
            (1.0, 1.0),
            (1.0, 0.23),
            (0.5, -0.5),
            (0.0, 0.0),
        ] {
            let up = face.direction(u, v).unwrap();
            let q = DQuat::from_rotation_arc(DVec3::Y, up);
            let g = GlobeTerrain::with_relief_lod(up, q, R, 42, true);
            let landscape = g.landscape.as_ref().unwrap();
            assert!(landscape.positions_m.len() <= 6400);
            let n = landscape.positions_m.len() / 10;
            let edges = boundary(
                g.patches
                    .values()
                    .chain(std::iter::once(landscape))
                    .map(|p| p.as_ref()),
            );
            assert!(edges.values().all(|&count| count <= 2));
            let outer: Vec<_> = edges.iter().filter(|(_, n)| **n == 1).collect();
            assert_eq!(outer.len(), n);
            for ((a, b), _) in outer {
                for vertex in [a, b] {
                    let p = DVec3::from_array(vertex.map(f64::from_bits));
                    assert!(p.distance(up * R) > 10500.0);
                }
            }
            let next = g.rebased((up * R + q * DVec3::X * 64.1).normalize(), q, R);
            if next.center == g.center {
                assert!(std::sync::Arc::ptr_eq(
                    landscape,
                    next.landscape.as_ref().unwrap()
                ));
            }
        }
    }
}
#[test]
fn fine_contacts_and_building_extrema_survive_relief_lod_and_rebasing() {
    for face in CubeFace::ALL {
        for (u, v) in [(0.0, 0.0), (1.0, 1.0), (-1.0, 0.23), (0.5, -0.5)] {
            let up = face.direction(u, v).unwrap();
            let q = DQuat::from_rotation_arc(DVec3::Y, up);
            let a = GlobeTerrain::with_relief_lod(up, q, R, 42, true);
            let b = GlobeTerrain::with_relief_lod(up, q, R, 42, false);
            for x in -4..=4 {
                for z in -4..=4 {
                    let p = DVec3::new(x as f64 * 11.0, 0.0, z as f64 * 11.0);
                    if p.x.hypot(p.z) > 65.0 {
                        continue;
                    }
                    for half in [0.0, 0.3, 4.5] {
                        let aa = a.height_range(p, half).unwrap();
                        let bb = b.height_range(p, half).unwrap();
                        assert!((aa.0 - bb.0).abs() < 1e-9 && (aa.1 - bb.1).abs() < 1e-9);
                        assert!(
                            aa.0 >= -relief::MAX_HEIGHT_M - 0.1 && aa.1 <= relief::MAX_HEIGHT_M
                        );
                    }
                }
            }
        }
    }
}
