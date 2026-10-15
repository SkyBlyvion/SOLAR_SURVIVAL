//! P6 : aventures nommées, sessions SI/f64 et restauration transactionnelle.
pub mod profiles;
use crate::{
    GameSet,
    base::{Base, BuildKind, Facility, MAX_FACILITIES, Renewable},
    buildings::{BuildingId, Buildings, OxygenShelter},
    camera::{GameCamera, NavigationRequest},
    construction::{Construction, MAX_SHELTERS, Salvage},
    inventory::{CARRIED_SLOTS, CHEST_SLOTS, Inventory, Item, Stack},
    physics::surface::{HALF_WIDTH_M, STEP_SECONDS, SolidBox},
    planets::{globe_terrain::GlobeTerrain, terrain::MarsTerrain},
    player::{Explorer, MovementMode, surface::SurfaceState},
    survival::{CAPACITY_SECONDS, Oxygen},
    ui::menu::{MenuInputSet, MenuState, PlayerSettings},
    universe::{
        MAX_JD, MIN_JD, SimulationClock, catalog::SolarSystem, celestial_body::BodyId,
        coordinates::RenderOrigin,
    },
};
use bevy::{
    math::{DQuat, DVec3},
    prelude::*,
};
use std::{
    fmt::Write as _,
    fs::{self, File, OpenOptions},
    io::{Read, Write as _},
    path::{Path, PathBuf},
    str::FromStr,
};

const MAX_BYTES: u64 = 262_144;
type Result<T> = std::result::Result<T, String>;

#[derive(Clone)]
pub enum SaveAction {
    Save,
    Load,
    Create(String),
    Open(String),
    Refresh,
}
#[derive(Resource)]
pub struct SaveControl {
    pub root: PathBuf,
    pub active_profile: Option<String>,
    pub profiles: Vec<String>,
    pub selected: usize,
    pub draft: String,
    pub path: PathBuf,
    pub request: Option<SaveAction>,
    pub status: String,
}
impl Default for SaveControl {
    fn default() -> Self {
        Self {
            root: std::env::var_os("SOLAR_SAVE_ROOT")
                .map_or_else(|| PathBuf::from("saves"), PathBuf::from),
            active_profile: None,
            profiles: Vec::new(),
            selected: 0,
            draft: String::new(),
            path: PathBuf::new(),
            request: None,
            status: "Créez une aventure ou choisissez un profil à charger".into(),
        }
    }
}
pub struct PersistencePlugin;
impl Plugin for PersistencePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SaveControl>().add_systems(
            Update,
            process
                .after(MenuInputSet)
                .before(crate::camera::controls)
                .in_set(GameSet::Input),
        );
    }
}
fn process(world: &mut World) {
    let action = world.resource_mut::<SaveControl>().request.take();
    let Some(action) = action else {
        return;
    };
    let save_action = matches!(action, SaveAction::Save);
    let result: Result<String> = match action {
        SaveAction::Save => {
            if world.resource::<SaveControl>().active_profile.is_none() {
                Err("Aucune aventure active".into())
            } else {
                profiles::save_current(world).map(|()| "Aventure sauvegardée".into())
            }
        }
        SaveAction::Load => {
            if let Some(name) = world.resource::<SaveControl>().active_profile.clone() {
                profiles::open(world, &name)
                    .map(|_| "Aventure chargée · Reprendre pour jouer".into())
            } else {
                Err("Choisissez une aventure dans la liste".into())
            }
        }
        SaveAction::Create(name) => {
            profiles::create(world, &name).map(|()| format!("Nouvelle aventure : {name}"))
        }
        SaveAction::Open(name) => profiles::open(world, &name).map(|backup| {
            if backup {
                format!("{name} · chargée depuis le secours")
            } else {
                format!("{name} · aventure chargée")
            }
        }),
        SaveAction::Refresh => {
            profiles::refresh(world).map(|()| "Choisissez une aventure, puis Charger".into())
        }
    };
    let mut c = world.resource_mut::<SaveControl>();
    c.status = match result {
        Ok(message) => message,
        Err(error) => format!(
            "{} impossible : {error}",
            if save_action {
                "Sauvegarde"
            } else {
                "Opération"
            }
        ),
    };
}

