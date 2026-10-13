//! Mars entière, chargement local uniquement. Sommets communs par identité cubique.
use super::{FaceCoordinates, PatchEdge, PatchKey, terrain::triangle_support_area};
use bevy::math::{DQuat, DVec3};
use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

pub const LEVEL: u8 = 15;
pub const SEGMENTS: u32 = 32;
pub const MAX_PATCHES: usize = 13;
pub const REBASE_M: f64 = 64.0;
/// 66 m de requête + demi-largeur maximale 5 m, réserve d'arrondi 1 m.
pub const INDEX_RADIUS_M: f64 = 72.0;
pub const DETAILS: [u32; 3] = [32, 16, 8];

const BIN_M: f64 = 8.0;

pub fn key_at(direction: DVec3) -> PatchKey {
    let face = FaceCoordinates::from_direction(direction).expect("Direction de surface finie");
    let count = 1u32 << LEVEL;
    let coordinate = |v: f64| (((v + 1.0) * 0.5 * count as f64).floor() as u32).min(count - 1);
    PatchKey {
        face: face.face,
        level: LEVEL,
        x: coordinate(face.u),
        y: coordinate(face.v),
    }
}

/// Distance de graphe ≤2 : couvre les diagonales, y compris aux huit coins.
pub fn neighborhood(center: PatchKey) -> Vec<PatchKey> {
    let mut keys = vec![center];
    let mut start = 0;
    for _ in 0..2 {
        let end = keys.len();
        for i in start..end {
            for edge in PatchEdge::ALL {
                let key = keys[i].neighbor(edge).expect("Clé valide").key;
                if !keys.contains(&key) {
                    keys.push(key);
                }
            }
        }
        start = end;
    }
    assert!(keys.len() <= MAX_PATCHES);
    keys
}

pub struct SurfacePatch {
    pub key: PatchKey,
    /// Résolution intérieure. Tous les bords restent à SEGMENTS.
    pub segments: u32,
    pub positions_m: Vec<DVec3>,
    pub indices: Vec<u32>,
}
impl SurfacePatch {
    pub fn with_segments(key: PatchKey, radius_m: f64, segments: u32) -> Self {
        assert!(DETAILS.contains(&segments));
        let vertices = if segments == SEGMENTS {
            (segments + 1).pow(2)
        } else {
            (segments + 1).pow(2) + 4 * (SEGMENTS - segments) + 4 * (segments - 1)
        };
        let triangles = if segments == SEGMENTS {
            2 * segments * segments
        } else {
            2 * segments * segments + 4 * segments + 120
        };
        let mut positions_m = Vec::with_capacity(vertices as usize);
        let mut indices = Vec::with_capacity(3 * triangles as usize);
        if segments == SEGMENTS {
            let side = SEGMENTS + 1;
            for j in 0..side {
                for i in 0..side {
                    positions_m.push(key.vertex(i, j, SEGMENTS, radius_m).unwrap());
                }
            }
            for j in 0..SEGMENTS {
                for i in 0..SEGMENTS {
                    let a = j * side + i;
                    indices.extend_from_slice(&[a, a + 1, a + side, a + 1, a + side + 1, a + side]);
                }
            }
        } else {
            let step = SEGMENTS / segments;
            let mut vertices = BTreeMap::new();
            let mut vertex = |i, j| {
                *vertices.entry((i, j)).or_insert_with(|| {
                    let id = positions_m.len() as u32;
                    positions_m.push(key.vertex(i, j, SEGMENTS, radius_m).unwrap());
                    id
                })
            };
            for j in (0..SEGMENTS).step_by(step as usize) {
                for i in (0..SEGMENTS).step_by(step as usize) {
                    if i > 0 && j > 0 && i + step < SEGMENTS && j + step < SEGMENTS {
                        let [a, b, c, d] =
                            [(i, j), (i + step, j), (i, j + step), (i + step, j + step)]
                                .map(|(x, y)| vertex(x, y));
                        indices.extend_from_slice(&[a, b, c, b, d, c]);
                    } else {
                        // Contour anti-horaire dans U,V. Un éventail relie
                        // le bord fin à l'intérieur grossier sans T-junction.
                        let mut ring = Vec::new();
                        let bottom = if j == 0 { 1 } else { step };
                        let right = if i + step == SEGMENTS { 1 } else { step };
                        let top = if j + step == SEGMENTS { 1 } else { step };
                        let left = if i == 0 { 1 } else { step };
                        for k in (0..step).step_by(bottom as usize) {
                            ring.push(vertex(i + k, j));
                        }
                        for k in (0..step).step_by(right as usize) {
                            ring.push(vertex(i + step, j + k));
                        }
                        for k in (0..step).step_by(top as usize) {
                            ring.push(vertex(i + step - k, j + step));
                        }
                        for k in (0..step).step_by(left as usize) {
                            ring.push(vertex(i, j + step - k));
                        }
                        let center = vertex(i + step / 2, j + step / 2);
                        for k in 0..ring.len() {
                            indices.extend_from_slice(&[
                                center,
                                ring[k],
                                ring[(k + 1) % ring.len()],
                            ]);
                        }
                    }
                }
            }
        }
        Self {
            key,
            segments,
            positions_m,
            indices,
        }
    }
}

