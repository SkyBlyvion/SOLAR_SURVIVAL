//! Champ de hauteur local sur la sphère moyenne de Mars, en mètres/f64.
//! Le relief est artistique. Un seul LOD, neuf tuiles autour du joueur.
use crate::universe::{catalog::SolarSystem, celestial_body::BodyId, coordinates::ReferenceFrame};
use bevy::math::{DQuat, DVec3};
use bevy::prelude::*;

pub const TILE_M: f64 = 128.0;
pub const CELL_M: f64 = 4.0;
pub const CELLS: usize = 32;
pub const SITE_LIMIT_M: f64 = 4096.0;

#[derive(Resource)]
pub struct MarsTerrain {
    pub radius_m: f64,
    pub seed: u64,
    /// Orientation fixe du site dans le repère du corps tournant de Mars.
    /// La rotation quotidienne ne modifie ni ce champ ni la révision des meshes.
    pub orientation: DQuat,
    pub revision: u64,
}
impl FromWorld for MarsTerrain {
    fn from_world(world: &mut World) -> Self {
        let mars = world.resource::<SolarSystem>().body(BodyId::MARS);
        Self {
            radius_m: mars.radius_m,
            seed: mars.generation.seed,
            orientation: DQuat::IDENTITY,
            revision: 0,
        }
    }
}
impl MarsTerrain {
    pub fn set_site(&mut self, outward: DVec3) {
        self.orientation = DQuat::from_rotation_arc(DVec3::Y, outward.normalize());
        self.revision = self.revision.wrapping_add(1);
    }
    /// Coordonnées centrées sur Mars dans les axes du site (avant rotation).
    pub fn site_relative(&self, point: DVec3) -> DVec3 {
        point + DVec3::Y * self.radius_m
    }
    /// Repère du site → axes héliocentriques, origine relative au centre de Mars.
    /// Ajouter la position orbitale seulement pour obtenir une position globale.
    pub fn inertial_frame(&self, body_rotation: DQuat) -> ReferenceFrame {
        let orientation = body_rotation * self.orientation;
        ReferenceFrame {
            origin: orientation * (DVec3::Y * self.radius_m),
            orientation,
        }
    }
    /// Position dans les axes du corps tournant, pas dans les axes inertiels.
    pub fn body_relative(&self, point: DVec3) -> DVec3 {
        self.orientation * self.site_relative(point)
    }
    pub fn chart(&self, body_relative: DVec3) -> DVec3 {
        self.orientation.inverse() * body_relative - DVec3::Y * self.radius_m
    }
    /// Seuls les sommets sont échantillonnés ; contacts et mesh partagent les triangles.
    pub fn vertex(&self, x: i32, z: i32) -> DVec3 {
        let x = x as f64 * CELL_M;
        let z = z as f64 * CELL_M;
        let phase = (self.seed % 997) as f64 * 0.01;
        let relief = 3.0 * (x / 43.0 + phase).sin() * (z / 61.0).sin()
            + 0.22 * (x / 9.0).sin() * (z / 13.0).cos();
        // Forme stable de sqrt(R²-x²-z²)-R, sans soustraction de deux grands nombres.
        let squared = x * x + z * z;
        let curvature =
            -squared / ((self.radius_m * self.radius_m - squared).sqrt() + self.radius_m);
        DVec3::new(x, curvature + relief, z)
    }
    pub fn triangles(&self, x: i32, z: i32) -> [[DVec3; 3]; 2] {
        let a = self.vertex(x, z);
        let b = self.vertex(x + 1, z);
        let c = self.vertex(x, z + 1);
        let d = self.vertex(x + 1, z + 1);
        [[a, c, b], [b, c, d]]
    }
    pub fn height(&self, x: f64, z: f64) -> f64 {
        let ix = (x / CELL_M).floor() as i32;
        let iz = (z / CELL_M).floor() as i32;
        let u = x / CELL_M - ix as f64;
        let v = z / CELL_M - iz as f64;
        let [a, c, b] = self.triangles(ix, iz)[0];
        if u + v <= 1.0 {
            a.y + u * (b.y - a.y) + v * (c.y - a.y)
        } else {
            let d = self.vertex(ix + 1, iz + 1);
            d.y + (1.0 - u) * (c.y - d.y) + (1.0 - v) * (b.y - d.y)
        }
    }
    /// Hauteur maximale exacte des triangles sous une empreinte rectangulaire.
    pub fn support_height(&self, feet: DVec3, half_width: f64) -> f64 {
        let min_x = feet.x - half_width;
        let max_x = feet.x + half_width;
        let min_z = feet.z - half_width;
        let max_z = feet.z + half_width;
        let mut height = f64::NEG_INFINITY;
        for x in (min_x / CELL_M).floor() as i32..=(max_x / CELL_M).floor() as i32 {
            for z in (min_z / CELL_M).floor() as i32..=(max_z / CELL_M).floor() as i32 {
                for triangle in self.triangles(x, z) {
                    let mut polygon = triangle.to_vec();
                    for (axis, bound, sign) in [
                        (0, min_x, 1.0),
                        (0, max_x, -1.0),
                        (2, min_z, 1.0),
                        (2, max_z, -1.0),
                    ] {
                        let mut clipped = Vec::with_capacity(6);
                        if let Some(mut previous) = polygon.last().copied() {
                            for &current in &polygon {
                                let a = (previous[axis] - bound) * sign;
                                let b = (current[axis] - bound) * sign;
                                if (a >= 0.0) != (b >= 0.0) {
                                    clipped.push(previous + (current - previous) * (a / (a - b)));
                                }
                                if b >= 0.0 {
                                    clipped.push(current);
                                }
                                previous = current;
                            }
                        }
                        polygon = clipped;
                    }
                    for point in polygon {
                        height = height.max(point.y);
                    }
                }
            }
        }
        height
    }
    pub fn tile_at(feet: DVec3) -> (i32, i32) {
        (
            (feet.x / TILE_M).floor() as i32,
            (feet.z / TILE_M).floor() as i32,
        )
    }
}
