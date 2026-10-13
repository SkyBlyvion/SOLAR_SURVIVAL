//! Deux cartes hors centre seulement : admission certifiée avant construction.
use super::{PatchKey, cube_terrain::CubeTerrain};
use crate::physics::surface::{HALF_WIDTH_M, PLAYER_HEIGHT_M};
use bevy::math::{DQuat, DVec3};

const ALTITUDE_M: f64 = 100.0;
const SLAB_HALF_M: f64 = 8.0;
const HYSTERESIS_M: f64 = 2.0;
/// Réserve pour le changement d'alignement de la boîte, sous la peau de 2 mm.
pub const BOX_DRIFT_LIMIT_M: f64 = 0.00025;
/// Expansion maximale par dimension des obstacles de ce parcours.
pub const OBSTACLE_EXPANSION_LIMIT_M: f64 = 0.0005;

#[derive(Clone, Copy, Debug)]
pub struct Chart {
    pub key: PatchKey,
    pub orientation: DQuat,
    pub min: DVec3,
    pub max: DVec3,
}
impl Chart {
    fn new(orientation: DQuat, cube: &CubeTerrain) -> Self {
        Self {
            key: cube.pair.keys[0],
            orientation,
            min: cube.min,
            max: cube.max,
        }
    }
    /// Soustraire les origines dans le corps en f64, avant la rotation.
    pub fn convert(self, point: DVec3, from: Self, radius_m: f64) -> DVec3 {
        self.orientation.inverse()
            * (from.orientation * point
                + (from.orientation * (DVec3::Y * radius_m)
                    - self.orientation * (DVec3::Y * radius_m)))
    }
    fn contains(self, p: DVec3, half: f64) -> bool {
        p.is_finite()
            && p.x - half >= self.min.x
            && p.x + half <= self.max.x
            && p.z - half >= self.min.z
            && p.z + half <= self.max.z
    }
}

pub struct CubeMigration {
    pub charts: [Chart; 2],
    /// Boîte d'admission exprimée dans la première carte, pas dans la carte active.
    pub center_x: f64,
    pub half_z: f64,
    pub box_drift_m: f64,
    pub rotation_rad: f64,
    /// Constructions CPU réelles (deux à l'entrée, une par remplacement admis).
    pub constructions: usize,
    radius_m: f64,
}
impl CubeMigration {
    /// Construire au plus deux paires CPU simultanées ; ne conserver que la
    /// première et les métadonnées certifiées de la seconde. Pas de mesh GPU ici.
    pub fn prepare(key: PatchKey, radius_m: f64) -> Option<(DQuat, CubeTerrain, Self)> {
        let (q0, first) = CubeTerrain::from_key(key, radius_m)?;
        let next_key = PatchKey {
            x: key.x.checked_add(1)?,
            ..key
        };
        let (q1, next) = CubeTerrain::from_key(next_key, radius_m)?;
        let charts = [Chart::new(q0, &first), Chart::new(q1, &next)];
        let delta = q1.inverse() * q0;
        // Les mêmes huit coins reboîtés dans la nouvelle carte : déplacement
        // maximal d'un coin, incluant le roulis hors centre, pas seulement Y.
        let mut box_drift_m: f64 = 0.0;
        for x in [-HALF_WIDTH_M, HALF_WIDTH_M] {
            for y in [0.0, PLAYER_HEIGHT_M] {
                for z in [-HALF_WIDTH_M, HALF_WIDTH_M] {
                    let corner = DVec3::new(x, y, z);
                    box_drift_m = box_drift_m.max((delta * corner).distance(corner));
                }
            }
        }
        if box_drift_m > BOX_DRIFT_LIMIT_M {
            return None;
        }
        let shift = charts[0].convert(DVec3::ZERO, charts[1], radius_m);
        let center_x = shift.x * 0.5;
        let center = DVec3::X * center_x;
        let half = HALF_WIDTH_M + BOX_DRIFT_LIMIT_M;
        let mut half_z: f64 = f64::INFINITY;
        // Chaque inégalité |a.p+t| <= h est satisfaite pour TOUS les coins
        // de la boîte x=centre±8, y=±100, z=±half_z dans les deux rectangles.
        for chart in charts {
            let a = chart.orientation.inverse() * q0;
            let p = chart.convert(center, charts[0], radius_m);
            for axis in [0, 2] {
                let available = chart.max[axis]
                    - half
                    - p[axis].abs()
                    - (a * DVec3::X)[axis].abs() * SLAB_HALF_M
                    - (a * DVec3::Y)[axis].abs() * ALTITUDE_M;
                if available <= 0.0 {
                    return None;
                }
                let coefficient = (a * DVec3::Z)[axis].abs();
                if coefficient > 1e-15 {
                    half_z = half_z.min(available / coefficient);
                }
            }
        }
        // Réserve supplémentaire de 20 % : pas de seuil repris du centre.
        half_z *= 0.8;
        if !half_z.is_finite() || half_z < 1.0 {
            return None;
        }
        let rotation_rad = 2.0 * delta.w.abs().clamp(0.0, 1.0).acos();
        Some((
            q0,
            first,
            Self {
                charts,
                center_x,
                half_z,
                box_drift_m,
                rotation_rad,
                constructions: 2,
                radius_m,
            },
        ))
    }
    pub fn active(&self, key: PatchKey) -> Option<usize> {
        self.charts.iter().position(|c| c.key == key)
    }
    /// Au plus une candidate. Refus sans allocation, même au bord du couloir.
    pub fn candidate(&self, key: PatchKey, feet: DVec3) -> Option<(Chart, DVec3)> {
        let active = self.active(key)?;
        let entry = self.charts[0].convert(feet, self.charts[active], self.radius_m);
        if !entry.is_finite()
            || (entry.x - self.center_x).abs() > SLAB_HALF_M
            || entry.y.abs() > ALTITUDE_M
            || entry.z.abs() > self.half_z
        {
            return None;
        }
        let offset = entry.x - self.center_x;
        if (active == 0 && offset <= HYSTERESIS_M * 0.5)
            || (active == 1 && offset >= -HYSTERESIS_M * 0.5)
        {
            return None;
        }
        let next = self.charts[1 - active];
        let point = next.convert(feet, self.charts[active], self.radius_m);
        let half = HALF_WIDTH_M + BOX_DRIFT_LIMIT_M;
        if !self.charts[active].contains(feet, half) || !next.contains(point, half) {
            return None;
        }
        Some((next, point))
    }
    /// Pose de préparation du diagnostic ; jamais comptée comme une marche.
    pub fn approach(&self, key: PatchKey) -> DVec3 {
        let active = self.active(key).expect("Une des deux cartes");
        self.charts[active].convert(
            DVec3::new(self.center_x - 3.0, 0.0, self.half_z * 0.5),
            self.charts[0],
            self.radius_m,
        )
    }
    pub fn preparation(&self, key: PatchKey) -> DVec3 {
        let active = self.active(key).expect("Une des deux cartes");
        self.charts[active].convert(
            DVec3::new(
                self.center_x + if active == 0 { 1.1 } else { -1.1 },
                0.0,
                self.half_z * 0.5,
            ),
            self.charts[0],
            self.radius_m,
        )
    }
}
