pub mod catalog;
pub mod celestial_body;
pub mod coordinates;
pub mod orbital_mechanics;
pub mod units;

use crate::{GameSet, config::GameConfig};
use bevy::prelude::*;
use catalog::SolarSystem;
use celestial_body::CelestialBody;
use coordinates::AstronomicalPosition;
use units::{DAY, J2000};

pub const MIN_JD: f64 = 2_378_496.5; // 1800-01-01
pub const MAX_JD: f64 = 2_469_807.5; // 2050-01-01, limite conservatrice JPL
#[derive(Resource, Debug, Clone)]
pub struct SimulationClock {
    pub jd: f64,
    pub speed_index: usize,
    pub paused: bool,
}
impl SimulationClock {
    pub const SPEEDS: [f64; 4] = [1.0, 10.0, 100.0, 1000.0];
    pub fn advance(&mut self, real_seconds: f64) {
        if !self.paused {
            self.jd = (self.jd + real_seconds * Self::SPEEDS[self.speed_index] / DAY)
                .clamp(MIN_JD, MAX_JD);
            if self.jd == MAX_JD {
                self.paused = true;
            }
        }
    }
    pub fn seconds_since_epoch(&self) -> f64 {
        (self.jd - J2000) * DAY
    }
}
pub struct UniversePlugin;
impl Plugin for UniversePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SolarSystem>()
            .add_systems(Startup, setup)
            .add_systems(Update, tick.in_set(GameSet::Simulation));
    }
}
fn setup(mut commands: Commands, config: Res<GameConfig>, solar: Res<SolarSystem>) {
    commands.insert_resource(SimulationClock {
        jd: config.start_jd,
        speed_index: 0,
        paused: false,
    });
    for body in &solar.bodies {
        commands.spawn((
            body.clone(),
            body.id,
            AstronomicalPosition(solar.position(body.id, config.start_jd)),
        ));
    }
}
fn tick(
    time: Res<Time<Real>>,
    mut clock: ResMut<SimulationClock>,
    mut solar: ResMut<SolarSystem>,
    mut bodies: Query<(&CelestialBody, &mut AstronomicalPosition)>,
) {
    clock.advance(time.delta_secs_f64());
    solar.cache_positions(clock.jd);
    for (body, mut position) in &mut bodies {
        position.0 = solar.position(body.id, clock.jd);
    }
}
