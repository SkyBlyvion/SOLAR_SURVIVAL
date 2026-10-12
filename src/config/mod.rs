use crate::universe::{MAX_JD, MIN_JD};
use bevy::prelude::*;
mod cube_validation;
pub mod profiling;
pub mod validation;
#[derive(Resource, Debug)]
pub struct GameConfig {
    pub start_jd: f64,
    pub smoke_seconds: Option<f64>,
    pub surface_smoke_seconds: Option<f64>,
    pub rotation_smoke_seconds: Option<f64>,
    pub p2_seconds: Option<f64>,
    pub cube_surface: bool,
    pub cube_validation_dir: Option<std::path::PathBuf>,
    pub profile: Option<profiling::ProfileConfig>,
}
impl Default for GameConfig {
    fn default() -> Self {
        let profile = profiling::ProfileConfig::from_env();
        Self {
            cube_surface: std::env::var("SOLAR_CUBE_SURFACE").is_ok_and(|s| s == "1")
                && profile.is_none(),
            cube_validation_dir: std::env::var_os("SOLAR_CUBE_VALIDATE_DIR")
                .map(std::path::PathBuf::from),
            profile,
            p2_seconds: std::env::var("SOLAR_P2_SECONDS")
                .ok()
                .and_then(|s| s.parse::<f64>().ok())
                .filter(|v| v.is_finite() && (160.0..=300.0).contains(v)),
            // J2000 donne une date reproductible; SOLAR_JD accepte un jour julien dans la plage JPL.
            start_jd: std::env::var("SOLAR_JD")
                .ok()
                .and_then(|s| s.parse::<f64>().ok())
                .filter(|v| v.is_finite() && (MIN_JD..=MAX_JD).contains(v))
                .unwrap_or(2451545.0),
            surface_smoke_seconds: std::env::var("SOLAR_SURFACE_SMOKE_SECONDS")
                .ok()
                .and_then(|s| s.parse::<f64>().ok())
                .filter(|v| v.is_finite() && *v >= 40.0),
            rotation_smoke_seconds: std::env::var("SOLAR_ROTATION_SMOKE_SECONDS")
                .ok()
                .and_then(|s| s.parse::<f64>().ok())
                .filter(|v| v.is_finite() && *v >= 40.0),
            smoke_seconds: std::env::var("SOLAR_SMOKE_SECONDS")
                .ok()
                .and_then(|s| s.parse::<f64>().ok())
                .filter(|v| v.is_finite() && *v >= 5.0),
        }
    }
}
impl GameConfig {
    pub fn automated(&self) -> bool {
        self.profile.is_some()
            || (self.cube_surface && self.cube_validation_dir.is_some())
            || self.p2_seconds.is_some()
            || self.smoke_seconds.is_some()
            || self.surface_smoke_seconds.is_some()
            || self.rotation_smoke_seconds.is_some()
    }
}
