//! Paire jouable opt-in : triangles f64 communs au rendu, un seul repère tournant.
use super::{FaceCoordinates, PatchEdge, PatchKey, PatchPair, terrain::triangle_support};
use bevy::math::{DMat3, DQuat, DVec3};
use std::collections::BTreeMap;

pub const LEVEL: u8 = 15;
pub const SEGMENTS: u32 = 64;
const BIN_M: f64 = 4.0;

pub struct CubeTerrain {
    pub pair: PatchPair,
    /// Soustraction d'origine et rotation f64 une fois par entrée/remplacement.
    pub local_positions: Vec<DVec3>,
    /// Rectangle intérieur conservateur, avec 1 m de réserve géométrique.
    pub min: DVec3,
    pub max: DVec3,
    triangles: Vec<[u32; 3]>,
    bins: BTreeMap<(i32, i32), Vec<usize>>,
}

impl CubeTerrain {
    /// Face dominante éclairée, deux patches centraux de part et d'autre de u=0.
    /// Entrée du parcours ; le changement de carte est un opt-in distinct.
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
        Self::from_key(key, radius_m).expect("Paire centrale valide")
    }

    /// Deux patches +U sur la même face, y compris hors du centre.
    /// Le rectangle est certifié par TOUS les segments de frontière projetés.
    pub fn from_key(key: PatchKey, radius_m: f64) -> Option<(DQuat, Self)> {
        // Ce fournisseur reste borné à la résolution P3.2 et à une largeur
        // nominale de patch de 128–256 m ; ce n'est pas un LOD général.
        let width = 2.0 * radius_m / (1u32 << LEVEL) as f64;
        if key.level != LEVEL || !width.is_finite() || !(128.0..=256.0).contains(&width) {
            return None;
        }
        let pair = PatchPair::new(key, PatchEdge::PositiveU, SEGMENTS, radius_m)?;
        if pair.keys[1].face != key.face {
            return None;
        }
        let count = (1u32 << key.level) as f64;
        let up = key.face.direction(
            2.0 * (key.x + 1) as f64 / count - 1.0,
            2.0 * (key.y as f64 + 0.5) / count - 1.0,
        )?;
        let (_, u, _) = key.face.basis();
        let x = (u - up * u.dot(up)).normalize();
        let orientation = DQuat::from_mat3(&DMat3::from_cols(x, up, x.cross(up)));
        let origin = up * radius_m;
        let local_positions: Vec<_> = pair
            .positions_m
            .iter()
            .map(|&p| orientation.inverse() * (p - origin))
            .collect();
        // Les arêtes de triangles présentes une seule fois forment la frontière.
        // Intersection des demi-plans intérieurs = noyau conservateur du polygone,
        // même quand la projection des bords sphériques n'est pas un rectangle.
        let mut edges = BTreeMap::<(u32, u32), usize>::new();
        for group in &pair.indices {
            for &[a, b, c] in group.as_chunks::<3>().0 {
                for (a, b) in [(a, b), (b, c), (c, a)] {
                    *edges.entry((a.min(b), a.max(b))).or_default() += 1;
                }
            }
        }
        let lo = local_positions.iter().copied().reduce(DVec3::min)?;
        let hi = local_positions.iter().copied().reduce(DVec3::max)?;
        let mut hx = (-lo.x).min(hi.x) - 1.0;
        let mut hz = (-lo.z).min(hi.z) - 1.0;
        let mut scale: f64 = 1.0;
        for ((a, b), occurrences) in edges {
            if occurrences != 1 {
                continue;
            }
            let a = local_positions[a as usize];
            let b = local_positions[b as usize];
            let mut normal = DVec3::new(-(b.z - a.z), 0.0, b.x - a.x).normalize();
            if normal.dot(-a) < 0.0 {
                normal = -normal;
            }
            let distance = normal.dot(-a);
            scale = scale.min((distance - 1.0) / (normal.x.abs() * hx + normal.z.abs() * hz));
        }
        if scale <= 0.0 {
            return None;
        }
        hx *= scale;
        hz *= scale;
        let min = DVec3::new(-hx, 0.0, -hz);
        let max = -min;
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
        Some((
            orientation,
            Self {
                pair,
                local_positions,
                min,
                max,
                triangles,
                bins,
            },
        ))
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
        if x0 < self.min.x - 1e-9
            || x1 > self.max.x + 1e-9
            || z0 < self.min.z - 1e-9
            || z1 > self.max.z + 1e-9
        {
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
    fn noncentral_rectangles_cover_whole_footprints_on_all_faces() {
        let mut invalid = PatchKey {
            face: CubeFace::PositiveX,
            level: 24,
            x: 0,
            y: 0,
        };
        assert!(CubeTerrain::from_key(invalid, R).is_none());
        invalid.level = LEVEL;
        for radius in [0.0, f64::NAN, 1e12] {
            assert!(CubeTerrain::from_key(invalid, radius).is_none());
        }
        for face in CubeFace::ALL {
            for (x, y) in [
                (16384, 16384),
                (2000, 1000),
                (31000, 29000),
                (0, 0),
                (32766, 32767),
            ] {
                let key = PatchKey {
                    face,
                    level: LEVEL,
                    x,
                    y,
                };
                let (orientation, cube) = CubeTerrain::from_key(key, R).unwrap();
                let counts = cube.index_counts();
                println!("{key:?} bounds {:?} index {counts:?}", cube.max);
                assert!(counts.0 <= 5700 && counts.1 <= 85000 && counts.2 <= 64);
                for i in 0..=12 {
                    for j in 0..=12 {
                        let p = DVec3::new(
                            cube.min.x + 0.3 + (cube.max.x - cube.min.x - 0.6) * i as f64 / 12.0,
                            0.0,
                            cube.min.z + 0.3 + (cube.max.z - cube.min.z - 0.6) * j as f64 / 12.0,
                        );
                        let h = cube.support_height(p, 0.3).unwrap();
                        for dx in [-0.3, 0.0, 0.3] {
                            for dz in [-0.3, 0.0, 0.3] {
                                assert!(
                                    h + 1e-10
                                        >= cube
                                            .support_height(p + DVec3::new(dx, 0.0, dz), 0.0)
                                            .unwrap()
                                );
                            }
                        }
                    }
                }
                let brute = cube
                    .triangles
                    .iter()
                    .filter_map(|ids| {
                        triangle_support(
                            ids.map(|i| cube.local_positions[i as usize]),
                            -0.3,
                            0.3,
                            -0.3,
                            0.3,
                        )
                    })
                    .reduce(f64::max)
                    .unwrap();
                assert_eq!(cube.support_height(DVec3::ZERO, 0.3).unwrap(), brute);
                for &p in &cube.local_positions {
                    assert!(((orientation * (p + DVec3::Y * R)).length() - R).abs() < 3e-9);
                }
                assert!(cube.support_height(cube.max, 0.3).is_none());
            }
            assert!(
                CubeTerrain::from_key(
                    PatchKey {
                        face,
                        level: LEVEL,
                        x: 32767,
                        y: 0
                    },
                    R
                )
                .is_none()
            );
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
