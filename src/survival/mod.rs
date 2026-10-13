//! Première boucle d'oxygène. Paramètres de gameplay, secondes réelles de réserve.
use crate::{
    GameSet,
    buildings::{BuildingId, Buildings},
    config::GameConfig,
    planets::terrain::MarsTerrain,
    player::{Explorer, MovementMode, surface::SurfaceState},
    ui::menu::MenuState,
};
use bevy::prelude::*;

pub const CAPACITY_SECONDS: f64 = 180.0;
pub const REFILL_PER_SECOND: f64 = 30.0;
pub const LOW_SECONDS: f64 = 30.0;

#[derive(Resource, Debug)]
pub struct Oxygen {
    pub enabled: bool,
    pub remaining_seconds: f64,
    pub failed: bool,
    pub shelter: Option<BuildingId>,
}
impl Default for Oxygen {
    fn default() -> Self {
        Self {
            enabled: true,
            remaining_seconds: CAPACITY_SECONDS,
            failed: false,
            shelter: None,
        }
    }
}
impl Oxygen {
    /// Menu et espace figent la réserve ; aucun delta orbital ni plafond physique.
    pub fn advance(
        &mut self,
        seconds: f64,
        surface: bool,
        menu: bool,
        shelter: Option<BuildingId>,
    ) {
        self.shelter = if surface { shelter } else { None };
        if !self.enabled || self.failed || !surface || menu {
            return;
        }
        let seconds = if seconds.is_finite() {
            seconds.max(0.0)
        } else {
            0.0
        };
        if shelter.is_some() {
            self.remaining_seconds =
                (self.remaining_seconds + seconds * REFILL_PER_SECOND).min(CAPACITY_SECONDS);
        } else {
            self.remaining_seconds = (self.remaining_seconds - seconds).max(0.0);
            self.failed = self.remaining_seconds == 0.0;
        }
    }
    pub fn restart(&mut self) {
        self.remaining_seconds = CAPACITY_SECONDS;
        self.failed = false;
        self.shelter = None;
    }
}

pub struct SurvivalPlugin;
impl Plugin for SurvivalPlugin {
    fn build(&self, app: &mut App) {
        let config = app.world().resource::<GameConfig>();
        // Les campagnes historiques gardent leurs scènes exactes ; le diagnostic P4 est actif.
        let enabled = !config.automated() && !config.cube_surface
            || config.survival_validation_dir.is_some() && config.profile.is_none();
        app.insert_resource(Oxygen {
            enabled,
            ..default()
        })
        .init_resource::<Buildings>()
        .add_systems(
            Update,
            update
                .after(crate::camera::navigate)
                .in_set(GameSet::Navigation),
        );
    }
}
fn update(
    time: Res<Time<Real>>,
    menu: Option<Res<MenuState>>,
    pilot: Res<Explorer>,
    state: Res<SurfaceState>,
    terrain: Res<MarsTerrain>,
    buildings: Res<Buildings>,
    mut oxygen: ResMut<Oxygen>,
) {
    if !oxygen.enabled {
        return;
    }
    let on_surface = pilot.mode == MovementMode::Surface;
    let shelter = if on_surface {
        buildings.shelter_at(terrain.body_relative(state.feet), terrain.orientation)
    } else {
        None
    };
    oxygen.advance(
        time.delta_secs_f64(),
        on_surface,
        menu.is_some_and(|m| m.is_open()),
        shelter,
    );
}
