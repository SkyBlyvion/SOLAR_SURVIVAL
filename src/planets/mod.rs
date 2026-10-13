//! Clés cube-sphere et terrain martien local, sans génération globale.
mod cube_sphere;
pub use cube_sphere::{CubeFace, FaceCoordinates, PatchEdge, PatchKey, PatchNeighbor, PatchPair};
pub mod cube_migration;
pub mod cube_terrain;
pub mod terrain;
#[derive(Clone, Copy, Debug)]
pub struct GenerationSettings {
    pub seed: u64,
    pub max_lod: u8,
}
