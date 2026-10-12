use bevy::math::DVec3;
use solar_survival::planets::{CubeFace, FaceCoordinates, PatchEdge, PatchKey, PatchPair};
use std::collections::{HashMap, HashSet};

const RADIUS: f64 = 3_389_500.0;

fn key(face: CubeFace, level: u8, x: u32, y: u32) -> PatchKey {
    PatchKey { face, level, x, y }
}

#[test]
fn six_face_conventions_and_roundtrips_include_edges() {
    let normals = [
        DVec3::X,
        DVec3::NEG_X,
        DVec3::Y,
        DVec3::NEG_Y,
        DVec3::Z,
        DVec3::NEG_Z,
    ];
    for (face, normal) in CubeFace::ALL.into_iter().zip(normals) {
        assert_eq!(face.direction(0.0, 0.0).unwrap(), normal);
        for i in 0..=32 {
            for j in 0..=32 {
                let u = i as f64 / 16.0 - 1.0;
                let v = j as f64 / 16.0 - 1.0;
                let d = face.direction(u, v).unwrap();
                let inverse = FaceCoordinates::from_direction(d * 7.0).unwrap();
                assert!(inverse.direction().unwrap().distance(d) < 6e-16);
                if u.abs() < 1.0 && v.abs() < 1.0 {
                    assert_eq!(inverse.face, face);
                    assert!((inverse.u - u).abs() < 4e-16);
                    assert!((inverse.v - v).abs() < 4e-16);
                }
            }
        }
    }
}

#[test]
fn inverse_handles_extreme_scales_and_has_deterministic_ties() {
    for scale in [1e-300, 1.0, 1e300, f64::MAX] {
        let c = FaceCoordinates::from_direction(DVec3::new(scale, -scale, scale)).unwrap();
        assert_eq!(c.face, CubeFace::PositiveX);
        assert_eq!((c.u, c.v), (-1.0, -1.0));
    }
    let c = FaceCoordinates::from_direction(DVec3::new(0.0, -1.0, 1.0)).unwrap();
    assert_eq!(c.face, CubeFace::NegativeY);
    assert_eq!(
        FaceCoordinates::from_direction(DVec3::splat(f64::from_bits(1)))
            .unwrap()
            .u,
        -1.0
    );
}

#[test]
fn invalid_inputs_are_rejected_before_allocation() {
    for d in [
        DVec3::ZERO,
        DVec3::splat(f64::NAN),
        DVec3::splat(f64::INFINITY),
    ] {
        assert!(FaceCoordinates::from_direction(d).is_none());
    }
    let k = key(CubeFace::PositiveX, 2, 1, 1);
    for value in [f64::NAN, f64::INFINITY, -1.01, 1.01] {
        assert!(k.face.direction(value, 0.0).is_none());
        assert!(k.face.direction(0.0, value).is_none());
    }
    assert!(k.direction(-0.01, 0.5).is_none());
    assert!(k.direction(0.5, 1.01).is_none());
    for invalid in [
        key(k.face, 25, 0, 0),
        key(k.face, 255, 0, 0),
        key(k.face, 2, 4, 0),
        key(k.face, 2, 0, 4),
    ] {
        assert!(invalid.center_direction().is_none());
        assert!(invalid.neighbor(PatchEdge::PositiveU).is_none());
        assert!(PatchPair::new(invalid, PatchEdge::PositiveU, 4, RADIUS).is_none());
    }
    for segments in [0, 65, u32::MAX] {
        assert!(PatchPair::new(k, PatchEdge::PositiveU, segments, RADIUS).is_none());
        assert!(k.vertex(0, 0, segments, RADIUS).is_none());
    }
    for radius in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(PatchPair::new(k, PatchEdge::PositiveU, 4, radius).is_none());
        assert!(k.vertex(0, 0, 4, radius).is_none());
    }
    assert!(k.vertex(5, 0, 4, RADIUS).is_none());
    assert!(PatchEdge::PositiveV.sample(5, 4).is_none());
}

fn check_neighbor(k: PatchKey, edge: PatchEdge, segments: u32) {
    let n = k.neighbor(edge).unwrap();
    assert_eq!(n.key.level, k.level);
    let back = n.key.neighbor(n.edge).unwrap();
    assert_eq!(back.key, k);
    assert_eq!(back.edge, edge);
    assert_eq!(back.reversed, n.reversed);
    for i in 0..=segments {
        let (x, y) = edge.sample(i, segments).unwrap();
        let t = if n.reversed { segments - i } else { i };
        let (nx, ny) = n.edge.sample(t, segments).unwrap();
        assert_eq!(
            k.vertex(x, y, segments, RADIUS),
            n.key.vertex(nx, ny, segments, RADIUS)
        );
    }
}

#[test]
fn all_neighbors_and_oriented_edges_match_exactly_at_small_levels() {
    for level in 0..=3 {
        let count = 1 << level;
        for face in CubeFace::ALL {
            for y in 0..count {
                for x in 0..count {
                    for edge in PatchEdge::ALL {
                        check_neighbor(key(face, level, x, y), edge, 7);
                    }
                }
            }
        }
    }
}

