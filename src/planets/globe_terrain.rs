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
    pub positions_m: Vec<DVec3>,
    pub indices: Vec<u32>,
}
impl SurfacePatch {
    fn new(key: PatchKey, radius_m: f64) -> Self {
        let side = SEGMENTS + 1;
        let mut positions_m = Vec::with_capacity((side * side) as usize);
        for j in 0..side {
            for i in 0..side {
                positions_m.push(key.vertex(i, j, SEGMENTS, radius_m).unwrap());
            }
        }
        let mut indices = Vec::with_capacity((6 * SEGMENTS * SEGMENTS) as usize);
        for j in 0..SEGMENTS {
            for i in 0..SEGMENTS {
                let a = j * side + i;
                // U×V est radial : orientation extérieure.
                indices.extend_from_slice(&[a, a + 1, a + side, a + 1, a + side + 1, a + side]);
            }
        }
        Self {
            key,
            positions_m,
            indices,
        }
    }
}

pub struct GlobeTerrain {
    pub center: PatchKey,
    pub patches: HashMap<PatchKey, Arc<SurfacePatch>>,
    pub constructions: usize,
    pub rebases: usize,
    triangles: Vec<[DVec3; 3]>,
    bins: BTreeMap<(i32, i32), Vec<usize>>,
}
impl GlobeTerrain {
    pub fn new(up: DVec3, orientation: DQuat, radius_m: f64) -> Self {
        Self::build(up, orientation, radius_m, None)
    }
    pub fn rebased(&self, up: DVec3, orientation: DQuat, radius_m: f64) -> Self {
        Self::build(up, orientation, radius_m, Some(self))
    }
    fn build(up: DVec3, orientation: DQuat, radius_m: f64, previous: Option<&Self>) -> Self {
        let center = key_at(up);
        let keys = neighborhood(center);
        let mut constructions = previous.map_or(0, |p| p.constructions);
        let patches: HashMap<_, _> = keys
            .into_iter()
            .map(|key| {
                let patch = previous
                    .and_then(|p| p.patches.get(&key))
                    .cloned()
                    .unwrap_or_else(|| {
                        constructions += 1;
                        Arc::new(SurfacePatch::new(key, radius_m))
                    });
                (key, patch)
            })
            .collect();
        let origin = orientation * (DVec3::Y * radius_m);
        let inverse = orientation.inverse();
        let mut triangles = Vec::with_capacity(patches.len() * (2 * SEGMENTS * SEGMENTS) as usize);
        let mut bins = BTreeMap::<_, Vec<_>>::new();
        for patch in patches.values() {
            let local: Vec<_> = patch
                .positions_m
                .iter()
                .map(|p| inverse * (*p - origin))
                .collect();
            for &ids in patch.indices.as_chunks::<3>().0 {
                let points = ids.map(|i| local[i as usize]);
                let id = triangles.len();
                triangles.push(points);
                let lo = points.into_iter().reduce(DVec3::min).unwrap();
                let hi = points.into_iter().reduce(DVec3::max).unwrap();
                for x in bin(lo.x)..=bin(hi.x) {
                    for z in bin(lo.z)..=bin(hi.z) {
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
        let mut height: Option<f64> = None;
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
        height
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
    fn partial_footprints_are_rejected_even_with_a_valid_center_height() {
        let mut terrain = GlobeTerrain::new(DVec3::Y, DQuat::IDENTITY, 3_389_500.0);
        assert!(terrain.support_height(DVec3::ZERO, 0.3).is_some());
        terrain.bins.retain(|&(x, _), _| x >= 0);
        assert!(terrain.support_height(DVec3::ZERO, 0.0).is_some());
        assert!(terrain.support_height(DVec3::ZERO, 0.3).is_none());
        assert!(terrain.support_height(DVec3::ZERO, f64::MAX).is_none());
    }
}
