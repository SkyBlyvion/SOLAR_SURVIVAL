//! Paire jouable opt-in : triangles f64 communs au rendu, un seul repère tournant.
use super::{FaceCoordinates, PatchEdge, PatchKey, PatchPair, terrain::triangle_support};
use bevy::math::{DMat3, DQuat, DVec3};
use std::collections::BTreeMap;

pub const LEVEL: u8 = 15;
pub const SEGMENTS: u32 = 64;
const BIN_M: f64 = 4.0;

pub struct CubeTerrain {
    pub pair: PatchPair,
    /// Soustraction d'origine et rotation f64 effectuées une fois à l'entrée.
    pub local_positions: Vec<DVec3>,
    /// Rectangle intérieur conservateur, avec 1 m de réserve géométrique.
    pub min: DVec3,
    pub max: DVec3,
    triangles: Vec<[u32; 3]>,
    bins: BTreeMap<(i32, i32), Vec<usize>>,
}

impl CubeTerrain {
    /// Face dominante éclairée, deux patches centraux de part et d'autre de u=0.
    /// Cette restriction évite tout changement de face/repère pendant le parcours.
    pub fn new(outward: DVec3, radius_m: f64) -> (DQuat, Self) {
        let face = FaceCoordinates::from_direction(outward)
            .expect("Direction valide")
            .face;
        let middle = 1 << (LEVEL - 1);
        let key = PatchKey {
            face,
            level: LEVEL,
            x: middle - 1,
            y: middle,
        };
        let pair = PatchPair::new(key, PatchEdge::PositiveU, SEGMENTS, radius_m).unwrap();
        let up = face.direction(0.0, 1.0 / (1u32 << LEVEL) as f64).unwrap();
        let (_, u, _) = face.basis();
        let x = (u - up * u.dot(up)).normalize();
        let orientation = DQuat::from_mat3(&DMat3::from_cols(x, up, x.cross(up)));
        let origin = up * radius_m;
        let local_positions: Vec<_> = pair
            .positions_m
            .iter()
            .map(|&p| orientation.inverse() * (p - origin))
            .collect();
        // Patches centraux presque rectangulaires ; les extrema des quatre
        // coins externes donnent un rectangle inscrit, réduit encore de 1 m.
        let left = key;
        let right = pair.keys[1];
        let corners = [
            (left, 0, 0),
            (left, 0, SEGMENTS),
            (right, SEGMENTS, 0),
            (right, SEGMENTS, SEGMENTS),
        ]
        .map(|(key, i, j)| {
            orientation.inverse() * (key.vertex(i, j, SEGMENTS, radius_m).unwrap() - origin)
        });
        let min = DVec3::new(
            corners[0].x.max(corners[1].x) + 1.0,
            0.0,
            corners[1].z.max(corners[3].z) + 1.0,
        );
        let max = DVec3::new(
            corners[2].x.min(corners[3].x) - 1.0,
            0.0,
            corners[0].z.min(corners[2].z) - 1.0,
        );
        let triangles: Vec<_> = pair
            .indices
            .iter()
            .flat_map(|group| group.as_chunks::<3>().0.iter().copied())
            .collect();
        let mut bins = BTreeMap::<_, Vec<_>>::new();
        for (id, indices) in triangles.iter().enumerate() {
            let points = indices.map(|i| local_positions[i as usize]);
            let lo = points.into_iter().reduce(DVec3::min).unwrap();
            let hi = points.into_iter().reduce(DVec3::max).unwrap();
            for x in bin(lo.x)..=bin(hi.x) {
                for z in bin(lo.z)..=bin(hi.z) {
                    bins.entry((x, z)).or_default().push(id);
                }
            }
        }
        (
            orientation,
            Self {
                pair,
                local_positions,
                min,
                max,
                triangles,
                bins,
            },
        )
    }

