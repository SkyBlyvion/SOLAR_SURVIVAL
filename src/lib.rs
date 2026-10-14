//! Solar Survival: la simulation SI/f64 reste indépendante des proxies graphiques.
pub mod buildings;
pub mod camera;
pub mod config;
pub mod construction;
pub mod persistence;
pub mod physics;
pub mod planets;
pub mod player;
pub mod rendering;
pub mod survival;
pub mod ui;
pub mod universe;

use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameSet {
    Input,
    Simulation,
    Navigation,
    Rendering,
    Interface,
}

pub struct SolarSurvivalPlugin;
impl Plugin for SolarSurvivalPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                GameSet::Input,
                GameSet::Simulation,
                GameSet::Navigation,
                GameSet::Rendering,
                GameSet::Interface,
            )
                .chain(),
        )
        .add_plugins((
            universe::UniversePlugin,
            camera::ExplorationPlugin,
            survival::SurvivalPlugin,
            construction::ConstructionPlugin,
            persistence::PersistencePlugin,
            rendering::SpaceRenderingPlugin,
            ui::HudPlugin,
        ));
    }
}