/// Aucun handle, mesh, index ou Transform n'est sérialisé.
#[derive(Clone)]
pub struct Session {
    clock: SimulationClock,
    pilot: Explorer,
    surface: SurfaceState,
    orientation: DQuat,
    global: bool,
    ready: bool,
    seed: u64,
    obstacles: Option<(DQuat, [SolidBox; 3])>,
    buildings: Buildings,
    construction: Construction,
    oxygen: Oxygen,
    base: Base,
    settings: PlayerSettings,
}
impl Session {
    fn fresh(world: &World) -> Result<Self> {
        if !world.resource::<Oxygen>().enabled || world.resource::<MarsTerrain>().cube_enabled {
            return Err("Nouvelles aventures indisponibles dans ce diagnostic".into());
        }
        let config = world.resource::<crate::config::GameConfig>();
        let jd = config.start_jd;
        let mut pilot = Explorer::default();
        pilot.visit(world.resource::<SolarSystem>(), jd);
        let s = Self {
            clock: SimulationClock {
                jd,
                speed_index: 0,
                paused: false,
            },
            pilot,
            surface: SurfaceState::default(),
            orientation: DQuat::IDENTITY,
            global: config.mars_global,
            ready: false,
            seed: world.resource::<MarsTerrain>().seed,
            obstacles: None,
            buildings: Buildings::default(),
            construction: Construction::default(),
            oxygen: Oxygen::default(),
            base: Base::default(),
            settings: world.resource::<PlayerSettings>().clone(),
        };
        s.validate()?;
        Ok(s)
    }
    pub fn capture(world: &World) -> Result<Self> {
        let t = world.resource::<MarsTerrain>();
        if !world.resource::<Oxygen>().enabled || t.cube_enabled {
            return Err("Mode de diagnostic historique sans sauvegarde".into());
        }
        let s = Self {
            clock: world.resource::<SimulationClock>().clone(),
            pilot: world.resource::<Explorer>().clone(),
            surface: world.resource::<SurfaceState>().clone(),
            orientation: t.orientation,
            global: t.globe_enabled,
            ready: world.resource::<Buildings>().home_orientation.is_some(),
            seed: t.seed,
            obstacles: t.site_obstacles,
            buildings: world.resource::<Buildings>().clone(),
            construction: world.resource::<Construction>().clone(),
            oxygen: world.resource::<Oxygen>().clone(),
            base: world.resource::<Base>().clone(),
            settings: world.resource::<PlayerSettings>().clone(),
        };
        s.validate()?;
        Ok(s)
    }
    fn validate(&self) -> Result<()> {
        let fail = || Err("Données de session invalides ou incohérentes".into());
        let vector = |p: DVec3| p.is_finite() && p.length() < 1e16;
        let quaternion = |q: DQuat| q.is_finite() && (q.length_squared() - 1.0).abs() < 1e-9;
        if !(MIN_JD..=MAX_JD).contains(&self.clock.jd)
            || self.clock.speed_index >= 4
            || self.pilot.selected.0 >= 10
            || self.pilot.anchor.is_some_and(|a| a.0 >= 10)
            || self.pilot.speed_index >= 6
            || !vector(self.pilot.local_m)
            || !quaternion(self.pilot.rotation)
            || !quaternion(self.orientation)
            || !vector(self.surface.feet)
            || !vector(self.surface.velocity)
            || self.surface.velocity.length() > 1000.0
            || !quaternion(self.surface.heading)
            || !self.surface.yaw.is_finite()
            || !(-1.55..=1.55).contains(&self.surface.pitch)
            || !(0.0..STEP_SECONDS + 1e-12).contains(&self.surface.accumulator)
            || !(0.0..=CAPACITY_SECONDS).contains(&self.oxygen.remaining_seconds)
            || self.oxygen.failed != (self.oxygen.remaining_seconds == 0.0)
            || self.settings.sensitivity_index >= 4
            || self.settings.fov_index >= 4
            || self.settings.msaa_index >= 3
        {
            return fail();
        }
        if self.pilot.mode == MovementMode::Surface
            && (!self.ready || self.pilot.anchor != Some(BodyId::MARS))
        {
            return fail();
        }
        if self.buildings.shelters.len() > MAX_SHELTERS || self.construction.deposits.len() > 8 {
            return fail();
        }
        let radius = SolarSystem::default().body(BodyId::MARS).radius_m;
        let pose = |p: DVec3, q: DQuat| {
            vector(p)
                && (p.length() - radius).abs() < crate::planets::relief::MAX_HEIGHT_M + 2.0
                && quaternion(q)
        };
        let mut ids = std::collections::HashSet::new();
        for b in &self.buildings.shelters {
            if b.id.0 == 0
                || b.id.0 > self.buildings.next_id
                || !ids.insert(b.id)
                || !pose(b.position_m, b.orientation)
            {
                return fail();
            }
        }
        if self.buildings.facilities.len() > MAX_FACILITIES
            || !self.construction.inventory.valid(CARRIED_SLOTS)
        {
            return fail();
        }
        for f in &self.buildings.facilities {
            if f.kind == BuildKind::Shelter
                || f.id.0 == 0
                || f.id.0 > self.buildings.next_id
                || !ids.insert(f.id)
                || !pose(f.position_m, f.orientation)
                || !f.inventory.valid(if f.kind == BuildKind::Chest {
                    CHEST_SLOTS
                } else {
                    0
                })
                || !matches!(f.kind, BuildKind::Chest)
                    && self.buildings.shelters.first().is_none_or(|b| {
                        b.position_m.distance(f.position_m) > crate::base::BASE_RANGE_M + 1e-7
                    })
            {
                return fail();
            }
        }
        if self.buildings.next_id != ids.len() as u64
            || (1..=self.buildings.next_id).any(|i| !ids.contains(&BuildingId(i)))
        {
            return fail();
        }
        if !self.base.energy_j.is_finite()
            || !(0.0..=Base::energy_capacity(&self.buildings)).contains(&self.base.energy_j)
            || !(0.0..=Base::oxygen_capacity(&self.buildings)).contains(&self.base.oxygen_s)
            || self.base.initialized != self.ready
            || self.base.sources.len() != if self.ready { 4 } else { 0 }
        {
            return fail();
        }
        for (index, source) in self.base.sources.iter().enumerate() {
            if source.id != index as u64 + 1
                || source.item
                    != if index % 2 == 0 {
                        Item::Alloy
                    } else {
                        Item::Circuit
                    }
                || !pose(source.position_m, source.orientation)
                || ![0, 6].contains(&source.remaining)
                || !(0.0..=crate::base::RENEW_SECONDS).contains(&source.cooldown_s)
                || (source.remaining == 0) != (source.cooldown_s > 0.0)
            {
                return fail();
            }
        }
        if let Some(q) = self.buildings.home_orientation {
            if !quaternion(q)
                || !self.ready
                || self
                    .buildings
                    .shelters
                    .first()
                    .is_none_or(|b| b.id != BuildingId(1))
                || !self.construction.initialized
                || self.construction.deposits.len() != 8
            {
                return fail();
            }
            for d in &self.construction.deposits {
                if !pose(d.position_m, d.orientation) || ![0, 6].contains(&d.remaining) {
                    return fail();
                }
            }
        } else if !self.buildings.shelters.is_empty()
            || !self.construction.deposits.is_empty()
            || self.construction.initialized
            || self.construction.inventory.used() != 0
            || !self.buildings.facilities.is_empty()
            || self.pilot.mode == MovementMode::Surface
        {
            return fail();
        }
        if let Some((q, solids)) = self.obstacles {
            if !quaternion(q) || !self.ready {
                return fail();
            }
            for s in solids {
                if !vector(s.min)
                    || !vector(s.max)
                    || !s.min.cmplt(s.max).all()
                    || s.min.abs().max_element() > 120.0
                    || s.max.abs().max_element() > 120.0
                {
                    return fail();
                }
            }
        }
        if self.ready && self.global && self.obstacles.is_none() {
            return fail();
        }
        Ok(())
    }
    pub fn encode(&self) -> String {
        let mut text = String::from("SOLAR_SURVIVAL 3\n");
        writeln!(
            text,
            "clock {} {} {}",
            self.clock.jd, self.clock.speed_index, self.clock.paused
        )
        .unwrap();
        writeln!(
            text,
            "pilot {} {} {} {} {} {}",
            if self.pilot.mode == MovementMode::Surface {
                "surface"
            } else {
                "space"
            },
            self.pilot.anchor.map_or(10, |b| b.0),
            v(self.pilot.local_m),
            q(self.pilot.rotation),
            self.pilot.selected.0,
            self.pilot.speed_index
        )
        .unwrap();
        writeln!(
            text,
            "terrain {} {} {} {}",
            self.global,
            self.ready,
            self.seed,
            q(self.orientation)
        )
        .unwrap();
        writeln!(text, "obstacles {}", self.obstacles.is_some()).unwrap();
        if let Some((rotation, solids)) = self.obstacles {
            writeln!(text, "frame {}", q(rotation)).unwrap();
            for solid in solids {
                writeln!(text, "solid {} {}", v(solid.min), v(solid.max)).unwrap();
            }
        }
        writeln!(
            text,
            "home {} {}",
            self.buildings.home_orientation.is_some(),
            q(self.buildings.home_orientation.unwrap_or(DQuat::IDENTITY))
        )
        .unwrap();
        writeln!(
            text,
            "buildings {} {}",
            self.buildings.shelters.len(),
            self.buildings.next_id
        )
        .unwrap();
        for b in &self.buildings.shelters {
            writeln!(
                text,
                "building {} {} {}",
                b.id.0,
                v(b.position_m),
                q(b.orientation)
            )
            .unwrap();
        }
        writeln!(text, "facilities {}", self.buildings.facilities.len()).unwrap();
        for f in &self.buildings.facilities {
            writeln!(
                text,
                "facility {} {} {} {}",
                f.id.0,
                f.kind.code(),
                v(f.position_m),
                q(f.orientation)
            )
            .unwrap();
            write_inventory(&mut text, &f.inventory);
        }
        writeln!(
            text,
            "base {} {} {} {}",
            self.base.initialized,
            self.base.energy_j,
            self.base.oxygen_s,
            self.base.sources.len()
        )
        .unwrap();
        for s in &self.base.sources {
            writeln!(
                text,
                "source {} {} {} {} {} {}",
                s.id,
                s.item.code(),
                v(s.position_m),
                q(s.orientation),
                s.remaining,
                s.cooldown_s
            )
            .unwrap();
        }
        writeln!(
            text,
            "inventory {} {}",
            self.construction.initialized,
            self.construction.deposits.len()
        )
        .unwrap();
        write_inventory(&mut text, &self.construction.inventory);
        for d in &self.construction.deposits {
            writeln!(
                text,
                "deposit {} {} {}",
                v(d.position_m),
                q(d.orientation),
                d.remaining
            )
            .unwrap();
        }
        writeln!(
            text,
            "oxygen {} {}",
            self.oxygen.remaining_seconds, self.oxygen.failed
        )
        .unwrap();
        writeln!(
            text,
            "surface {} {} {} {} {} {} {}",
            v(self.surface.feet),
            v(self.surface.velocity),
            self.surface.grounded,
            self.surface.yaw,
            self.surface.pitch,
            q(self.surface.heading),
            self.surface.accumulator
        )
        .unwrap();
        writeln!(
            text,
            "settings {} {} {} {}",
            self.settings.sensitivity_index,
            self.settings.fov_index,
            self.settings.msaa_index,
            self.settings.fullscreen
        )
        .unwrap();
        let sum = checksum(text.as_bytes());
        writeln!(text, "checksum {sum:016x}").unwrap();
        text
    }
    pub fn decode(text: &str) -> Result<Self> {
        if text.len() as u64 > MAX_BYTES {
            return Err("Fichier trop volumineux".into());
        }
        let (payload, sum) = text
            .rsplit_once("checksum ")
            .ok_or("Fichier incomplet : contrôle absent")?;
        if sum.trim() != format!("{:016x}", checksum(payload.as_bytes())) {
            return Err("Fichier corrompu : contrôle d'intégrité".into());
        }
        let mut r = Reader(payload.split_whitespace());
        r.tag("SOLAR_SURVIVAL")?;
        if r.num::<u32>()? != 3 {
            return Err("Version de sauvegarde incompatible".into());
        }
        r.tag("clock")?;
        let clock = SimulationClock {
            jd: r.num()?,
            speed_index: r.num()?,
            paused: r.num()?,
        };
        r.tag("pilot")?;
        let mode = match r.token()? {
            "space" => MovementMode::Space,
            "surface" => MovementMode::Surface,
            _ => return Err("Mode joueur invalide".into()),
        };
        let anchor: usize = r.num()?;
        if anchor > 10 {
            return Err("Astre suivi invalide".into());
        }
        let pilot = Explorer {
            mode,
            anchor: (anchor < 10).then_some(BodyId(anchor)),
            local_m: r.vec()?,
            rotation: r.quat()?,
            selected: BodyId(r.num()?),
            speed_index: r.num()?,
            boosted: false,
            captured: false,
        };
        r.tag("terrain")?;
        let global = r.num()?;
        let ready = r.num()?;
        let seed = r.num()?;
        let orientation = r.quat()?;
        r.tag("obstacles")?;
        let obstacles = if r.num::<bool>()? {
            r.tag("frame")?;
            let rotation = r.quat()?;
            let mut solids = [SolidBox {
                min: DVec3::ZERO,
                max: DVec3::ZERO,
            }; 3];
            for s in &mut solids {
                r.tag("solid")?;
                *s = SolidBox {
                    min: r.vec()?,
                    max: r.vec()?,
                };
            }
            Some((rotation, solids))
        } else {
            None
        };
        r.tag("home")?;
        let has_home = r.num::<bool>()?;
        let home = r.quat()?;
        r.tag("buildings")?;
        let n = r.count(MAX_SHELTERS)?;
        let next_id = r.num()?;
        let mut shelters = Vec::with_capacity(n);
        for _ in 0..n {
            r.tag("building")?;
            shelters.push(OxygenShelter {
                id: BuildingId(r.num()?),
                position_m: r.vec()?,
                orientation: r.quat()?,
            });
        }
        r.tag("facilities")?;
        let n = r.count(MAX_FACILITIES)?;
        let mut facilities = Vec::with_capacity(n);
        for _ in 0..n {
            r.tag("facility")?;
            let id = BuildingId(r.num()?);
            let kind = BuildKind::from_code(r.num()?).ok_or("Type d'équipement inconnu")?;
            let position_m = r.vec()?;
            let orientation = r.quat()?;
            let inventory = read_inventory(
                &mut r,
                if kind == BuildKind::Chest {
                    CHEST_SLOTS
                } else {
                    0
                },
            )?;
            facilities.push(Facility {
                id,
                kind,
                position_m,
                orientation,
                inventory,
            });
        }
        let buildings = Buildings {
            shelters,
            facilities,
            next_id,
            home_orientation: has_home.then_some(home),
        };
        r.tag("base")?;
        let mut base = Base {
            initialized: r.num()?,
            energy_j: r.num()?,
            oxygen_s: r.num()?,
            ..default()
        };
        let n = r.count(4)?;
        for _ in 0..n {
            r.tag("source")?;
            base.sources.push(Renewable {
                id: r.num()?,
                item: Item::from_code(r.num()?).ok_or("Ressource inconnue")?,
                position_m: r.vec()?,
                orientation: r.quat()?,
                remaining: r.num()?,
                cooldown_s: r.num()?,
            });
        }
        r.tag("inventory")?;
        let initialized = r.num()?;
        let n = r.count(8)?;
        let mut construction = Construction {
            initialized,
            inventory: read_inventory(&mut r, CARRIED_SLOTS)?,
            ..default()
        };
        for _ in 0..n {
            r.tag("deposit")?;
            construction.deposits.push(Salvage {
                position_m: r.vec()?,
                orientation: r.quat()?,
                remaining: r.num()?,
            });
        }
        r.tag("oxygen")?;
        let oxygen = Oxygen {
            enabled: true,
            remaining_seconds: r.num()?,
            failed: r.num()?,
            shelter: None,
        };
        r.tag("surface")?;
        let surface = SurfaceState {
            feet: r.vec()?,
            velocity: r.vec()?,
            grounded: r.num()?,
            yaw: r.num()?,
            pitch: r.num()?,
            heading: r.quat()?,
            accumulator: r.num()?,
            jump_pending: false,
        };
        r.tag("settings")?;
        let settings = PlayerSettings {
            sensitivity_index: r.num()?,
            fov_index: r.num()?,
            msaa_index: r.num()?,
            fullscreen: r.num()?,
        };
        if r.0.next().is_some() {
            return Err("Données supplémentaires inattendues".into());
        }
        let s = Self {
            clock,
            pilot,
            surface,
            orientation,
            global,
            ready,
            seed,
            obstacles,
            buildings,
            construction,
            oxygen,
            base,
            settings,
        };
        s.validate()?;
        Ok(s)
    }
    /// Assertions opt-in du diagnostic à deux processus ; aucune action en partie normale.
    pub(crate) fn verify_restored(&self, world: &World) {
        let actual = Self::capture(world).unwrap();
        assert_eq!(self.clock.jd, actual.clock.jd);
        assert_eq!(self.orientation, actual.orientation);
        assert_eq!(self.pilot.mode, actual.pilot.mode);
        assert_eq!(
            self.buildings.home_orientation,
            actual.buildings.home_orientation
        );
        assert_eq!(self.buildings.next_id, actual.buildings.next_id);
        assert_eq!(
            self.buildings.shelters.len(),
            actual.buildings.shelters.len()
        );
        for (a, b) in self
            .buildings
            .shelters
            .iter()
            .zip(&actual.buildings.shelters)
        {
            assert_eq!(a.id, b.id);
            assert_eq!(a.position_m, b.position_m);
            assert_eq!(a.orientation, b.orientation);
        }
        for (a, b) in self
            .construction
            .deposits
            .iter()
            .zip(&actual.construction.deposits)
        {
            assert_eq!(a.position_m, b.position_m);
            assert_eq!(a.orientation, b.orientation);
            assert_eq!(a.remaining, b.remaining);
        }
        assert_eq!(
            self.construction
                .inventory
                .count(crate::inventory::Item::Alloy),
            actual
                .construction
                .inventory
                .count(crate::inventory::Item::Alloy)
        );
        assert_eq!(
            self.oxygen.remaining_seconds,
            actual.oxygen.remaining_seconds
        );
        assert_eq!(self.base.energy_j, actual.base.energy_j);
        assert_eq!(self.base.oxygen_s, actual.base.oxygen_s);
        assert_eq!(self.construction.inventory, actual.construction.inventory);
        assert_eq!(
            self.buildings.facilities.len(),
            actual.buildings.facilities.len()
        );
        for (a, b) in self
            .buildings
            .facilities
            .iter()
            .zip(&actual.buildings.facilities)
        {
            assert_eq!(
                (a.id, a.kind, a.position_m, a.orientation),
                (b.id, b.kind, b.position_m, b.orientation)
            );
            assert_eq!(a.inventory, b.inventory);
        }
        for (a, b) in self.base.sources.iter().zip(&actual.base.sources) {
            assert_eq!(
                (a.id, a.remaining, a.cooldown_s),
                (b.id, b.remaining, b.cooldown_s)
            );
        }
        assert_eq!(self.oxygen.failed, actual.oxygen.failed);
        assert!(self.surface.feet.distance(actual.surface.feet) < 1e-7);
        assert!(self.surface.velocity.distance(actual.surface.velocity) < 1e-7);
        assert_eq!(self.surface.heading, actual.surface.heading);
        assert_eq!(self.surface.yaw, actual.surface.yaw);
        assert_eq!(self.surface.pitch, actual.surface.pitch);
        assert_eq!(self.settings.fov_index, actual.settings.fov_index);
        assert_eq!(self.settings.msaa_index, actual.settings.msaa_index);
        assert_eq!(
            self.settings.sensitivity_index,
            actual.settings.sensitivity_index
        );
        assert_eq!(self.settings.fullscreen, actual.settings.fullscreen);
    }
    /// Construire et vérifier les caches AVANT de remplacer le moindre propriétaire.
    pub fn restore(mut self, world: &mut World) -> Result<()> {
        self.validate()?;
        if !world.resource::<Oxygen>().enabled || world.resource::<MarsTerrain>().cube_enabled {
            return Err("Mode de diagnostic historique sans chargement".into());
        }
        let mut terrain = MarsTerrain::from_world(world);
        if self.seed != terrain.seed {
            return Err("Génération du terrain incompatible".into());
        }
        terrain.cube_enabled = false;
        terrain.migration_enabled = false;
        terrain.offcenter_enabled = false;
        terrain.globe_enabled = self.global;
        terrain.orientation = self.orientation;
        terrain.site_obstacles = self.obstacles;
        terrain.revision = world.resource::<MarsTerrain>().revision.wrapping_add(1);
        if self.ready && self.global {
            terrain.globe = Some(GlobeTerrain::with_relief(
                self.orientation * DVec3::Y,
                self.orientation,
                terrain.radius_m,
                terrain.seed,
            ));
        }
        if self.pilot.mode == MovementMode::Surface {
            let f = self.surface.feet;
            if !(-crate::planets::relief::MAX_HEIGHT_M - 1.0..=1000.0).contains(&f.y)
                || if self.global {
                    f.x.hypot(f.z) > 64.1
                } else {
                    f.x.abs().max(f.z.abs()) > 4096.0 - HALF_WIDTH_M
                }
            {
                return Err("Pose hors du terrain sauvegardé".into());
            }
            let support = if let Some(g) = &terrain.globe {
                g.support_height(f, HALF_WIDTH_M)
                    .ok_or("Empreinte non couverte")?
            } else {
                terrain.support_height(f, HALF_WIDTH_M)
            };
            if f.y < support - 0.002 {
                return Err("Joueur sous le terrain".into());
            }
            self.surface.sync(
                &mut self.pilot,
                &terrain,
                world
                    .resource::<SolarSystem>()
                    .body(BodyId::MARS)
                    .rotation(self.clock.seconds_since_epoch()),
            );
        }
        self.buildings.publish_colliders(&mut terrain);
        self.oxygen.shelter = if self.pilot.mode == MovementMode::Surface {
            self.buildings.shelter_at(
                terrain.body_relative(self.surface.feet),
                terrain.orientation,
            )
        } else {
            None
        };
        self.construction.catalog_open = false;
        self.construction.place_requested = false;
        self.construction.close_requested = false;
        self.construction.active = false;
        self.construction.preview = None;
        self.construction.message = "Session chargée · inventaire et bâtiments restaurés";
        self.pilot.captured = false;
        self.pilot.boosted = false;
        self.surface.jump_pending = false;
        let rotation = if self.pilot.mode == MovementMode::Surface {
            self.surface.local_rotation(&terrain)
        } else {
            self.pilot.rotation
        };
        let origin = self
            .pilot
            .global(world.resource::<SolarSystem>(), self.clock.jd);
        world.insert_resource(terrain);
        world.insert_resource(self.clock);
        world.insert_resource(self.pilot);
        world.insert_resource(self.surface);
        world.insert_resource(self.buildings);
        world.insert_resource(self.construction);
        world.insert_resource(self.oxygen);
        world.insert_resource(self.base);
        world.insert_resource(crate::ui::inventory::InventoryScreen::default());
        world.insert_resource(self.settings);
        world.insert_resource(RenderOrigin(origin));
        world.insert_resource(NavigationRequest::default());
        if let Some(mut menu) = world.get_resource_mut::<MenuState>() {
            menu.loaded();
        }
        if let Some(mut keys) = world.get_resource_mut::<ButtonInput<KeyCode>>() {
            keys.reset_all();
        }
        if let Some(mut mouse) = world.get_resource_mut::<ButtonInput<MouseButton>>() {
            mouse.reset_all();
        }
        for mut transform in world
            .query_filtered::<&mut Transform, With<GameCamera>>()
            .iter_mut(world)
        {
            transform.translation = Vec3::ZERO;
            transform.rotation = rotation.as_quat();
        }
        Ok(())
    }
}
fn write_inventory(text: &mut String, inventory: &Inventory) {
    writeln!(text, "slots {}", inventory.slots.len()).unwrap();
    for slot in &inventory.slots {
        let (item, n) = slot
            .as_ref()
            .map_or((2, 0), |s| (s.item.code(), s.quantity));
        writeln!(text, "slot {item} {n}").unwrap();
    }
}
fn read_inventory(r: &mut Reader<'_>, capacity: usize) -> Result<Inventory> {
    r.tag("slots")?;
    if r.count(CHEST_SLOTS)? != capacity {
        return Err("Capacité d'inventaire incohérente".into());
    }
    let mut inventory = Inventory::new(capacity);
    for slot in &mut inventory.slots {
        r.tag("slot")?;
        let code = r.num()?;
        let quantity = r.num()?;
        *slot = if code == 2 && quantity == 0 {
            None
        } else {
            Some(Stack {
                item: Item::from_code(code).ok_or("Type d'objet inconnu")?,
                quantity,
            })
        };
    }
    if !inventory.valid(capacity) {
        return Err("Pile invalide".into());
    }
    Ok(inventory)
}
fn v(p: DVec3) -> String {
    format!("{} {} {}", p.x, p.y, p.z)
}
fn q(p: DQuat) -> String {
    format!("{} {} {} {}", p.x, p.y, p.z, p.w)
}
fn checksum(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325u64, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(0x100000001b3)
    })
}
struct Reader<'a>(std::str::SplitWhitespace<'a>);
impl<'a> Reader<'a> {
    fn token(&mut self) -> Result<&'a str> {
        self.0.next().ok_or_else(|| "Fichier incomplet".into())
    }
    fn tag(&mut self, expected: &str) -> Result<()> {
        if self.token()? == expected {
            Ok(())
        } else {
            Err(format!("Champ attendu : {expected}"))
        }
    }
    fn num<T: FromStr>(&mut self) -> Result<T> {
        self.token()?.parse().map_err(|_| "Valeur invalide".into())
    }
    fn vec(&mut self) -> Result<DVec3> {
        Ok(DVec3::new(self.num()?, self.num()?, self.num()?))
    }
    fn quat(&mut self) -> Result<DQuat> {
        Ok(DQuat::from_xyzw(
            self.num()?,
            self.num()?,
            self.num()?,
            self.num()?,
        ))
    }
    fn count(&mut self, max: usize) -> Result<usize> {
        let n = self.num()?;
        if n <= max {
            Ok(n)
        } else {
            Err("Collection trop volumineuse".into())
        }
    }
}
fn sibling(path: &Path, suffix: &str) -> PathBuf {
    let mut name = path.as_os_str().to_os_string();
    name.push(suffix);
    PathBuf::from(name)
}
/// Publication portable : fichier temporaire synchronisé, ancien fichier conservé en .bak.
/// Si interruption entre les deux renommages, load retrouve .bak ; aucune suppression de l'unique copie.
pub fn save(world: &World, path: &Path) -> Result<()> {
    write_session(&Session::capture(world)?, path)
}
fn write_session(session: &Session, path: &Path) -> Result<()> {
    let text = session.encode();
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    fs::create_dir_all(parent).map_err(|e| format!("Créer le dossier : {e}"))?;
    let tmp = sibling(path, &format!(".tmp-{}", std::process::id()));
    let backup = sibling(path, ".bak");
    if path.exists() && !path.is_file() {
        return Err("Le chemin de sauvegarde ne désigne pas un fichier".into());
    }
    let mut created = false;
    let result = (|| -> std::io::Result<()> {
        let mut file = OpenOptions::new().write(true).create_new(true).open(&tmp)?;
        created = true;
        file.write_all(text.as_bytes())?;
        file.sync_all()?;
        drop(file);
        if path.exists() {
            if backup.exists() {
                fs::remove_file(&backup)?;
            }
            fs::rename(path, &backup)?;
        }
        if let Err(error) = fs::rename(&tmp, path) {
            if !path.exists() && backup.exists() {
                let _ = fs::rename(&backup, path);
            }
            return Err(error);
        }
        Ok(())
    })();
    if result.is_err() && created {
        let _ = fs::remove_file(&tmp);
    }
    result.map_err(|e| format!("Écriture : {e}"))
}
pub fn load(world: &mut World, path: &Path) -> Result<bool> {
    let (file, backup) = match File::open(path) {
        Ok(file) => (file, false),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (
            File::open(sibling(path, ".bak"))
                .map_err(|_| "Aucune sauvegarde locale".to_string())?,
            true,
        ),
        Err(e) => return Err(format!("Lecture : {e}")),
    };
    let mut text = String::new();
    file.take(MAX_BYTES + 1)
        .read_to_string(&mut text)
        .map_err(|e| format!("Lecture : {e}"))?;
    Session::decode(&text)?.restore(world)?;
    Ok(backup)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::{
        input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
        time::TimeUpdateStrategy,
        window::{CursorOptions, PrimaryWindow},
    };
    use std::{
        sync::atomic::{AtomicU64, Ordering},
        time::Duration,
    };
    struct Folder(PathBuf);
    impl Folder {
        fn new() -> Self {
            static NEXT: AtomicU64 = AtomicU64::new(0);
            let p = std::env::temp_dir().join(format!(
                "solar-p6-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&p).unwrap();
            Self(p)
        }
        fn file(&self) -> PathBuf {
            self.0.join("session.solar")
        }
    }
    impl Drop for Folder {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }
    fn app(global: bool) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(crate::config::GameConfig {
                mars_global: global,
                ..default()
            })
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(
                1.0 / 60.0,
            )))
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<ButtonInput<MouseButton>>()
            .init_resource::<AccumulatedMouseMotion>()
            .init_resource::<AccumulatedMouseScroll>()
            .init_resource::<PlayerSettings>()
            .configure_sets(
                Update,
                (GameSet::Input, GameSet::Simulation, GameSet::Navigation).chain(),
            )
            .add_plugins((
                crate::universe::UniversePlugin,
                crate::camera::ExplorationPlugin,
                crate::survival::SurvivalPlugin,
                crate::construction::ConstructionPlugin,
            ));
        app.world_mut().spawn((
            Window {
                focused: true,
                ..default()
            },
            CursorOptions::default(),
            PrimaryWindow,
        ));
        app.update();
        app.world_mut().resource_mut::<SimulationClock>().paused = true;
        app
    }
    fn press(app: &mut App, key: KeyCode) {
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(key);
        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .reset_all();
    }
    fn prepare(app: &mut App, x: f64, z: f64) {
        let y = app
            .world()
            .resource::<MarsTerrain>()
            .support_height(DVec3::new(x, 0.0, z), HALF_WIDTH_M)
            + 0.002;
        let mut state = app.world_mut().resource_mut::<SurfaceState>();
        state.feet = DVec3::new(x, y, z);
        state.velocity = DVec3::ZERO;
        state.yaw = 0.0;
    }
    fn built() -> App {
        let mut app = app(true);
        press(&mut app, KeyCode::KeyM);
        for z in [0.0, -6.0] {
            prepare(&mut app, 8.0, z);
            press(&mut app, KeyCode::KeyE);
        }
        prepare(&mut app, 16.0, 0.0);
        press(&mut app, KeyCode::KeyB);
        press(&mut app, KeyCode::Enter);
        press(&mut app, KeyCode::Enter);
        assert_eq!(app.world().resource::<Buildings>().shelters.len(), 2);
        app
    }
    fn capture(app: &App) -> String {
        Session::capture(app.world()).unwrap().encode()
    }
    fn resign(text: &str) -> String {
        let payload = text.rsplit_once("checksum ").unwrap().0;
        format!("{payload}checksum {:016x}\n", checksum(payload.as_bytes()))
    }
    #[test]
    fn two_named_adventures_keep_independent_progress_across_fresh_application() {
        let folder = Folder::new();
        let root = folder.0.join("profiles");
        let mut game = app(true);
        game.init_resource::<SaveControl>();
        game.world_mut().resource_mut::<SaveControl>().root = root.clone();
        profiles::create(game.world_mut(), "Alpha").unwrap();
        press(&mut game, KeyCode::KeyM);
        for z in [0.0, -6.0] {
            prepare(&mut game, 8.0, z);
            press(&mut game, KeyCode::KeyE);
        }
        prepare(&mut game, 16.0, 0.0);
        press(&mut game, KeyCode::KeyB);
        press(&mut game, KeyCode::Enter);
        press(&mut game, KeyCode::Enter);
        game.world_mut().resource_mut::<Oxygen>().remaining_seconds = 80.0;
        let shelters = game.world().resource::<Buildings>().shelters.clone();
        profiles::create(game.world_mut(), "Bravo").unwrap();
        assert_eq!(
            game.world().resource::<Explorer>().mode,
            MovementMode::Space
        );
        assert!(game.world().resource::<Buildings>().shelters.is_empty());
        assert!(game.world().resource::<Construction>().deposits.is_empty());
        assert_eq!(
            game.world()
                .resource::<Construction>()
                .inventory
                .count(crate::inventory::Item::Alloy),
            0
        );
        assert_eq!(
            game.world().resource::<Oxygen>().remaining_seconds,
            CAPACITY_SECONDS
        );
        assert!(
            game.world()
                .resource::<Buildings>()
                .home_orientation
                .is_none()
        );
        press(&mut game, KeyCode::KeyM);
        prepare(&mut game, 8.0, 0.0);
        press(&mut game, KeyCode::KeyE);
        {
            let mut o = game.world_mut().resource_mut::<Oxygen>();
            o.remaining_seconds = 0.0;
            o.failed = true;
        }
        profiles::open(game.world_mut(), "Alpha").unwrap();
        assert_eq!(
            game.world()
                .resource::<Construction>()
                .inventory
                .count(crate::inventory::Item::Alloy),
            0
        );
        assert_eq!(game.world().resource::<Buildings>().shelters.len(), 2);
        assert_eq!(
            game.world().resource::<Buildings>().shelters[1].position_m,
            shelters[1].position_m
        );
        assert_eq!(game.world().resource::<Oxygen>().remaining_seconds, 80.0);
        assert!(!game.world().resource::<Oxygen>().failed);
        let mut fresh = app(true);
        fresh.init_resource::<SaveControl>();
        fresh.world_mut().resource_mut::<SaveControl>().root = root.clone();
        profiles::refresh(fresh.world_mut()).unwrap();
        assert_eq!(
            fresh.world().resource::<SaveControl>().profiles,
            vec!["Alpha", "Bravo"]
        );
        profiles::open(fresh.world_mut(), "Bravo").unwrap();
        assert_eq!(
            fresh
                .world()
                .resource::<Construction>()
                .inventory
                .count(crate::inventory::Item::Alloy),
            6
        );
        assert_eq!(fresh.world().resource::<Buildings>().shelters.len(), 1);
        assert!(fresh.world().resource::<Oxygen>().failed);
        assert_eq!(
            fresh
                .world()
                .resource::<Construction>()
                .deposits
                .iter()
                .filter(|d| d.remaining == 0)
                .count(),
            1
        );
        let before = capture(&fresh);
        assert!(profiles::create(fresh.world_mut(), "alpha").is_err());
        assert_eq!(capture(&fresh), before);
        fs::write(root.join("Alpha/session.solar"), "corrompu").unwrap();
        assert!(profiles::open(fresh.world_mut(), "Alpha").is_err());
        assert_eq!(
            fresh
                .world()
                .resource::<SaveControl>()
                .active_profile
                .as_deref(),
            Some("Bravo")
        );
        assert_eq!(capture(&fresh), before);
    }
    #[test]
    fn p9_profiles_restore_organized_stacks_chests_equipment_reserves_and_renewal_transactionally()
    {
        let folder = Folder::new();
        let root = folder.0.join("profiles");
        let mut source = built();
        source.init_resource::<SaveControl>();
        source.world_mut().resource_mut::<SaveControl>().root = root.clone();
        profiles::create(source.world_mut(), "Base Alpha").unwrap();
        press(&mut source, KeyCode::KeyM);
        let (position, orientation) = {
            let t = source.world().resource::<MarsTerrain>();
            (
                t.body_relative(DVec3::new(18.0, t.height(18.0, 0.0), 0.0)),
                t.orientation,
            )
        };
        for kind in [
            BuildKind::Chest,
            BuildKind::Chest,
            BuildKind::Panel,
            BuildKind::Battery,
            BuildKind::Generator,
            BuildKind::Tank,
        ] {
            let mut b = source.world_mut().resource_mut::<Buildings>();
            let id = b.install_facility(kind, position, orientation);
            if kind == BuildKind::Chest {
                let f = b.facilities.iter_mut().find(|f| f.id == id).unwrap();
                f.inventory.add(Item::Circuit, 13).unwrap();
                f.inventory.swap(0, 7).unwrap();
            }
        }
        {
            let mut c = source.world_mut().resource_mut::<Construction>();
            c.inventory.add(Item::Alloy, 27).unwrap();
            c.inventory.swap(0, 6).unwrap();
        }
        {
            let mut b = source.world_mut().resource_mut::<Base>();
            b.energy_j = 345678.0;
            b.oxygen_s = 2345.0;
            b.sources[0].remaining = 0;
            b.sources[0].cooldown_s = 17.25;
        }
        profiles::save_current(source.world_mut()).unwrap();
        let expected = capture(&source);
        profiles::create(source.world_mut(), "Base Bravo").unwrap();
        let bravo = capture(&source);
        assert_ne!(bravo, expected);
        let mut fresh = app(true);
        fresh.init_resource::<SaveControl>();
        fresh.world_mut().resource_mut::<SaveControl>().root = root;
        profiles::open(fresh.world_mut(), "Base Alpha").unwrap();
        assert_eq!(capture(&fresh), expected);
        assert_eq!(
            fresh
                .world()
                .resource::<MarsTerrain>()
                .building_solids
                .len(),
            12
        );
        let good = Session::capture(fresh.world()).unwrap();
        let mut invalid = Vec::new();
        let mut bad = good.clone();
        bad.construction.inventory.slots[6]
            .as_mut()
            .unwrap()
            .quantity = 21;
        invalid.push(bad);
        let mut bad = good.clone();
        bad.buildings.facilities[0].id = BuildingId(1);
        invalid.push(bad);
        let mut bad = good.clone();
        bad.base.energy_j = crate::base::BATTERY_J + 1.0;
        invalid.push(bad);
        let mut bad = good.clone();
        bad.base.oxygen_s = f64::NAN;
        invalid.push(bad);
        let mut bad = good.clone();
        bad.base.sources[0].cooldown_s = 0.0;
        invalid.push(bad);
        let mut bad = good.clone();
        bad.buildings.facilities[0].inventory.slots.push(None);
        invalid.push(bad);
        for bad in invalid {
            assert!(Session::decode(&bad.encode()).is_err());
            assert!(bad.restore(fresh.world_mut()).is_err());
            assert_eq!(capture(&fresh), expected);
        }
        profiles::open(fresh.world_mut(), "Base Bravo").unwrap();
        assert_eq!(capture(&fresh), bravo);
    }
    #[test]
    fn adventure_names_are_portable_and_never_escape_the_save_root() {
        let folder = Folder::new();
        let mut game = app(true);
        game.init_resource::<SaveControl>();
        game.world_mut().resource_mut::<SaveControl>().root = folder.0.join("profiles");
        for name in [
            "",
            "..",
            "../ailleurs",
            "a/b",
            "a\\b",
            "CON",
            "nul",
            "COM1",
            "LPT9",
            "a:",
            &"x".repeat(41),
        ] {
            assert!(profiles::create(game.world_mut(), name).is_err(), "{name}");
        }
        assert!(!folder.0.join("profiles").exists());
        profiles::create(game.world_mut(), "Évasion 01").unwrap();
        assert!(folder.0.join("profiles/Évasion 01/session.solar").is_file());
    }

    #[test]
    fn complete_session_roundtrip_after_construction_airborne_rebase_and_settings() {
        let mut source = built();
        prepare(&mut source, 64.1, 0.0);
        source
            .world_mut()
            .resource_scope(|world, mut t: Mut<MarsTerrain>| {
                world.resource_mut::<SurfaceState>().migrate(&mut t);
            });
        {
            let mut state = source.world_mut().resource_mut::<SurfaceState>();
            state.feet.y += 2.0;
            state.velocity = DVec3::new(1.0, 2.0, 3.0);
            state.grounded = false;
            state.yaw = 0.7;
            state.pitch = -0.3;
        }
        source
            .world_mut()
            .resource_mut::<PlayerSettings>()
            .fov_index = 2;
        source
            .world_mut()
            .resource_mut::<PlayerSettings>()
            .msaa_index = 0;
        source.update();
        let text = capture(&source);
        let folder = Folder::new();
        save(source.world(), &folder.file()).unwrap();
        let mut target = app(false);
        let entity = target
            .world_mut()
            .query_filtered::<Entity, With<GameCamera>>()
            .single(target.world())
            .unwrap();
        assert!(!load(target.world_mut(), &folder.file()).unwrap());
        assert_eq!(capture(&target), text);
        assert_eq!(
            target
                .world()
                .resource::<MarsTerrain>()
                .building_solids
                .len(),
            12
        );
        assert!(target.world().resource::<MarsTerrain>().globe.is_some());
        assert_eq!(
            target
                .world_mut()
                .query_filtered::<Entity, With<GameCamera>>()
                .single(target.world())
                .unwrap(),
            entity
        );
        assert!(!target.world().resource::<Explorer>().captured);
        for (a, b) in source
            .world()
            .resource::<Buildings>()
            .shelters
            .iter()
            .zip(&target.world().resource::<Buildings>().shelters)
        {
            assert_eq!(a.id, b.id);
            assert_eq!(a.position_m, b.position_m);
            assert_eq!(a.orientation, b.orientation);
        }
        let before = target.world().resource::<Oxygen>().remaining_seconds;
        target.update();
        assert!(target.world().resource::<Oxygen>().remaining_seconds < before);
    }
    #[test]
    fn restored_built_shelter_recharges_and_next_id_inventory_and_depletion_continue() {
        let source = built();
        let snapshot = Session::capture(source.world()).unwrap();
        let mut target = app(true);
        snapshot.restore(target.world_mut()).unwrap();
        let b = target.world().resource::<Buildings>().shelters[1].clone();
        let p = target.world().resource::<MarsTerrain>().chart(b.position_m);
        prepare(&mut target, p.x, p.z);
        target
            .world_mut()
            .resource_mut::<Oxygen>()
            .remaining_seconds = 60.0;
        for _ in 0..60 {
            target.update();
        }
        assert_eq!(target.world().resource::<Oxygen>().shelter, Some(b.id));
        assert!(target.world().resource::<Oxygen>().remaining_seconds > 89.9);
        prepare(&mut target, 8.0, 0.0);
        press(&mut target, KeyCode::KeyE);
        assert_eq!(
            target
                .world()
                .resource::<Construction>()
                .inventory
                .count(crate::inventory::Item::Alloy),
            0
        );
        for z in [0.0, -6.0] {
            prepare(&mut target, -8.0, z);
            press(&mut target, KeyCode::KeyE);
        }
        prepare(&mut target, -16.0, 0.0);
        press(&mut target, KeyCode::KeyB);
        press(&mut target, KeyCode::Enter);
        press(&mut target, KeyCode::Enter);
        assert_eq!(
            target.world().resource::<Buildings>().shelters[2].id,
            BuildingId(3)
        );
        assert_eq!(
            target
                .world()
                .resource::<Construction>()
                .inventory
                .count(crate::inventory::Item::Alloy),
            0
        );
        for key in [KeyCode::KeyM, KeyCode::KeyG, KeyCode::KeyO, KeyCode::KeyU] {
            press(&mut target, key);
            press(&mut target, KeyCode::KeyM);
            assert_eq!(
                target
                    .world()
                    .resource::<MarsTerrain>()
                    .building_solids
                    .len(),
                18
            );
        }
    }
    #[test]
    fn failure_space_historical_and_unvisited_sessions_survive_loading() {
        for mode in ["unvisited", "space", "failed", "historical"] {
            let mut source = if mode == "historical" {
                let mut a = app(false);
                press(&mut a, KeyCode::KeyM);
                a
            } else if mode == "unvisited" {
                app(true)
            } else {
                built()
            };
            if mode == "failed" {
                let mut o = source.world_mut().resource_mut::<Oxygen>();
                o.failed = true;
                o.remaining_seconds = 0.0;
            }
            if mode == "space" {
                press(&mut source, KeyCode::KeyM);
            }
            let text = capture(&source);
            let mut target = app(true);
            Session::decode(&text)
                .unwrap()
                .restore(target.world_mut())
                .unwrap();
            assert_eq!(capture(&target), text, "{mode}");
            if mode == "failed" {
                press(&mut target, KeyCode::KeyR);
                assert!(!target.world().resource::<Oxygen>().failed);
                assert_eq!(target.world().resource::<Buildings>().shelters.len(), 2);
            }
            if mode == "unvisited" {
                press(&mut target, KeyCode::KeyM);
                assert_eq!(target.world().resource::<Construction>().deposits.len(), 8);
            }
        }
    }
    #[test]
    fn corrupt_incompatible_missing_oversized_and_invalid_state_never_mutate_session() {
        let mut target = built();
        let before = capture(&target);
        let folder = Folder::new();
        assert!(load(target.world_mut(), &folder.file()).is_err());
        for text in [
            String::from("incomplet"),
            before.replace("SOLAR_SURVIVAL 3", "SOLAR_SURVIVAL 2"),
            resign(&before.replace("SOLAR_SURVIVAL 3", "SOLAR_SURVIVAL 2")),
            resign(&before.replace("buildings 2 2", "buildings 999999999 2")),
            resign(&before.replace("oxygen ", "oxygen NaN ")),
            "x".repeat(MAX_BYTES as usize + 1),
        ] {
            fs::write(folder.file(), text).unwrap();
            assert!(load(target.world_mut(), &folder.file()).is_err());
            assert_eq!(capture(&target), before);
        }
        let mut bad = Session::capture(target.world()).unwrap();
        bad.seed += 1;
        assert!(bad.restore(target.world_mut()).is_err());
        assert_eq!(capture(&target), before);
        let mut bad = Session::capture(target.world()).unwrap();
        bad.surface.feet.x = 1e9;
        assert!(bad.restore(target.world_mut()).is_err());
        assert_eq!(capture(&target), before);
        let mut bad = Session::capture(target.world()).unwrap();
        bad.buildings.shelters[1].id = BuildingId(1);
        assert!(Session::decode(&bad.encode()).is_err());
        let mut bad = Session::capture(target.world()).unwrap();
        bad.orientation = DQuat::from_xyzw(0.0, 0.0, 0.0, 0.0);
        assert!(Session::decode(&bad.encode()).is_err());
    }
    #[test]
    fn loading_is_rejected_in_diagnostics_without_survival_render_assets() {
        let mut target = built();
        let text = capture(&target);
        let before = target.world().resource::<Buildings>().shelters[1].position_m;
        target.world_mut().resource_mut::<Oxygen>().enabled = false;
        assert!(
            Session::decode(&text)
                .unwrap()
                .restore(target.world_mut())
                .is_err()
        );
        assert!(!target.world().resource::<Oxygen>().enabled);
        assert_eq!(
            target.world().resource::<Buildings>().shelters[1].position_m,
            before
        );
        target.world_mut().resource_mut::<Oxygen>().enabled = true;
        target
            .world_mut()
            .resource_mut::<MarsTerrain>()
            .cube_enabled = true;
        assert!(
            Session::decode(&text)
                .unwrap()
                .restore(target.world_mut())
                .is_err()
        );
        assert!(target.world().resource::<MarsTerrain>().cube_enabled);
        target
            .world_mut()
            .resource_mut::<MarsTerrain>()
            .cube_enabled = false;
        assert_eq!(capture(&target), text);
    }

    #[test]
    fn overwrite_backup_recovery_and_failed_write_keep_existing_files() {
        let mut source = built();
        let folder = Folder::new();
        let path = folder.file();
        save(source.world(), &path).unwrap();
        let first = fs::read(&path).unwrap();
        source
            .world_mut()
            .resource_mut::<Oxygen>()
            .remaining_seconds = 80.0;
        save(source.world(), &path).unwrap();
        assert_eq!(fs::read(sibling(&path, ".bak")).unwrap(), first);
        let second = fs::read(&path).unwrap();
        let tmp = sibling(&path, &format!(".tmp-{}", std::process::id()));
        fs::write(&tmp, "préexistant").unwrap();
        assert!(save(source.world(), &path).is_err());
        assert_eq!(fs::read_to_string(&tmp).unwrap(), "préexistant");
        assert_eq!(fs::read(&path).unwrap(), second);
        fs::remove_file(&tmp).unwrap();
        fs::remove_file(&path).unwrap();
        assert!(load(source.world_mut(), &path).unwrap());
        assert_eq!(capture(&source).as_bytes(), first);
        let directory = folder.0.join("directory");
        fs::create_dir(&directory).unwrap();
        assert!(save(source.world(), &directory).is_err());
        assert!(directory.is_dir());
    }
}