    /// Aucun support extrapolé au-delà du rectangle résident. L'index spatial
    /// sélectionne les triangles des DEUX patches, puis découpe toute l'empreinte.
    pub fn support_height(&self, feet: DVec3, half_width: f64) -> Option<f64> {
        if !feet.is_finite() || !half_width.is_finite() || half_width < 0.0 {
            return None;
        }
        let x0 = feet.x - half_width;
        let x1 = feet.x + half_width;
        let z0 = feet.z - half_width;
        let z1 = feet.z + half_width;
        if x0 < self.min.x || x1 > self.max.x || z0 < self.min.z || z1 > self.max.z {
            return None;
        }
        let mut height: Option<f64> = None;
        for x in bin(x0)..=bin(x1) {
            for z in bin(z0)..=bin(z1) {
                if let Some(ids) = self.bins.get(&(x, z)) {
                    for &id in ids {
                        let triangle = self.triangles[id].map(|i| self.local_positions[i as usize]);
                        if let Some(value) = triangle_support(triangle, x0, x1, z0, z1) {
                            height = Some(height.map_or(value, |h| h.max(value)));
                        }
                    }
                }
            }
        }
        height
    }
    pub fn index_counts(&self) -> (usize, usize, usize) {
        (
            self.bins.len(),
            self.bins.values().map(Vec::len).sum(),
            self.bins.values().map(Vec::len).max().unwrap_or(0),
        )
    }
}
fn bin(value: f64) -> i32 {
    (value / BIN_M).floor() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::planets::CubeFace;
    const R: f64 = 3_389_500.0;

    #[test]
    fn all_six_central_pairs_have_supported_interior_and_metric_budget() {
        for face in CubeFace::ALL {
            let (orientation, cube) = CubeTerrain::new(face.direction(0.0, 0.0).unwrap(), R);
            assert_eq!(cube.pair.keys[0].face, cube.pair.keys[1].face);
            assert_eq!(cube.local_positions.len(), 8385);
            assert_eq!(cube.triangles.len(), 16384);
            assert!((413.7..413.8).contains(&(cube.max.x - cube.min.x + 2.0)));
            assert!((206.8..207.0).contains(&(cube.max.z - cube.min.z + 2.0)));
            let (bins, references, maximum) = cube.index_counts();
            assert!(bins <= 5700 && references <= 85000 && maximum <= 32);
            for x in 0..=24 {
                for z in 0..=12 {
                    let feet = DVec3::new(
                        cube.min.x + 0.3 + (cube.max.x - cube.min.x - 0.6) * x as f64 / 24.0,
                        0.0,
                        cube.min.z + 0.3 + (cube.max.z - cube.min.z - 0.6) * z as f64 / 12.0,
                    );
                    assert!(
                        cube.support_height(feet, 0.3).is_some(),
                        "{face:?} {feet:?}"
                    );
                }
            }
            for &p in &cube.local_positions {
                let body = orientation * (p + DVec3::Y * R);
                assert!((body.length() - R).abs() < 2e-9);
            }
            assert!(
                cube.support_height(DVec3::new(cube.max.x, 0.0, 0.0), 0.3)
                    .is_none()
            );
            assert!(cube.support_height(DVec3::splat(f64::NAN), 0.3).is_none());
        }
    }

    #[test]
    fn spatial_index_matches_all_triangles_and_entire_footprint_across_seam() {
        let (_, cube) = CubeTerrain::new(DVec3::Y, R);
        for x in [-4.0, -0.3, -0.15, 0.0, 0.15, 0.3, 4.0] {
            for z in [-100.0, -40.0, 0.0, 40.0, 100.0] {
                let feet = DVec3::new(x, 0.0, z);
                let indexed = cube.support_height(feet, 0.3).unwrap();
                let brute = cube
                    .triangles
                    .iter()
                    .filter_map(|ids| {
                        triangle_support(
                            ids.map(|i| cube.local_positions[i as usize]),
                            x - 0.3,
                            x + 0.3,
                            z - 0.3,
                            z + 0.3,
                        )
                    })
                    .reduce(f64::max)
                    .unwrap();
                assert_eq!(indexed, brute);
                for dx in [-0.3, 0.0, 0.3] {
                    for dz in [-0.3, 0.0, 0.3] {
                        assert!(
                            indexed
                                >= cube
                                    .support_height(feet + DVec3::new(dx, 0.0, dz), 0.0)
                                    .unwrap()
                                    - 1e-12
                        );
                    }
                }
                let a = cube.support_height(feet - DVec3::X * 1e-6, 0.3).unwrap();
                let b = cube.support_height(feet + DVec3::X * 1e-6, 0.3).unwrap();
                assert!((a - b).abs() < 1e-9);
            }
        }
        // La hauteur centrale seule sous-estime le support de la boîte.
        let p = DVec3::new(0.15, 0.0, 40.0);
        assert!(cube.support_height(p, 0.3).unwrap() - cube.support_height(p, 0.0).unwrap() > 1e-6);
        for &id in &cube.pair.seam {
            assert!(cube.pair.indices.iter().all(|group| group.contains(&id)));
            assert!(cube.local_positions[id as usize].x.abs() < 1e-9);
        }
    }
}
