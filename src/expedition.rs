//! P10 : trois relevés locaux, retour physique au premier abri et récompense unique.
use crate::{
    GameSet,
    buildings::Buildings,
    construction::Construction,
    inventory::{Inventory, Item},
    planets::{relief, terrain::MarsTerrain},
    player::{Explorer, MovementMode, surface::SurfaceState},
    survival::Oxygen,
    ui::{inventory::InventoryScreen, menu::MenuState},
};
use bevy::{
    math::{DQuat, DVec3},
    prelude::*,
};

pub const SITE_COUNT: usize = 3;
pub const SCAN_RANGE_M: f64 = 3.0;
pub const SITE_OFFSETS: [(f64, f64); SITE_COUNT] =
    [(72.0, -96.0), (192.0, -144.0), (-216.0, -288.0)];
pub const SITE_NAMES: [&str; SITE_COUNT] = ["Relais Aurore", "Station Horizon", "Sonde Lointaine"];
pub const REWARDS: [(u32, u32); SITE_COUNT] = [(8, 4), (12, 6), (16, 8)];

#[derive(Clone, Debug)]
pub struct SurveySite {
    pub position_m: DVec3,
    pub orientation: DQuat,
}
#[derive(Resource, Default, Clone, Debug)]
pub struct Expedition {
    /// Cache déterministe de trois poses corporelles ; reconstruit depuis le premier abri.
    pub sites: Vec<SurveySite>,
    pub discovered: [bool; SITE_COUNT],
    /// Préfixe des missions rapportées/payées ; la mission active a cet indice.
    pub completed: usize,
    pub carrying: bool,
    pub message: &'static str,
}
impl Expedition {
    pub fn seed_site(&mut self, terrain: &MarsTerrain, buildings: &Buildings) {
        if !self.sites.is_empty() || !terrain.globe_enabled {
            return;
        }
        let Some(home) = buildings.shelters.first() else {
            return;
        };
        self.sites = SITE_OFFSETS
            .into_iter()
            .map(|(x, z)| {
                let direction =
                    (home.position_m + home.orientation * DVec3::new(x, 0.0, z)).normalize();
                SurveySite {
                    position_m: direction
                        * (terrain.radius_m
                            + relief::height(direction, terrain.radius_m, terrain.seed)),
                    orientation: DQuat::from_rotation_arc(home.orientation * DVec3::Y, direction)
                        * home.orientation,
                }
            })
            .collect();
    }
    pub fn valid_progress(&self, ready: bool, surface: bool, failed: bool) -> bool {
        self.completed <= SITE_COUNT
            && self.discovered.iter().enumerate().all(|(i, &d)| {
                if i < self.completed {
                    d
                } else if i > self.completed {
                    !d
                } else {
                    true
                }
            })
            && (!self.carrying
                || self.completed < SITE_COUNT
                    && self.discovered[self.completed]
                    && surface
                    && !failed)
            && (ready
                || self.completed == 0 && !self.carrying && !self.discovered.iter().any(|d| *d))
    }
    pub fn abandon(&mut self) {
        if self.carrying {
            self.carrying = false;
            self.message = "Données perdues · retournez relever la balise connue";
        }
    }
    /// Radio de proximité : aucune ligne de visée requise, y compris depuis un abri construit.
    pub fn scan(&mut self, feet: DVec3) -> Result<(), &'static str> {
        if self.carrying {
            return Err("Rapportez d’abord les données au premier abri");
        }
        let site = self
            .sites
            .get(self.completed)
            .ok_or("Trois expéditions terminées")?;
        if !feet.is_finite() || site.position_m.distance(feet) > SCAN_RANGE_M {
            return Err("Approchez la balise active à moins de 3 m");
        }
        self.discovered[self.completed] = true;
        self.carrying = true;
        Ok(())
    }
    /// Tout le lot tient dans le porté avant la première mutation ; aucun paiement partiel.
    pub fn report(&mut self, inventory: &mut Inventory) -> Result<(), &'static str> {
        if !self.carrying {
            return Err("Aucune donnée à rapporter");
        }
        let (alloy, circuits) = *REWARDS
            .get(self.completed)
            .ok_or("Trois expéditions terminées")?;
        let mut candidate = inventory.clone();
        candidate.add(Item::Alloy, alloy)?;
        candidate.add(Item::Circuit, circuits)?;
        *inventory = candidate;
        self.carrying = false;
        self.completed += 1;
        Ok(())
    }
    pub fn target(&self, buildings: &Buildings) -> Option<DVec3> {
        if self.carrying || self.completed == SITE_COUNT {
            buildings.shelters.first().map(|b| b.position_m)
        } else {
            self.sites.get(self.completed).map(|s| s.position_m)
        }
    }
}
pub struct ExpeditionPlugin;
impl Plugin for ExpeditionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Expedition>().add_systems(
            Update,
            update
                .after(crate::survival::update)
                .after(crate::construction::update)
                .in_set(GameSet::Navigation),
        );
    }
}
#[expect(
    clippy::too_many_arguments,
    reason = "Objectifs après navigation/survie ; entrées et propriétaires distincts."
)]
fn update(
    mut commands: Commands,
    keys: Res<crate::controls::GameplayKeys>,
    menu: Option<Res<MenuState>>,
    pilot: Res<Explorer>,
    state: Res<SurfaceState>,
    terrain: Res<MarsTerrain>,
    buildings: Res<Buildings>,
    oxygen: Res<Oxygen>,
    screen: Res<InventoryScreen>,
    mut c: ResMut<Construction>,
    mut e: ResMut<Expedition>,
) {
    if !oxygen.enabled {
        return;
    }
    if oxygen.failed || pilot.mode != MovementMode::Surface {
        e.abandon();
        return;
    }
    e.seed_site(&terrain, &buildings);
    if e.sites.is_empty()
        || menu.is_some_and(|m| m.is_open())
        || screen.open
        || c.active
        || c.catalog_open
        || !keys.just_pressed(KeyCode::KeyF)
    {
        return;
    }
    let feet = terrain.body_relative(state.feet);
    let result = if e.carrying {
        if buildings
            .shelters
            .first()
            .is_some_and(|b| b.contains(feet, terrain.orientation))
        {
            e.report(&mut c.inventory)
        } else {
            Err("Entrez dans le premier abri pour transmettre avec F")
        }
    } else {
        e.scan(feet)
    };
    commands.trigger(match result {
        Err(_) => crate::audio::SoundCue::Refused,
        Ok(()) if e.carrying => crate::audio::SoundCue::Survey,
        Ok(()) => crate::audio::SoundCue::Report,
    });
    e.message = match result {
        Err(reason) => reason,
        Ok(()) if e.carrying => {
            "Relevé acquis · rentrez au premier abri puis F · données perdues si échec ou sortie spatiale"
        }
        Ok(()) if e.completed == SITE_COUNT => {
            "Trois rapports validés · récompense reçue · exploration libre"
        }
        Ok(()) => "Rapport validé · récompense reçue · nouvelle balise débloquée",
    };
}
