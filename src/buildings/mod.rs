//! Bâtiments de session : poses autoritaires dans le corps, indépendantes du terrain chargé.
use crate::{physics::surface::BodyBox, planets::terrain::MarsTerrain};
use bevy::{
    math::{DQuat, DVec3},
    prelude::*,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BuildingId(pub u64);

/// Abri ouvert par une porte de 2,4 m côté +Z. Toutes les dimensions sont du gameplay.
#[derive(Clone, Debug)]
pub struct OxygenShelter {
    pub id: BuildingId,
    pub position_m: DVec3,
    pub orientation: DQuat,
}
impl OxygenShelter {
    /// Pièces solides communes aux contacts et au rendu : deux côtés, fond, porte, toit.
    pub fn parts() -> [(DVec3, DVec3); 6] {
        [
            (DVec3::new(-3.0, 1.4, 0.0), DVec3::new(0.3, 3.8, 6.3)),
            (DVec3::new(3.0, 1.4, 0.0), DVec3::new(0.3, 3.8, 6.3)),
            (DVec3::new(0.0, 1.4, -3.0), DVec3::new(6.3, 3.8, 0.3)),
            (DVec3::new(-2.1, 1.4, 3.0), DVec3::new(1.8, 3.8, 0.3)),
            (DVec3::new(2.1, 1.4, 3.0), DVec3::new(1.8, 3.8, 0.3)),
            (DVec3::new(0.0, 3.3, 0.0), DVec3::new(6.3, 0.3, 6.3)),
        ]
    }
    pub fn solids(&self) -> impl Iterator<Item = BodyBox> + '_ {
        Self::parts().into_iter().map(|(center, size)| BodyBox {
            center: self.position_m + self.orientation * center,
            size,
            orientation: self.orientation,
        })
    }
    /// Volume utile : boîte joueur entièrement à l'intérieur, pas un rayon à travers les murs.
    pub fn contains(&self, feet_body: DVec3, player_orientation: DQuat) -> bool {
        let center = self.orientation.inverse() * (feet_body - self.position_m);
        let rotation = self.orientation.inverse() * player_orientation;
        for x in [-0.3, 0.3] {
            for y in [0.0, 1.8] {
                for z in [-0.3, 0.3] {
                    let p = center + rotation * DVec3::new(x, y, z);
                    if p.x.abs() > 2.7 || p.z.abs() > 2.7 || p.y < -0.5 || p.y > 3.15 {
                        return false;
                    }
                }
            }
        }
        true
    }
}

#[derive(Resource, Default)]
pub struct Buildings {
    pub shelters: Vec<OxygenShelter>,
    next_id: u64,
    /// Site de retour choisi une seule fois à la première arrivée en session.
    pub home_orientation: Option<DQuat>,
}
impl Buildings {
    /// Même installation pour l'abri initial, une deuxième instance et la construction P5.
    pub fn install_shelter(&mut self, position_m: DVec3, orientation: DQuat) -> BuildingId {
        assert!(position_m.is_finite() && orientation.is_finite());
        self.next_id += 1;
        let id = BuildingId(self.next_id);
        self.shelters.push(OxygenShelter {
            id,
            position_m,
            orientation: orientation.normalize(),
        });
        id
    }
    pub fn install_first(&mut self, terrain: &MarsTerrain) {
        if self.home_orientation.is_some() {
            return;
        }
        self.home_orientation = Some(terrain.orientation);
        let mut point = DVec3::new(0.0, 0.0, -6.0);
        point.y = terrain.height(point.x, point.z);
        self.install_shelter(terrain.body_relative(point), terrain.orientation);
    }
    /// Copie de collision fixe ; aucun calcul de placement pendant un rebasage.
    pub fn publish_colliders(&self, terrain: &mut MarsTerrain) {
        terrain.building_solids = self
            .shelters
            .iter()
            .flat_map(OxygenShelter::solids)
            .collect();
    }
    pub fn shelter_at(&self, feet_body: DVec3, orientation: DQuat) -> Option<BuildingId> {
        self.shelters
            .iter()
            .find(|b| b.contains(feet_body, orientation))
            .map(|b| b.id)
    }
}