#[test]
fn maximum_level_edges_and_patch_roundtrips_keep_precision() {
    let count = 1 << PatchKey::MAX_LEVEL;
    for face in CubeFace::ALL {
        for x in [0, count / 2, count - 1] {
            for y in [0, count / 2, count - 1] {
                let k = key(face, PatchKey::MAX_LEVEL, x, y);
                for edge in PatchEdge::ALL {
                    check_neighbor(k, edge, 64);
                }
                let c = FaceCoordinates::from_direction(k.center_direction().unwrap()).unwrap();
                assert_eq!(c.face, face);
                assert_eq!(((c.u + 1.0) * 0.5 * count as f64).floor() as u32, x);
                assert_eq!(((c.v + 1.0) * 0.5 * count as f64).floor() as u32, y);
            }
        }
    }
}

#[test]
fn eight_cube_corners_have_three_identical_face_representations() {
    let mut corners: HashMap<[i32; 3], Vec<DVec3>> = HashMap::new();
    for face in CubeFace::ALL {
        let k = key(face, 0, 0, 0);
        for i in [0, 1] {
            for j in [0, 1] {
                let p = k.vertex(i, j, 1, RADIUS).unwrap();
                let signs = [
                    p.x.signum() as i32,
                    p.y.signum() as i32,
                    p.z.signum() as i32,
                ];
                corners.entry(signs).or_default().push(p);
            }
        }
    }
    assert_eq!(corners.len(), 8);
    for points in corners.values() {
        assert_eq!(points.len(), 3);
        assert!(points.iter().all(|p| *p == points[0]));
        assert!((points[0].length() - RADIUS).abs() < 1e-9);
    }
}

fn check_topology(pair: &PatchPair, segments: u32) {
    assert_eq!(
        pair.positions_m.len(),
        (2 * (segments + 1).pow(2) - segments - 1) as usize
    );
    let mut edges: HashMap<(u32, u32), (u32, i32)> = HashMap::new();
    let mut groups = Vec::new();
    for indices in &pair.indices {
        assert_eq!(indices.len(), (segments * segments * 6) as usize);
        groups.push(indices.iter().copied().collect::<HashSet<_>>());
        for &[a, b, c] in indices.as_chunks::<3>().0 {
            let pa = pair.positions_m[a as usize];
            let pb = pair.positions_m[b as usize];
            let pc = pair.positions_m[c as usize];
            assert!((pb - pa).cross(pc - pa).dot(pa) > 0.0);
            for (u, v) in [(a, b), (b, c), (c, a)] {
                let e = edges.entry((u.min(v), u.max(v))).or_default();
                e.0 += 1;
                e.1 += if u < v { 1 } else { -1 };
            }
        }
    }
    let shared: HashSet<_> = groups[0].intersection(&groups[1]).copied().collect();
    assert_eq!(shared, pair.seam.iter().copied().collect());
    assert_eq!(shared.len(), (segments + 1) as usize);
    assert_eq!(
        edges.values().filter(|e| e.0 == 1).count(),
        (6 * segments) as usize
    );
    assert!(
        edges
            .values()
            .all(|&(count, orientation)| count == 1 || (count == 2 && orientation == 0))
    );
    for edge in pair.seam.windows(2) {
        assert_eq!(edges[&(edge[0].min(edge[1]), edge[0].max(edge[1]))], (2, 0));
    }
    for p in &pair.positions_m {
        assert!((p.length() - RADIUS).abs() < 1e-9);
    }
}

#[test]
fn pairs_share_indices_have_outward_triangles_and_no_open_seam() {
    for face in CubeFace::ALL {
        for level in [0, 2, 15, 24] {
            let count = 1 << level;
            for (x, y) in [(0, 0), (count - 1, count - 1), (count / 2, count / 2)] {
                for edge in PatchEdge::ALL {
                    let k = key(face, level, x, y);
                    for segments in [1, 3, 8] {
                        check_topology(
                            &PatchPair::new(k, edge, segments, RADIUS).unwrap(),
                            segments,
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn pair_allocation_and_generation_are_bounded_and_reproducible() {
    let k = key(CubeFace::NegativeZ, 15, 0, 0);
    let a = PatchPair::new(k, PatchEdge::NegativeV, 64, RADIUS).unwrap();
    let b = PatchPair::new(k, PatchEdge::NegativeV, 64, RADIUS).unwrap();
    assert_eq!(a.positions_m.len(), 8_385);
    assert_eq!(a.indices.iter().map(|i| i.len() / 3).sum::<usize>(), 16_384);
    assert_eq!(a.positions_m, b.positions_m);
    assert_eq!(a.indices, b.indices);
    assert_eq!(a.seam, b.seam);
    check_topology(&a, 64);
}

#[test]
fn seeds_preserve_historical_vectors_and_distinguish_valid_keys() {
    // Valeurs calculées indépendamment du code Rust avec le mélange historique.
    assert_eq!(
        key(CubeFace::PositiveX, 0, 0, 0).deterministic_seed(0),
        0xe220a8397b1dcdaf
    );
    assert_eq!(
        key(CubeFace::PositiveY, 8, 12, 140).deterministic_seed(42),
        0x31838971074a4528
    );
    assert_eq!(
        key(CubeFace::NegativeZ, 24, 16_777_215, 16_777_215).deterministic_seed(42),
        0xd42b59aa1449c4b8
    );
    let mut seeds = HashSet::new();
    for face in CubeFace::ALL {
        for level in 0..=4 {
            for x in 0..(1 << level) {
                for y in 0..(1 << level) {
                    let k = key(face, level, x, y);
                    let seed = k.deterministic_seed(42);
                    assert!(seeds.insert(seed));
                    assert_ne!(seed, k.deterministic_seed(43));
                    assert_eq!(seed, k.deterministic_seed(42));
                }
            }
        }
    }
}