/// Borne inférieure de la distance à toute la surface du patch, en mètres.
/// La normalisation est Lipschitz sur le rectangle de face (norme >= min_norm).
/// Sa demi-diagonale avant projection vaut sqrt(2)/count.
fn patch_distance(key: PatchKey, up: DVec3, radius_m: f64) -> f64 {
    let count = key.count().unwrap() as f64;
    let minimum = |coordinate: u32| {
        let lo = 2.0 * coordinate as f64 / count - 1.0;
        let hi = lo + 2.0 / count;
        if lo <= 0.0 && hi >= 0.0 {
            0.0
        } else {
            lo.abs().min(hi.abs())
        }
    };
    let min_norm = (1.0 + minimum(key.x).powi(2) + minimum(key.y).powi(2)).sqrt();
    let reach = std::f64::consts::SQRT_2 * radius_m / (count * min_norm);
    (key.center_direction().unwrap().distance(up) * radius_m - reach).max(0.0)
}
fn select_segments(distance_m: f64, previous: Option<u32>) -> u32 {
    // Zone fine toujours >=80 m : protège le disque de requête/index de 72 m.
    // Départs 110/200 m, retours 80/160 m : pas de sélection à chaque frame.
    if distance_m <= 80.0 || previous == Some(32) && distance_m <= 110.0 {
        32
    } else if distance_m <= 160.0 || previous == Some(16) && distance_m <= 200.0 {
        16
    } else {
        8
    }
}

