use bevy::{
    math::{DQuat, DVec3},
    prelude::*,
};

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct AstronomicalPosition(pub DVec3);

/// Repère rigide f64. Les repères peuvent être composés: héliocentrique → planète → zone.
#[derive(Clone, Copy, Debug)]
pub struct ReferenceFrame {
    pub origin: DVec3,
    pub orientation: DQuat,
}
impl ReferenceFrame {
    pub const HELIOCENTRIC: Self = Self {
        origin: DVec3::ZERO,
        orientation: DQuat::IDENTITY,
    };
    pub fn to_global(self, local: DVec3) -> DVec3 {
        self.origin + self.orientation * local
    }
    pub fn to_local(self, global: DVec3) -> DVec3 {
        self.orientation.inverse() * (global - self.origin)
    }
    pub fn compose(self, child: Self) -> Self {
        Self {
            origin: self.to_global(child.origin),
            orientation: self.orientation * child.orientation,
        }
    }
    pub fn convert(local: DVec3, from: Self, to: Self) -> DVec3 {
        to.to_local(from.to_global(local))
    }
}

/// Origine flottante utilisée par le rendu, reconstruite depuis la caméra f64.
#[derive(Resource, Default, Debug)]
pub struct RenderOrigin(pub DVec3);

/// Proxy relatif conservant le rayon angulaire r/d, jamais réinjecté dans la simulation.
/// La profondeur est compressée de manière monotone, bornée à 10 000 unités GPU.
pub fn render_proxy(relative_m: DVec3, radius_m: f64) -> (Vec3, f32) {
    let distance = relative_m.length();
    let scale = 1e-4 / (1.0 + distance / 1e8);
    ((relative_m * scale).as_vec3(), (radius_m * scale) as f32)
}

/// Conversion des zones de terrain locales, bornée avant le cast.
pub fn local_to_render(local_m: DVec3) -> Option<Vec3> {
    (local_m.is_finite() && local_m.abs().max_element() <= 100_000.0).then(|| local_m.as_vec3())
}
