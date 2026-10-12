use crate::{
    physics::surface::{self, EYE_HEIGHT_M, HALF_WIDTH_M, STEP_SECONDS},
    planets::terrain::MarsTerrain,
    player::{Explorer, MovementMode},
    universe::{
        catalog::SolarSystem,
        celestial_body::BodyId,
        units::{DAY, J2000},
    },
};
use bevy::{
    math::{DQuat, DVec3},
    prelude::*,
};

#[derive(Resource, Default)]
pub struct SurfaceState {
    /// État autoritaire dans les axes du site tournant, mètres et m/s réelles.
    pub feet: DVec3,
    pub velocity: DVec3,
    pub grounded: bool,
    pub yaw: f64,
    pub pitch: f64,
    pub accumulator: f64,
    pub jump_pending: bool,
}
impl SurfaceState {
    pub fn enter(
        &mut self,
        pilot: &mut Explorer,
        terrain: &mut MarsTerrain,
        solar: &SolarSystem,
        jd: f64,
    ) {
        let sun = -solar.position(BodyId::MARS, jd).normalize();
        let outward = (sun + sun.cross(DVec3::Y).normalize_or_zero() * 0.8).normalize();
        let rotation = solar.body(BodyId::MARS).rotation((jd - J2000) * DAY);
        // Choisir le côté éclairé dans les axes globaux, puis fixer ce site
        // dans les axes du corps, y compris à une époque différente de J2000.
        terrain.set_site(rotation.inverse() * outward);
        *self = Self::default();
        self.feet.y = terrain.support_height(self.feet, HALF_WIDTH_M) + 0.002;
        self.grounded = true;
        pilot.anchor = Some(BodyId::MARS);
        pilot.selected = BodyId::MARS;
        pilot.mode = MovementMode::Surface;
        self.sync(pilot, terrain, rotation);
    }
    pub fn look(&mut self, delta: Vec2) {
        self.yaw = (self.yaw - delta.x as f64 * 0.002).rem_euclid(std::f64::consts::TAU);
        self.pitch = (self.pitch - delta.y as f64 * 0.002).clamp(-1.55, 1.55);
    }
    pub fn advance(
        &mut self,
        pilot: &Explorer,
        terrain: &MarsTerrain,
        keys: &ButtonInput<KeyCode>,
        seconds: f64,
        mu: f64,
    ) {
        let mut input = DVec3::ZERO;
        if pilot.captured {
            input.x =
                (keys.pressed(KeyCode::KeyD) as i32 - keys.pressed(KeyCode::KeyA) as i32) as f64;
            input.z =
                (keys.pressed(KeyCode::KeyS) as i32 - keys.pressed(KeyCode::KeyW) as i32) as f64;
            self.jump_pending |= keys.just_pressed(KeyCode::Space);
        } else {
            self.jump_pending = false;
        }
        let desired = DQuat::from_rotation_y(self.yaw) * input.normalize_or_zero() * pilot.speed();
        // Maximum 30 sous-pas par frame ; les longues interruptions ne sont pas rattrapées.
        self.accumulator += seconds.min(0.25);
        while self.accumulator + 1e-12 >= STEP_SECONDS {
            surface::step(
                terrain,
                &mut self.feet,
                &mut self.velocity,
                &mut self.grounded,
                desired,
                self.jump_pending,
                mu,
            );
            self.jump_pending = false;
            self.accumulator = (self.accumulator - STEP_SECONDS).max(0.0);
        }
    }
    pub fn eye(&self, terrain: &MarsTerrain) -> DVec3 {
        self.feet + terrain.site_relative(self.feet).normalize() * EYE_HEIGHT_M
    }
    /// Visée dans le site : indépendante de l'orbite et des proxies f32.
    pub fn local_rotation(&self, terrain: &MarsTerrain) -> DQuat {
        DQuat::from_rotation_arc(DVec3::Y, terrain.site_relative(self.feet).normalize())
            * DQuat::from_rotation_y(self.yaw)
            * DQuat::from_rotation_x(self.pitch)
    }
    /// Publication de la pose après la physique ; également sans sous-pas.
    /// En l'air, le joueur reste dans le repère tournant (compromis cinématique).
    pub fn sync(&self, pilot: &mut Explorer, terrain: &MarsTerrain, body_rotation: DQuat) {
        let frame = terrain.inertial_frame(body_rotation);
        pilot.local_m = frame.to_global(self.eye(terrain));
        pilot.rotation = frame.orientation * self.local_rotation(terrain);
    }
}
