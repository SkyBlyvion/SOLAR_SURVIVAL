use crate::universe::{MAX_JD, MIN_JD};
use bevy::prelude::*;
#[derive(Resource, Debug)]
pub struct GameConfig {
    pub start_jd: f64,
    pub smoke_seconds: Option<f64>,
}
impl Default for GameConfig {
    fn default() -> Self {
        Self {
            // J2000 donne une date reproductible; SOLAR_JD accepte un jour julien dans la plage JPL.
            start_jd: std::env::var("SOLAR_JD")
                .ok()
                .and_then(|s| s.parse::<f64>().ok())
                .filter(|v| v.is_finite() && (MIN_JD..=MAX_JD).contains(v))
                .unwrap_or(2451545.0),
            smoke_seconds: std::env::var("SOLAR_SMOKE_SECONDS")
                .ok()
                .and_then(|s| s.parse::<f64>().ok())
                .filter(|v| v.is_finite() && *v >= 5.0),
        }
    }
}
