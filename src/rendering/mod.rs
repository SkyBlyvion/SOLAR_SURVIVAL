mod buildings;
mod planets;
mod stars;
pub mod terrain;
use crate::GameSet;
use bevy::prelude::*;
pub use planets::PlanetMaterial;
pub struct SpaceRenderingPlugin;
impl Plugin for SpaceRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<PlanetMaterial>::default())
            .add_plugins((
                terrain::SurfaceRenderingPlugin,
                buildings::BuildingRenderingPlugin,
            ))
            .insert_resource(ClearColor(Color::BLACK))
            .add_systems(Startup, (planets::spawn, stars::spawn))
            .add_systems(Update, planets::sync.in_set(GameSet::Rendering));
    }
}
