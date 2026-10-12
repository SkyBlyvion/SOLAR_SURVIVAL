use crate::universe::{catalog::SolarSystem, celestial_body::BodyId};
use bevy::{math::DVec3, prelude::*};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MovementMode {
    Space,
    Surface,
}
#[derive(Resource)]
pub struct Explorer {
    /// Offset local f64 dans le repère de l'astre suivi, ou héliocentrique si None.
    pub anchor: Option<BodyId>,
    pub local_m: DVec3,
    pub rotation: Quat,
    pub selected: BodyId,
    pub speed_index: usize,
    pub boosted: bool,
    pub captured: bool,
    pub mode: MovementMode,
}
impl Default for Explorer {
    fn default() -> Self {
        Self {
            anchor: Some(BodyId::EARTH),
            local_m: DVec3::ZERO,
            rotation: Quat::IDENTITY,
            selected: BodyId::EARTH,
            speed_index: 3,
            boosted: false,
            captured: false,
            mode: MovementMode::Space,
        }
    }
}
impl Explorer {
    pub const SPEEDS: [f64; 6] = [
        5.0,
        250.0,
        20_000.0,
        20_000_000.0,
        2_000_000_000.0,
        100_000_000_000.0,
    ];
    pub const SPEED_NAMES: [&'static str; 6] = [
        "Marche",
        "Rapide",
        "Spatial",
        "Interplanétaire",
        "Supraluminique",
        "Hypervoyage",
    ];
    pub fn speed(&self) -> f64 {
        Self::SPEEDS[self.speed_index] * if self.boosted { 10.0 } else { 1.0 }
    }
    pub fn global(&self, solar: &SolarSystem, jd: f64) -> DVec3 {
        self.local_m + self.anchor.map_or(DVec3::ZERO, |id| solar.position(id, jd))
    }
    pub fn set_global(&mut self, global: DVec3, solar: &SolarSystem, jd: f64) {
        self.local_m = global - self.anchor.map_or(DVec3::ZERO, |id| solar.position(id, jd));
    }
    pub fn look_toward(&mut self, direction: DVec3) {
        if direction.length_squared() > 1e-12 {
            let direction = direction.normalize().as_vec3();
            let up = if direction.dot(Vec3::Y).abs() > 0.99 {
                Vec3::Z
            } else {
                Vec3::Y
            };
            self.rotation = Transform::default().looking_to(direction, up).rotation;
        }
    }
    pub fn visit(&mut self, solar: &SolarSystem, jd: f64) {
        let body = solar.body(self.selected);
        let position = solar.position(self.selected, jd);
        let outward = if self.selected == BodyId::SUN {
            DVec3::Z
        } else {
            (-position.normalize() + position.normalize().cross(DVec3::Y).normalize_or_zero() * 0.7)
                .normalize()
        };
        self.anchor = Some(self.selected);
        let stand_off = if self.selected == BodyId(6) { 5.5 } else { 3.5 };
        self.local_m = (outward + DVec3::Y * 0.25).normalize() * body.radius_m * stand_off;
        self.look_toward(-self.local_m);
    }
}
