//! Base P9 : équipements corporels f64, réserves SI et sources renouvelables bornées.
use crate::{
    buildings::{BuildingId, Buildings},
    inventory::{Inventory, Item},
    physics::surface::BodyBox,
    planets::terrain::MarsTerrain,
    rendering::atmosphere::{self, Weather},
    universe::{SimulationClock, catalog::SolarSystem, celestial_body::BodyId},
};
use bevy::{
    math::{DQuat, DVec3},
    prelude::*,
};

pub const BASE_RANGE_M: f64 = 40.0;
pub const ACCESS_RANGE_M: f64 = 2.5;
pub const MAX_FACILITIES: usize = 64;
pub const BATTERY_J: f64 = 4_800_000.0;
pub const TANK_SECONDS: f64 = 3600.0;
pub const BOOTSTRAP_O2: f64 = 720.0;
pub const PANEL_W: f64 = 160.0;
pub const GENERATOR_W: f64 = 40.0;
pub const GENERATOR_O2_PER_S: f64 = 4.0;
pub const RENEW_SECONDS: f64 = 30.0;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BuildKind {
    #[default]
    Shelter,
    Chest,
    Panel,
    Battery,
    Generator,
    Tank,
}
impl BuildKind {
    pub const ALL: [Self; 6] = [
        Self::Shelter,
        Self::Chest,
        Self::Panel,
        Self::Battery,
        Self::Generator,
        Self::Tank,
    ];
    pub fn code(self) -> usize {
        Self::ALL.iter().position(|k| *k == self).unwrap()
    }
    pub fn from_code(code: usize) -> Option<Self> {
        Self::ALL.get(code).copied()
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::Shelter => "Abri",
            Self::Chest => "Coffre",
            Self::Panel => "Panneau solaire",
            Self::Battery => "Batterie",
            Self::Generator => "Générateur O₂",
            Self::Tank => "Réservoir O₂",
        }
    }
    pub fn recipe(self) -> &'static [(Item, u32)] {
        use Item::*;
        match self {
            Self::Shelter => &[(Alloy, 12)],
            Self::Chest => &[(Alloy, 6)],
            Self::Panel => &[(Alloy, 8), (Circuit, 4)],
            Self::Battery => &[(Alloy, 8), (Circuit, 4)],
            Self::Generator => &[(Alloy, 12), (Circuit, 6)],
            Self::Tank => &[(Alloy, 6)],
        }
    }
    pub fn recipe_label(self) -> String {
        self.recipe()
            .iter()
            .map(|(i, n)| format!("{n} {}", i.name()))
            .collect::<Vec<_>>()
            .join(" + ")
    }
    /// Géométrie fermée simple : collider et rendu identiques, pieds au bas du boîtier.
    pub fn size(self) -> DVec3 {
        match self {
            Self::Shelter => DVec3::new(6.3, 3.95, 6.3),
            Self::Chest => DVec3::new(1.4, 0.9, 1.0),
            Self::Panel => DVec3::new(2.8, 0.65, 2.0),
            Self::Battery => DVec3::new(1.0, 1.3, 0.8),
            Self::Generator => DVec3::new(1.4, 1.6, 1.0),
            Self::Tank => DVec3::new(1.0, 1.8, 1.0),
        }
    }
    pub fn description(self) -> &'static str {
        match self {
            Self::Shelter => "Recharge depuis la réserve de base à moins de 40 m du premier abri.",
            Self::Chest => "16 emplacements de 20 objets. Accès E / I à 2,5 m.",
            Self::Panel => "160 W crête. Soleil, nuages et poussière réduisent la puissance.",
            Self::Battery => "4 800 000 J (1 333 Wh). Stocke le surplus ; alimente la nuit.",
            Self::Generator => {
                "40 W → 4 secondes d’autonomie O₂ par seconde. Arrêt réserve pleine."
            }
            Self::Tank => "+3 600 secondes d’autonomie de réserve. Installation vide.",
        }
    }
}
#[derive(Clone, Debug)]
pub struct Facility {
    pub id: BuildingId,
    pub kind: BuildKind,
    pub position_m: DVec3,
    pub orientation: DQuat,
    pub inventory: Inventory,
}
impl Facility {
    pub fn solid(&self) -> BodyBox {
        BodyBox {
            center: self.position_m + self.orientation * DVec3::Y * (self.kind.size().y * 0.5),
            size: self.kind.size(),
            orientation: self.orientation,
        }
    }
}
#[derive(Clone, Debug)]
pub struct Renewable {
    pub id: u64,
    pub item: Item,
    pub position_m: DVec3,
    pub orientation: DQuat,
    pub remaining: u32,
    pub cooldown_s: f64,
}
#[derive(Resource, Clone, Debug)]
pub struct Base {
    pub initialized: bool,
    pub sources: Vec<Renewable>,
    pub energy_j: f64,
    pub oxygen_s: f64,
    pub solar_w: f64,
    pub load_w: f64,
}
impl Default for Base {
    fn default() -> Self {
        Self {
            initialized: false,
            sources: Vec::new(),
            energy_j: 0.0,
            oxygen_s: BOOTSTRAP_O2,
            solar_w: 0.0,
            load_w: 0.0,
        }
    }
}
impl Base {
    pub fn seed_site(&mut self, terrain: &MarsTerrain) {
        if self.initialized {
            return;
        }
        self.initialized = true;
        for (index, (x, z, item)) in [
            (12.0, 4.0, Item::Alloy),
            (16.0, 4.0, Item::Circuit),
            (-12.0, 4.0, Item::Alloy),
            (-16.0, 4.0, Item::Circuit),
        ]
        .into_iter()
        .enumerate()
        {
            self.sources.push(Renewable {
                id: index as u64 + 1,
                item,
                position_m: terrain.body_relative(DVec3::new(x, terrain.height(x, z), z)),
                orientation: terrain.orientation,
                remaining: 6,
                cooldown_s: 0.0,
            });
        }
    }
    pub fn nearby(&self, feet: DVec3) -> Option<usize> {
        self.sources
            .iter()
            .enumerate()
            .filter(|(_, s)| s.position_m.distance(feet) <= ACCESS_RANGE_M)
            .min_by(|(_, a), (_, b)| {
                a.position_m
                    .distance_squared(feet)
                    .total_cmp(&b.position_m.distance_squared(feet))
            })
            .map(|(i, _)| i)
    }
    pub fn harvest(
        &mut self,
        index: usize,
        inventory: &mut Inventory,
        feet: DVec3,
        eye: DVec3,
        terrain: &MarsTerrain,
    ) -> Result<u32, &'static str> {
        let s = self.sources.get_mut(index).ok_or("Source inconnue")?;
        if s.position_m.distance(feet) > ACCESS_RANGE_M
            || !accessible(
                terrain,
                eye,
                s.position_m + s.orientation * DVec3::Y * 0.4,
                None,
            )
        {
            return Err("Source hors de portée ou masquée");
        }
        if s.remaining == 0 {
            return Err("Source en renouvellement (30 secondes actives)");
        }
        let n = s.remaining;
        inventory.add(s.item, n)?;
        s.remaining = 0;
        s.cooldown_s = RENEW_SECONDS;
        Ok(n)
    }
    pub fn energy_capacity(buildings: &Buildings) -> f64 {
        buildings
            .facilities
            .iter()
            .filter(|f| f.kind == BuildKind::Battery)
            .count() as f64
            * BATTERY_J
    }
    pub fn oxygen_capacity(buildings: &Buildings) -> f64 {
        BOOTSTRAP_O2
            + buildings
                .facilities
                .iter()
                .filter(|f| f.kind == BuildKind::Tank)
                .count() as f64
                * TANK_SECONDS
    }
    /// Secondes réelles intégrales ; pas de rattrapage hors application/menu/espace/échec.
    pub fn advance(&mut self, seconds: f64, buildings: &Buildings, solar_w: f64) {
        let dt = if seconds.is_finite() {
            seconds.max(0.0)
        } else {
            0.0
        };
        for s in &mut self.sources {
            if s.remaining == 0 {
                s.cooldown_s = (s.cooldown_s - dt).max(0.0);
                if s.cooldown_s == 0.0 {
                    s.remaining = 6;
                }
            }
        }
        self.solar_w = solar_w.max(0.0);
        let required_o2 = (Self::oxygen_capacity(buildings) - self.oxygen_s).max(0.0);
        let generators = buildings
            .facilities
            .iter()
            .filter(|f| f.kind == BuildKind::Generator)
            .count() as f64;
        let requested_j =
            (generators * GENERATOR_W * dt).min(required_o2 / GENERATOR_O2_PER_S * GENERATOR_W);
        let available_j = self.energy_j + self.solar_w * dt;
        let used_j = requested_j.min(available_j);
        self.oxygen_s = (self.oxygen_s + used_j / GENERATOR_W * GENERATOR_O2_PER_S)
            .min(Self::oxygen_capacity(buildings));
        self.energy_j = (available_j - used_j).clamp(0.0, Self::energy_capacity(buildings));
        self.load_w = if dt > 0.0 { used_j / dt } else { 0.0 };
    }
    pub fn solar_power(
        buildings: &Buildings,
        terrain: &MarsTerrain,
        solar: &SolarSystem,
        clock: &SimulationClock,
    ) -> f64 {
        let mars = solar.body(BodyId::MARS);
        let rotation = mars.rotation(clock.seconds_since_epoch());
        let center = solar.position(BodyId::MARS, clock.jd);
        buildings
            .facilities
            .iter()
            .filter(|f| f.kind == BuildKind::Panel)
            .map(|f| {
                let sun_body = rotation.inverse() * (-center) - f.position_m;
                let incidence = (f.orientation * DVec3::Y).dot(sun_body.normalize());
                let weather = atmosphere::sample(
                    clock.seconds_since_epoch(),
                    mars.rotation_period_s,
                    f.position_m,
                    terrain.seed,
                );
                panel_power(incidence, weather)
            })
            .sum()
    }
    pub fn refill(&mut self, requested: f64) -> f64 {
        let n = requested.max(0.0).min(self.oxygen_s);
        self.oxygen_s -= n;
        n
    }
    pub fn connected(buildings: &Buildings, id: BuildingId) -> bool {
        buildings
            .shelters
            .first()
            .zip(buildings.shelters.iter().find(|b| b.id == id))
            .is_some_and(|(home, b)| home.position_m.distance(b.position_m) <= BASE_RANGE_M)
    }
}
/// Convention de gameplay électrique, indépendante des lux/couleurs de présentation P8.
pub fn panel_power(incidence: f64, weather: Weather) -> f64 {
    PANEL_W
        * incidence.clamp(0.0, 1.0)
        * (-f64::from(weather.dust) * 0.65).exp()
        * (1.0 - f64::from(weather.clouds) * 0.25)
}
/// Accès par segment : on ignore uniquement le volume du coffre consulté.
pub fn accessible(terrain: &MarsTerrain, eye: DVec3, target: DVec3, own: Option<BodyBox>) -> bool {
    let clear = |b: &BodyBox| {
        own.as_ref().is_some_and(|o| {
            b.center == o.center && b.size == o.size && b.orientation == o.orientation
        }) || !b.blocks_segment(eye, target)
    };
    terrain.building_solids.iter().all(clear)
        && (0..3).all(|i| {
            let (p, size, q) = crate::physics::surface::obstacle_pose(terrain, i);
            clear(&BodyBox {
                center: terrain.body_relative(p),
                size,
                orientation: terrain.orientation * q,
            })
        })
}
pub fn chest_access(
    buildings: &Buildings,
    id: BuildingId,
    feet: DVec3,
    eye: DVec3,
    terrain: &MarsTerrain,
) -> bool {
    buildings
        .facilities
        .iter()
        .find(|f| f.id == id && f.kind == BuildKind::Chest)
        .is_some_and(|f| {
            f.position_m.distance(feet) <= ACCESS_RANGE_M
                && accessible(terrain, eye, f.solid().center, Some(f.solid()))
        })
}
