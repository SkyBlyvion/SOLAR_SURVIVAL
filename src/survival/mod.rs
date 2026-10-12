//! Données de survie préparées pour le joueur FPS futur; aucune consommation en caméra spatiale.
use bevy::prelude::*;
#[derive(Component, Clone, Debug)]
pub struct Vitals {
    pub health: f32,
    pub oxygen_seconds: f64,
    pub energy: f32,
    pub temperature_kelvin: f32,
}
impl Default for Vitals {
    fn default() -> Self {
        Self {
            health: 100.0,
            oxygen_seconds: 3600.0,
            energy: 100.0,
            temperature_kelvin: 293.15,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ResourceKind {
    Water,
    Iron,
    Ice,
}
#[derive(Component, Default, Debug)]
pub struct Inventory {
    pub stacks: Vec<(ResourceKind, u32)>,
}
