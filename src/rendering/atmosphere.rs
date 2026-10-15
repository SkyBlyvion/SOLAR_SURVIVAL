//! Météo visuelle déterministe, sans solveur climatique ni pression de gameplay.
//! Couleurs, opacités, durées et altitude nuageuse sont des conventions artistiques.
use crate::{
    planets::terrain::MarsTerrain,
    player::surface::SurfaceState,
    universe::{SimulationClock, catalog::SolarSystem, celestial_body::BodyId},
};
use bevy::{math::DVec3, prelude::Vec3};

/// Ordre de grandeur moyen publié (~0,6 % de 101325 Pa), pas une pression locale simulée.
pub const MEAN_PRESSURE_PA: f64 = 600.0;
pub const CLOUD_HEIGHT_M: f64 = 20_000.0;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Weather {
    /// Opacité visuelle sans unité, non assimilée à une mesure instrumentale.
    pub dust: f32,
    pub clouds: f32,
    pub storm: f32,
    pub drift: [f32; 2],
}
impl Default for Weather {
    fn default() -> Self {
        Self {
            dust: 0.35,
            clouds: 0.0,
            storm: 0.0,
            drift: [0.0; 2],
        }
    }
}
fn random(value: u64) -> f64 {
    let mut x = value.wrapping_add(0x9e3779b97f4a7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    ((x ^ (x >> 31)) >> 11) as f64 / (1_u64 << 53) as f64
}
/// Une fenêtre potentielle par six rotations martiennes, avec enveloppe continue.
/// L'événement dure 0,7 sol ; la probabilité par fenêtre est 55 % (choix de jeu).
pub fn sample(seconds: f64, sol_seconds: f64, body: DVec3, seed: u64) -> Weather {
    let sol = seconds / sol_seconds;
    let cycle = (sol / 6.0).floor();
    let phase = sol.rem_euclid(6.0);
    let draw = random(seed ^ (cycle as i64 as u64).wrapping_mul(7919));
    let center = 1.0 + random(seed.wrapping_add(cycle as i64 as u64)) * 4.0;
    let envelope = (1.0 - ((phase - center) / 0.35).abs()).clamp(0.0, 1.0);
    let storm = if draw < 0.55 {
        envelope * envelope * (3.0 - 2.0 * envelope)
    } else {
        0.0
    };
    let region = body.normalize();
    let slow = (sol * 0.73 + region.dot(DVec3::new(1.7, 2.1, -0.8)) + random(seed) * 6.0).sin();
    let dust = 0.3 + 0.18 * (slow * 0.5 + 0.5) + storm * 2.2;
    let clouds = (0.35 + 0.25 * (sol * 1.13 + region.z * 2.0).sin()) * (1.0 - storm);
    // Déplacement périodique entier du motif shader ; pas de saut après des siècles.
    let drift = [
        (sol * 3.0).rem_euclid(std::f64::consts::TAU) as f32,
        (sol * 1.5).rem_euclid(std::f64::consts::TAU) as f32,
    ];
    Weather {
        dust: dust as f32,
        clouds: clouds as f32,
        storm: storm as f32,
        drift,
    }
}
pub fn current(
    terrain: &MarsTerrain,
    state: &SurfaceState,
    solar: &SolarSystem,
    clock: &SimulationClock,
) -> Weather {
    if terrain
        .globe
        .as_ref()
        .is_some_and(|g| g.relief_seed.is_some())
    {
        sample(
            clock.seconds_since_epoch(),
            solar.body(BodyId::MARS).rotation_period_s,
            terrain.body_relative(state.eye(terrain)),
            terrain.seed,
        )
    } else {
        Weather::default()
    }
}
impl Weather {
    pub fn visibility_m(self) -> (f32, f32) {
        let clarity = ((0.48 - self.dust) / 0.18).clamp(0.0, 1.0);
        let end = (5000.0 + 3500.0 * clarity) * (1.0 - self.storm) + 150.0 * self.storm;
        (end * (0.18 - 0.08 * self.storm), end)
    }
    /// Intensité relative au ciel antérieur : présentation, pas une photométrie mesurée.
    pub fn direct_light(self, elevation: f64) -> f32 {
        let air = 1.0 / (elevation.max(0.0) as f32 + 0.18);
        (-(self.dust - 0.35).max(0.0) * air).exp() * (1.0 - self.clouds * 0.18)
    }
    pub fn palette(self, elevation: f64) -> (Vec3, Vec3, f32) {
        let (zenith, horizon, day) = super::sky::sky_palette(elevation);
        let ochre = Vec3::new(0.31, 0.16, 0.065);
        let dustiness = ((self.dust - 0.3) / 2.4).clamp(0.0, 1.0);
        (
            zenith.lerp(ochre * day + Vec3::splat(0.0007), dustiness * 0.8),
            horizon.lerp(ochre * day + Vec3::splat(0.002), dustiness),
            day,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn weather_is_reproducible_continuous_and_has_occasional_storms() {
        let mut storm_frames = 0;
        let mut clear_frames = 0;
        for i in -1000..10000 {
            let seconds = i as f64 * 1000.0;
            let w = sample(seconds, 88775.0, DVec3::Y, 42);
            assert_eq!(w, sample(seconds, 88775.0, DVec3::Y, 42));
            assert!((0.3..=2.68).contains(&w.dust) && (0.0..=0.6).contains(&w.clouds));
            assert!((0.0..=1.0).contains(&w.storm));
            assert!((w.dust - sample(seconds + 0.01, 88775.0, DVec3::Y, 42).dust).abs() < 1e-4);
            let (start, end) = w.visibility_m();
            assert!(start > 0.0 && start < end && end <= 8500.0);
            storm_frames += usize::from(w.storm > 0.8);
            clear_frames += usize::from(w.storm == 0.0);
        }
        assert!(storm_frames > 10 && clear_frames > 9000);
        assert!(sample(0.0, 88775.0, DVec3::X, 42) != sample(0.0, 88775.0, DVec3::Y, 42));
    }
    #[test]
    fn storm_reduces_light_and_visibility_and_night_stays_dark() {
        let clear = Weather::default();
        let storm = Weather {
            dust: 2.6,
            storm: 1.0,
            ..clear
        };
        assert!(storm.direct_light(0.6) < clear.direct_light(0.6) * 0.1);
        assert!(storm.visibility_m().1 < clear.visibility_m().1 / 20.0);
        for w in [clear, storm] {
            let (z, h, day) = w.palette(-0.5);
            assert_eq!(day, 0.0);
            assert!(z.max_element() < 0.003 && h.max_element() < 0.005);
        }
    }
}
