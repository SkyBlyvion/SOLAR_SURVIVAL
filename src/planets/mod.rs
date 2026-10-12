//! Contrat minimal pour les futures zones cube-sphere, sans génération globale.
use bevy::math::DVec3;
#[derive(Clone, Copy, Debug)]
pub struct GenerationSettings {
    pub seed: u64,
    pub max_lod: u8,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CubeFace {
    PositiveX,
    NegativeX,
    PositiveY,
    NegativeY,
    PositiveZ,
    NegativeZ,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PatchKey {
    pub face: CubeFace,
    pub level: u8,
    pub x: u32,
    pub y: u32,
}
impl PatchKey {
    pub fn center_direction(self) -> Option<DVec3> {
        if self.level > 24 {
            return None;
        }
        let count = 1_u32 << self.level;
        if self.x >= count || self.y >= count {
            return None;
        }
        let u = 2.0 * (self.x as f64 + 0.5) / count as f64 - 1.0;
        let v = 2.0 * (self.y as f64 + 0.5) / count as f64 - 1.0;
        Some(
            match self.face {
                CubeFace::PositiveX => DVec3::new(1.0, v, -u),
                CubeFace::NegativeX => DVec3::new(-1.0, v, u),
                CubeFace::PositiveY => DVec3::new(u, 1.0, -v),
                CubeFace::NegativeY => DVec3::new(u, -1.0, v),
                CubeFace::PositiveZ => DVec3::new(u, v, 1.0),
                CubeFace::NegativeZ => DVec3::new(-u, v, -1.0),
            }
            .normalize(),
        )
    }
    pub fn deterministic_seed(self, seed: u64) -> u64 {
        let value = seed
            ^ ((self.face as u64) << 56)
            ^ ((self.level as u64) << 48)
            ^ ((self.x as u64) << 24)
            ^ self.y as u64;
        let mut z = value.wrapping_add(0x9e3779b97f4a7c15);
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}