pub struct GlobeTerrain {
    pub center: PatchKey,
    pub patches: HashMap<PatchKey, Arc<SurfacePatch>>,
    pub constructions: usize,
    pub rebases: usize,
    pub lod_changes: usize,
    pub adaptive: bool,
    /// Durées murales de la dernière préparation, pas des temps GPU.
    pub geometry_ms: f64,
    pub index_ms: f64,
    triangles: Vec<[DVec3; 3]>,
    bins: BTreeMap<(i32, i32), Vec<usize>>,
}
impl GlobeTerrain {
    pub fn new(up: DVec3, orientation: DQuat, radius_m: f64) -> Self {
        Self::with_lod(
            up,
            orientation,
            radius_m,
            std::env::var("SOLAR_TERRAIN_LOD").as_deref() != Ok("fixed"),
        )
    }
    pub fn with_lod(up: DVec3, orientation: DQuat, radius_m: f64, adaptive: bool) -> Self {
        Self::build(up, orientation, radius_m, None, adaptive)
    }
    pub fn rebased(&self, up: DVec3, orientation: DQuat, radius_m: f64) -> Self {
        Self::build(up, orientation, radius_m, Some(self), self.adaptive)
    }
    fn build(
        up: DVec3,
        orientation: DQuat,
        radius_m: f64,
        previous: Option<&Self>,
        adaptive: bool,
    ) -> Self {
        let started = std::time::Instant::now();
        let center = key_at(up);
        let keys = neighborhood(center);
        let mut constructions = previous.map_or(0, |p| p.constructions);
        let mut lod_changes = previous.map_or(0, |p| p.lod_changes);
        let patches: HashMap<_, _> = keys
            .into_iter()
            .map(|key| {
                let old = previous.and_then(|p| p.patches.get(&key));
                let segments = if adaptive {
                    select_segments(patch_distance(key, up, radius_m), old.map(|p| p.segments))
                } else {
                    SEGMENTS
                };
                if old.is_some_and(|p| p.segments != segments) {
                    lod_changes += 1;
                }
                let patch = old
                    .filter(|p| p.segments == segments)
                    .cloned()
                    .unwrap_or_else(|| {
                        constructions += 1;
                        Arc::new(SurfacePatch::with_segments(key, radius_m, segments))
                    });
                (key, patch)
            })
            .collect();
        let geometry_ms = started.elapsed().as_secs_f64() * 1000.0;
        let indexed = std::time::Instant::now();
        let origin = orientation * (DVec3::Y * radius_m);
        let inverse = orientation.inverse();
        let mut triangles = Vec::with_capacity(if adaptive {
            4096
        } else {
            patches.len() * (2 * SEGMENTS * SEGMENTS) as usize
        });
        let mut bins = BTreeMap::<_, Vec<_>>::new();
        for patch in patches.values() {
            let local: Vec<_> = patch
                .positions_m
                .iter()
                .map(|p| inverse * (*p - origin))
                .collect();
            for &ids in patch.indices.as_chunks::<3>().0 {
                let points = ids.map(|i| local[i as usize]);
                let lo = points.into_iter().reduce(DVec3::min).unwrap();
                let hi = points.into_iter().reduce(DVec3::max).unwrap();
                if adaptive
                    && (lo.x > INDEX_RADIUS_M
                        || hi.x < -INDEX_RADIUS_M
                        || lo.z > INDEX_RADIUS_M
                        || hi.z < -INDEX_RADIUS_M)
                {
                    continue;
                }
                let id = triangles.len();
                triangles.push(points);
                let low = if adaptive {
                    lo.max(DVec3::splat(-INDEX_RADIUS_M))
                } else {
                    lo
                };
                let high = if adaptive {
                    hi.min(DVec3::splat(INDEX_RADIUS_M))
                } else {
                    hi
                };
                for x in bin(low.x)..=bin(high.x) {
                    for z in bin(low.z)..=bin(high.z) {
                        bins.entry((x, z)).or_default().push(id);
                    }
                }
            }
        }
        Self {
            center,
            patches,
            constructions,
            rebases: previous.map_or(0, |p| p.rebases + 1),
            triangles,
            bins,
            adaptive,
            lod_changes,
            geometry_ms,
            index_ms: indexed.elapsed().as_secs_f64() * 1000.0,
        }
    }
    /// Maximum exact des triangles sous l'empreinte dans le disque opérationnel.
    pub fn support_height(&self, feet: DVec3, half: f64) -> Option<f64> {
        if !feet.is_finite()
            || !half.is_finite()
            || !(0.0..=1.0).contains(&half)
            || feet.x.hypot(feet.z) > REBASE_M + 2.0
        {
            return None;
        }
        self.query_range(feet, half, false).map(|v| v.1)
    }
    /// Admission construction : carré conservateur entièrement couvert, extrema exacts.
    pub fn height_range(&self, feet: DVec3, half: f64) -> Option<(f64, f64)> {
        self.query_range(feet, half, true)
    }
    fn query_range(&self, feet: DVec3, half: f64, include_min: bool) -> Option<(f64, f64)> {
        if !feet.is_finite()
            || !half.is_finite()
            || !(0.0..=5.0).contains(&half)
            || feet.x.hypot(feet.z) > REBASE_M + 2.0
        {
            return None;
        }
        let mut height: Option<f64> = None;
        let mut minimum = f64::INFINITY;
        let mut covered_area = 0.0;
        let first_bin = (bin(feet.x - half), bin(feet.z - half));
        // Un triangle est traité dans sa première case intersectée seulement :
        // aucune allocation de déduplication pendant une requête.
        for x in bin(feet.x - half)..=bin(feet.x + half) {
            for z in bin(feet.z - half)..=bin(feet.z + half) {
                if let Some(ids) = self.bins.get(&(x, z)) {
                    for &id in ids {
                        let triangle = self.triangles[id];
                        let lo = triangle.into_iter().reduce(DVec3::min).unwrap();
                        if x != bin(lo.x).max(first_bin.0) || z != bin(lo.z).max(first_bin.1) {
                            continue;
                        }
                        if let Some((h, area)) = triangle_support_area(
                            triangle,
                            feet.x - half,
                            feet.x + half,
                            feet.z - half,
                            feet.z + half,
                        ) {
                            height = Some(height.map_or(h, |v| v.max(h)));
                            if include_min {
                                let inverted = triangle.map(|p| DVec3::new(p.x, -p.y, p.z));
                                minimum = minimum.min(
                                    -triangle_support_area(
                                        inverted,
                                        feet.x - half,
                                        feet.x + half,
                                        feet.z - half,
                                        feet.z + half,
                                    )?
                                    .0,
                                );
                            }
                            covered_area += area;
                        }
                    }
                }
            }
        }
        // Le disque opérationnel autour de l'origine (64 m + un sous-pas) est
        // couvert par le voisinage à deux anneaux, testé aux arêtes/coins.
        if feet.x.hypot(feet.z) > REBASE_M + 2.0 {
            return None;
        }
        // Projection locale injective : les triangles ne se recouvrent pas.
        // Tolérance arithmétique en m² ; une empreinte partielle est refusée.
        if covered_area < 4.0 * half * half - 1e-9 {
            return None;
        }
        height.map(|maximum| (minimum, maximum))
    }
    pub fn detail_counts(&self) -> ([usize; 3], usize, usize) {
        let mut details = [0; 3];
        let mut vertices = 0;
        let mut triangles = 0;
        for patch in self.patches.values() {
            details[DETAILS.iter().position(|&s| s == patch.segments).unwrap()] += 1;
            vertices += patch.positions_m.len();
            triangles += patch.indices.len() / 3;
        }
        (details, vertices, triangles)
    }
    pub fn index_counts(&self) -> (usize, usize, usize) {
        (
            self.bins.len(),
            self.bins.values().map(Vec::len).sum(),
            self.triangles.len(),
        )
    }
}
fn bin(value: f64) -> i32 {
    (value / BIN_M).floor() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hysteresis_keeps_each_detail_in_its_band_without_ping_pong() {
        for _ in 0..1000 {
            for distance in [80.01, 95.0, 109.99] {
                assert_eq!(select_segments(distance, Some(32)), 32);
                assert_eq!(select_segments(distance, Some(16)), 16);
            }
            for distance in [160.01, 180.0, 199.99] {
                assert_eq!(select_segments(distance, Some(16)), 16);
                assert_eq!(select_segments(distance, Some(8)), 8);
            }
        }
        assert_eq!(select_segments(79.99, Some(8)), 32);
        assert_eq!(select_segments(110.01, Some(32)), 16);
        assert_eq!(select_segments(200.01, Some(16)), 8);
        assert_eq!(select_segments(159.99, Some(8)), 16);
    }
    #[test]
    fn partial_footprints_are_rejected_even_with_a_valid_center_height() {
        let mut terrain = GlobeTerrain::new(DVec3::Y, DQuat::IDENTITY, 3_389_500.0);
        assert!(terrain.support_height(DVec3::ZERO, 0.3).is_some());
        terrain.bins.retain(|&(x, _), _| x >= 0);
        assert!(terrain.support_height(DVec3::ZERO, 0.0).is_some());
        assert!(terrain.support_height(DVec3::ZERO, 0.3).is_none());
        assert!(terrain.support_height(DVec3::ZERO, f64::MAX).is_none());
    }
}
