use super::orbital_mechanics::OrbitalElements;
use crate::planets::GenerationSettings;
use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Component)]
pub struct BodyId(pub usize);
impl BodyId {
    pub const SUN: Self = Self(0);
    pub const EARTH: Self = Self(3);
    pub const MARS: Self = Self(4);
    pub const MOON: Self = Self(9);
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BodyKind {
    Star,
    RockyPlanet,
    GasGiant,
    IceGiant,
    Satellite,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceKind {
    Plasma,
    Rock,
    OceanAndLand,
    Gas,
    IceGiant,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atmosphere {
    Exosphere,
    ThinCarbonDioxide,
    DenseCarbonDioxide,
    EarthLike,
    GiantEnvelope,
    Stellar,
}

#[derive(Component, Clone, Debug)]
pub struct CelestialBody {
    pub id: BodyId,
    pub name: &'static str,
    pub kind: BodyKind,
    pub radius_m: f64,
    pub mass_kg: f64,
    pub gravitational_parameter: f64,
    pub rotation_period_s: f64,
    pub axial_tilt_deg: f64,
    pub orbit: Option<OrbitalElements>,
    pub parent: Option<BodyId>,
    pub atmosphere: Atmosphere,
    pub surface: SurfaceKind,
    pub color: [f32; 3],
    pub generation: GenerationSettings,
}
