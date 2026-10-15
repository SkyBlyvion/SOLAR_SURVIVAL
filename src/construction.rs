//! Boucle P5 de session : récupération, inventaire et installation du même abri P4.
use crate::{
    GameSet,
    base::{BASE_RANGE_M, Base, BuildKind, MAX_FACILITIES},
    buildings::{BuildingId, Buildings},
    inventory::{Inventory, Item},
    physics::surface::{BodyBox, obstacle_pose},
    planets::terrain::MarsTerrain,
    player::{Explorer, MovementMode, surface::SurfaceState},
    survival::Oxygen,
    ui::inventory::InventoryScreen,
    ui::menu::MenuState,
};
use bevy::{
    math::{DQuat, DVec3},
    prelude::*,
};

pub const SHELTER_COST: u32 = 12;
pub const HARVEST_RANGE_M: f64 = 2.5;
pub const MAX_SHELTERS: usize = 32;

#[derive(Clone, Debug)]
pub struct Salvage {
    pub position_m: DVec3,
    pub orientation: DQuat,
    pub remaining: u32,
}
#[derive(Clone, Debug)]
pub struct Placement {
    pub position_m: DVec3,
    pub orientation: DQuat,
    pub verdict: Result<(), &'static str>,
}
#[derive(Resource, Clone)]
pub struct Construction {
    pub inventory: Inventory,
    pub kind: BuildKind,
    pub deposits: Vec<Salvage>,
    pub active: bool,
    pub catalog_open: bool,
    pub(crate) place_requested: bool,
    pub(crate) close_requested: bool,
    pub distance_m: f64,
    pub lateral_m: f64,
    pub angle_rad: f64,
    pub preview: Option<Placement>,
    pub message: &'static str,
    pub(crate) initialized: bool,
}
impl Default for Construction {
    fn default() -> Self {
        Self {
            inventory: Inventory::default(),
            kind: BuildKind::Shelter,
            deposits: Vec::new(),
            active: false,
            catalog_open: false,
            place_requested: false,
            close_requested: false,
            distance_m: 9.0,
            lateral_m: 0.0,
            angle_rad: 0.0,
            preview: None,
            message: "",
            initialized: false,
        }
    }
}
impl Construction {
    /// Huit caisses de récupération artistiques au premier site, sans réapparition.
    pub fn seed_site(&mut self, terrain: &MarsTerrain) {
        if self.initialized {
            return;
        }
        self.initialized = true;
        for x in [-8.0, 8.0] {
            for z in [0.0, -6.0, -12.0, -18.0] {
                let point = DVec3::new(x, terrain.height(x, z), z);
                self.deposits.push(Salvage {
                    position_m: terrain.body_relative(point),
                    orientation: terrain.orientation,
                    remaining: 6,
                });
            }
        }
    }
    pub fn nearby(&self, feet: DVec3) -> Option<usize> {
        self.deposits
            .iter()
            .enumerate()
            .filter(|(_, d)| d.remaining > 0 && d.position_m.distance(feet) <= HARVEST_RANGE_M)
            .min_by(|(_, a), (_, b)| {
                a.position_m
                    .distance_squared(feet)
                    .total_cmp(&b.position_m.distance_squared(feet))
            })
            .map(|(index, _)| index)
    }
    pub fn harvest(&mut self, terrain: &MarsTerrain, state: &SurfaceState) -> bool {
        let feet = terrain.body_relative(state.feet);
        let Some(index) = self.nearby(feet) else {
            self.message = "Approchez une caisse dorée (2,5 m)";
            return false;
        };
        let target =
            self.deposits[index].position_m + self.deposits[index].orientation * DVec3::Y * 0.4;
        let eye = terrain.body_relative(state.eye(terrain));
        let blocked = terrain
            .building_solids
            .iter()
            .any(|b| b.blocks_segment(eye, target))
            || (0..3).any(|i| {
                let (center, size, rotation) = obstacle_pose(terrain, i);
                BodyBox {
                    center: terrain.body_relative(center),
                    size,
                    orientation: terrain.orientation * rotation,
                }
                .blocks_segment(eye, target)
            });
        if blocked {
            self.message = "Caisse masquée par un mur ou un obstacle";
            return false;
        }
        let quantity = self.deposits[index].remaining;
        if let Err(reason) = self.inventory.add(Item::Alloy, quantity) {
            self.message = reason;
            return false;
        }
        self.deposits[index].remaining = 0;
        self.message = "Récupération stockée : +6 alliage";
        true
    }
    pub fn cancel(&mut self) {
        self.active = false;
        self.catalog_open = false;
        self.place_requested = false;
        self.close_requested = false;
        self.preview = None;
        self.message = "Placement annulé · aucune dépense";
    }
    pub fn candidate(
        &self,
        terrain: &MarsTerrain,
        state: &SurfaceState,
        buildings: &Buildings,
        base: &Base,
    ) -> Placement {
        let forward = state.local_rotation(terrain) * -DVec3::Z;
        let forward = DVec3::new(forward.x, 0.0, forward.z).normalize_or(-DVec3::Z);
        let right = forward.cross(DVec3::Y);
        let mut center = state.feet + forward * self.distance_m + right * self.lateral_m;
        let rotation = DQuat::from_rotation_y((-forward.x).atan2(-forward.z) + self.angle_rad);
        // Carré enveloppant toutes les orientations du bâtiment et sa porte.
        let half = if self.kind == BuildKind::Shelter {
            4.5
        } else {
            self.kind.size().xz().length() * 0.5 + 0.1
        };
        let range = terrain.construction_height_range(center, half);
        if let Some((low, high)) = range {
            center.y = if self.kind == BuildKind::Shelter {
                (low + high) * 0.5
            } else {
                high
            };
        }
        let position_m = terrain.body_relative(center);
        let orientation = terrain.orientation * rotation;
        let envelope = BodyBox {
            center: position_m
                + orientation
                    * DVec3::Y
                    * if self.kind == BuildKind::Shelter {
                        1.475
                    } else {
                        self.kind.size().y * 0.5
                    },
            size: self.kind.size(),
            orientation,
        };
        let mut verdict = Ok(());
        if range.is_none()
            || center.x.hypot(center.z) + half * 2.0_f64.sqrt() > 66.0 && terrain.globe.is_some()
        {
            verdict = Err("Terrain hors de la couverture locale");
        } else if range.is_some_and(|(low, high)| high - low > 0.4) {
            verdict = Err("Terrain trop irrégulier (variation > 0,4 m)");
        } else if self
            .kind
            .recipe()
            .iter()
            .any(|(item, n)| self.inventory.count(*item) < *n)
        {
            verdict = Err("Ressources portées insuffisantes");
        } else if self.kind == BuildKind::Shelter && buildings.shelters.len() >= MAX_SHELTERS {
            verdict = Err("Limite de session : 32 abris");
        } else if self.kind != BuildKind::Shelter && buildings.facilities.len() >= MAX_FACILITIES {
            verdict = Err("Limite : 64 équipements/coffres");
        } else if !matches!(self.kind, BuildKind::Shelter | BuildKind::Chest)
            && buildings
                .shelters
                .first()
                .is_none_or(|b| b.position_m.distance(position_m) > BASE_RANGE_M)
        {
            verdict = Err("Équipement à installer à moins de 40 m du premier abri");
        } else {
            let player = BodyBox {
                center: terrain.body_relative(state.feet + DVec3::Y * 0.9),
                size: DVec3::new(0.6, 1.8, 0.6),
                orientation: terrain.orientation,
            };
            let blocked = envelope.intersects(&player, 0.2)
                || buildings.shelters.iter().any(|b| {
                    envelope.intersects(
                        &BodyBox {
                            center: b.position_m + b.orientation * DVec3::Y * 1.475,
                            size: DVec3::new(6.3, 3.95, 6.3),
                            orientation: b.orientation,
                        },
                        0.2,
                    )
                })
                || buildings
                    .facilities
                    .iter()
                    .any(|f| envelope.intersects(&f.solid(), 0.2))
                || (0..3).any(|i| {
                    let (p, size, q) = obstacle_pose(terrain, i);
                    envelope.intersects(
                        &BodyBox {
                            center: terrain.body_relative(p),
                            size,
                            orientation: terrain.orientation * q,
                        },
                        0.2,
                    )
                })
                || base.sources.iter().any(|d| {
                    envelope.intersects(
                        &BodyBox {
                            center: d.position_m + d.orientation * DVec3::Y * 0.4,
                            size: DVec3::new(0.85, 0.8, 0.85),
                            orientation: d.orientation,
                        },
                        0.2,
                    )
                })
                || self.deposits.iter().filter(|d| d.remaining > 0).any(|d| {
                    envelope.intersects(
                        &BodyBox {
                            center: d.position_m + d.orientation * DVec3::Y * 0.4,
                            size: DVec3::new(1.0, 0.8, 1.0),
                            orientation: d.orientation,
                        },
                        0.2,
                    )
                });
            if blocked {
                verdict = Err("Volume occupé : joueur, abri, obstacle ou caisse");
            }
        }
        Placement {
            position_m,
            orientation,
            verdict,
        }
    }
    /// Revalidation immédiate avant la transaction ; aucun paiement de preview/annulation.
    pub fn confirm(
        &mut self,
        terrain: &mut MarsTerrain,
        state: &SurfaceState,
        buildings: &mut Buildings,
        base: &Base,
    ) -> Result<BuildingId, &'static str> {
        if !self.active {
            return Err("Placement inactif");
        }
        let candidate = self.candidate(terrain, state, buildings, base);
        candidate.verdict?;
        self.inventory.pay(self.kind.recipe())?;
        let id = if self.kind == BuildKind::Shelter {
            buildings.install_shelter(candidate.position_m, candidate.orientation)
        } else {
            buildings.install_facility(self.kind, candidate.position_m, candidate.orientation)
        };
        buildings.publish_colliders(terrain);
        self.active = false;
        self.preview = None;
        self.message = "Installation construite · état consultable avec I";
        Ok(id)
    }
}
pub struct ConstructionPlugin;
impl Plugin for ConstructionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Construction>()
            .init_resource::<InventoryScreen>()
            .add_systems(
                Update,
                update
                    .after(crate::camera::navigate)
                    .after(crate::survival::update)
                    .in_set(GameSet::Navigation),
            );
    }
}
#[expect(
    clippy::too_many_arguments,
    reason = "Commandes et transaction de construction ECS P5."
)]
pub(crate) fn update(
    mut commands: Commands,
    keys: Res<crate::controls::GameplayKeys>,
    time: Res<Time<Real>>,
    menu: Option<Res<MenuState>>,
    pilot: Res<Explorer>,
    state: Res<SurfaceState>,
    oxygen: Res<Oxygen>,
    mut terrain: ResMut<MarsTerrain>,
    mut buildings: ResMut<Buildings>,
    mut c: ResMut<Construction>,
    screen: Res<InventoryScreen>,
    mut base: ResMut<Base>,
) {
    if !oxygen.enabled
        || oxygen.failed
        || pilot.mode != MovementMode::Surface
        || menu.is_some_and(|m| m.is_open())
    {
        if c.active || c.catalog_open {
            c.cancel();
        }
        return;
    }
    c.seed_site(&terrain);
    if screen.open {
        return;
    }
    if keys.just_pressed(KeyCode::KeyB) {
        if c.active || c.catalog_open {
            c.cancel();
        } else {
            c.catalog_open = true;
        }
    }
    if (c.active || c.catalog_open)
        && (c.close_requested
            || keys.just_pressed(KeyCode::KeyX)
            || keys.just_pressed(KeyCode::Escape))
    {
        c.cancel();
        return;
    }
    let started = c.catalog_open && (c.place_requested || keys.just_pressed(KeyCode::Enter));
    if started {
        c.active = true;
        c.catalog_open = false;
        c.place_requested = false;
        c.distance_m = 9.0;
        c.lateral_m = 0.0;
        c.angle_rad = 0.0;
    }
    if c.catalog_open {
        return;
    }
    if !c.active && keys.just_pressed(KeyCode::KeyE) {
        let feet = terrain.body_relative(state.feet);
        if let Some(i) = base.nearby(feet) {
            let result = base.harvest(
                i,
                &mut c.inventory,
                feet,
                terrain.body_relative(state.eye(&terrain)),
                &terrain,
            );
            commands.trigger(if result.is_ok() {
                crate::audio::SoundCue::Harvest
            } else {
                crate::audio::SoundCue::Refused
            });
            c.message = match result {
                Ok(_) => "Récolte renouvelable : +6 objets · renouvellement 30 s",
                Err(reason) => reason,
            };
        } else {
            let success = c.harvest(&terrain, &state);
            commands.trigger(if success {
                crate::audio::SoundCue::Harvest
            } else {
                crate::audio::SoundCue::Refused
            });
        }
    }
    if !c.active {
        return;
    }
    let dt = time.delta_secs_f64().min(0.25);
    c.distance_m = (c.distance_m
        + (keys.pressed(KeyCode::ArrowUp) as i32 - keys.pressed(KeyCode::ArrowDown) as i32) as f64
            * 4.0
            * dt)
        .clamp(4.0, 24.0);
    c.lateral_m = (c.lateral_m
        + (keys.pressed(KeyCode::ArrowRight) as i32 - keys.pressed(KeyCode::ArrowLeft) as i32)
            as f64
            * 4.0
            * dt)
        .clamp(-16.0, 16.0);
    c.angle_rad = (c.angle_rad
        + (keys.pressed(KeyCode::KeyL) as i32 - keys.pressed(KeyCode::KeyJ) as i32) as f64 * dt)
        .rem_euclid(std::f64::consts::TAU);
    c.preview = Some(c.candidate(&terrain, &state, &buildings, &base));
    if !started && keys.just_pressed(KeyCode::Enter) {
        let result = c.confirm(&mut terrain, &state, &mut buildings, &base);
        commands.trigger(if result.is_ok() {
            crate::audio::SoundCue::Built
        } else {
            crate::audio::SoundCue::Refused
        });
        if let Err(reason) = result {
            c.message = reason;
        }
    }
}
