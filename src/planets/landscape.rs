//! Couronne locale raccordée exactement au contour des treize patches fins.
//! Ne fournit aucun contact : on charge les patches fins à l'approche du joueur.
use super::{
    PatchEdge, PatchKey,
    globe_terrain::{self, SurfacePatch},
    relief,
};
use bevy::math::{DQuat, DVec3};
use std::collections::{BTreeMap, btree_map::Entry};

type PointKey = [u64; 3];
fn identity(p: DVec3) -> PointKey {
    p.to_array().map(f64::to_bits)
}
// Distances supplémentaires au contour local, en mètres ; dix anneaux au total.
const RINGS_M: [f64; 9] = [
    64.0, 160.0, 320.0, 640.0, 1200.0, 2200.0, 4000.0, 7000.0, 11000.0,
];
pub(super) fn build(center: PatchKey, radius_m: f64, seed: u64) -> SurfacePatch {
    let mut edges = BTreeMap::new();
    for key in globe_terrain::neighborhood(center) {
        for edge in PatchEdge::ALL {
            for i in 0..32 {
                let (a, b) = match edge {
                    PatchEdge::NegativeV => ((i, 0), (i + 1, 0)),
                    PatchEdge::PositiveU => ((32, i), (32, i + 1)),
                    PatchEdge::PositiveV => ((32 - i, 32), (31 - i, 32)),
                    PatchEdge::NegativeU => ((0, 32 - i), (0, 31 - i)),
                };
                let sample = |(u, v)| {
                    relief::displace(key.vertex(u, v, 32, radius_m).unwrap(), radius_m, seed)
                };
                let (a, b) = (sample(a), sample(b));
                let (ka, kb) = (identity(a), identity(b));
                match edges.entry((ka.min(kb), ka.max(kb))) {
                    Entry::Vacant(v) => {
                        v.insert((a, b));
                    }
                    Entry::Occupied(o) => {
                        o.remove();
                    }
                }
            }
        }
    }
    let mut contour: BTreeMap<_, _> = edges
        .into_values()
        .map(|(a, b)| (identity(a), (a, b)))
        .collect();
    let start = *contour.first_key_value().unwrap().0;
    let mut current = start;
    let mut ring = Vec::with_capacity(contour.len());
    loop {
        let (a, b) = contour.remove(&current).expect("Contour simple fermé");
        ring.push(a);
        current = identity(b);
        if current == start {
            break;
        }
    }
    assert!(contour.is_empty());
    let up = center.center_direction().unwrap();
    let q = DQuat::from_rotation_arc(DVec3::Y, up);
    let inverse = q.inverse();
    let count = ring.len();
    let mut positions_m = Vec::with_capacity(count * 10);
    positions_m.extend_from_slice(&ring);
    for distance in RINGS_M {
        for point in &ring {
            let local = inverse * *point;
            let tangent = DVec3::new(local.x, 0.0, local.z);
            let outward = tangent.normalize() * (tangent.length() + distance);
            let direction = (q * (DVec3::Y * radius_m + outward)).normalize();
            positions_m.push(direction * (radius_m + relief::height(direction, radius_m, seed)));
        }
    }
    let mut indices = Vec::with_capacity(count * 9 * 6);
    for r in 0..9 {
        for i in 0..count {
            let a = (r * count + i) as u32;
            let b = (r * count + (i + 1) % count) as u32;
            let c = a + count as u32;
            let d = b + count as u32;
            // Le contour est orienté avec la région intérieure à gauche.
            indices.extend_from_slice(&[a, c, b, b, c, d]);
        }
    }
    SurfacePatch {
        key: center,
        segments: 8,
        positions_m,
        indices,
        relief_seed: Some(seed),
    }
}
