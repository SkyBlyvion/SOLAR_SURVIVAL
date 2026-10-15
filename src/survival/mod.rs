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

#[derive(Resource, Debug, Clone)]
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
        reserve_seconds: &mut f64,
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
        if shelter.is_some() && *reserve_seconds > 0.0 {
            let n = (CAPACITY_SECONDS - self.remaining_seconds)
                .min(seconds * REFILL_PER_SECOND)
                .min(*reserve_seconds);
            self.remaining_seconds += n;
            *reserve_seconds -= n;
            // Si la base se vide pendant une longue frame, le reste du temps consomme
            // de nouveau la combinaison ; aucune protection gratuite jusqu'à la frame suivante.
            if *reserve_seconds == 0.0 {
                self.remaining_seconds =
                    (self.remaining_seconds - (seconds - n / REFILL_PER_SECOND).max(0.0)).max(0.0);
                self.failed = self.remaining_seconds == 0.0;
            }
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
            || (config.persistence_validation_dir.is_some()
                || config.survival_validation_dir.is_some()
                || config.construction_validation_dir.is_some()
                || config.sky_validation_dir.is_some())
                && config.profile.is_none();
        app.insert_resource(Oxygen {
            enabled,
            ..default()
        })
        .init_resource::<Buildings>()
        .init_resource::<crate::base::Base>()
        .add_systems(
            Update,
            update
                .after(crate::camera::navigate)
                .in_set(GameSet::Navigation),
        );
    }
}
#[expect(
    clippy::too_many_arguments,
    reason = "Production de base puis service O2, ordre explicite."
)]
pub(crate) fn update(
    time: Res<Time<Real>>,
    menu: Option<Res<MenuState>>,
    pilot: Res<Explorer>,
    state: Res<SurfaceState>,
    terrain: Res<MarsTerrain>,
    buildings: Res<Buildings>,
    mut oxygen: ResMut<Oxygen>,
    mut base: ResMut<crate::base::Base>,
    solar: Res<crate::universe::catalog::SolarSystem>,
    clock: Res<crate::universe::SimulationClock>,
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
    let paused = menu.is_some_and(|m| m.is_open());
    if on_surface {
        base.seed_site(&terrain);
    }
    oxygen.shelter = shelter;
    if !on_surface || paused || oxygen.failed {
        return;
    }
    base.advance(
        time.delta_secs_f64(),
        &buildings,
        crate::base::Base::solar_power(&buildings, &terrain, &solar, &clock),
    );
    oxygen.advance(
        time.delta_secs_f64(),
        true,
        false,
        shelter.filter(|id| crate::base::Base::connected(&buildings, *id)),
        &mut base.oxygen_s,
    );
    oxygen.shelter = shelter;
}
