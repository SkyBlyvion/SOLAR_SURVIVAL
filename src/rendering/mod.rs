pub mod atmosphere;
mod buildings;
mod construction;
mod ground;
mod planets;
pub mod sky;
mod stars;
pub mod terrain;
mod textures;
use crate::GameSet;
use bevy::prelude::*;
pub use planets::PlanetMaterial;
pub struct SpaceRenderingPlugin;
impl Plugin for SpaceRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<PlanetMaterial>::default())
            .add_plugins(MaterialPlugin::<ground::GroundMaterial>::default())
            .add_plugins(textures::TexturePlugin)
            .add_plugins((
                terrain::SurfaceRenderingPlugin,
                sky::SurfaceSkyPlugin,
                buildings::BuildingRenderingPlugin,
                construction::ConstructionRenderingPlugin,
            ))
            .insert_resource(ClearColor(Color::BLACK))
            .add_systems(Startup, (planets::spawn, stars::spawn))
            .add_systems(Update, planets::sync.in_set(GameSet::Rendering));
    }
}
