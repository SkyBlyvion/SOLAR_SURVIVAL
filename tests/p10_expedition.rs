//! Scénarios CPU/ECS uniquement : aucune fenêtre OS, aucun renderer ni périphérique.
use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    math::DVec3,
    prelude::*,
    time::TimeUpdateStrategy,
    window::{CursorOptions, PrimaryWindow},
};
use solar_survival::{
    GameSet,
    buildings::Buildings,
    camera::ExplorationPlugin,
    config::GameConfig,
    construction::{Construction, ConstructionPlugin},
    expedition::{Expedition, ExpeditionPlugin, REWARDS, SITE_COUNT},
    inventory::{Inventory, Item},
    persistence::{SaveControl, Session, profiles},
    planets::terrain::MarsTerrain,
    player::{Explorer, MovementMode, surface::SurfaceState},
    survival::{Oxygen, SurvivalPlugin},
    ui::{
        inventory::InventoryScreen,
        menu::{MenuState, PlayerSettings},
    },
    universe::{SimulationClock, UniversePlugin},
};
use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

fn app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .insert_resource(GameConfig {
            mars_global: true,
            ..default()
        })
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(
            1.0 / 30.0,
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
            UniversePlugin,
            ExplorationPlugin,
            SurvivalPlugin,
            ConstructionPlugin,
            ExpeditionPlugin,
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
fn tick(app: &mut App) {
    app.update();
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .clear();
}
fn press(app: &mut App, key: KeyCode) {
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(key);
    tick(app);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .reset_all();
}
fn feet(app: &App) -> DVec3 {
    app.world()
        .resource::<MarsTerrain>()
        .body_relative(app.world().resource::<SurfaceState>().feet)
}
fn go(app: &mut App, target: DVec3, range: f64) {
    app.world_mut().resource_mut::<Explorer>().captured = true;
    for key in [KeyCode::KeyW, KeyCode::ShiftLeft] {
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(key);
    }
    for _ in 0..30 * 65 {
        let t = app.world().resource::<MarsTerrain>();
        let s = app.world().resource::<SurfaceState>();
        let delta = target - feet(app);
        // La cible se projette sur le plan du contrôleur ; l'altitude vient des vrais triangles.
        let local = s.heading.inverse() * t.orientation.inverse() * delta;
        if local.x.hypot(local.z) < range {
            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .reset_all();
            return;
        }
        app.world_mut().resource_mut::<SurfaceState>().yaw = (-local.x).atan2(-local.z);
        tick(app);
        assert!(
            !app.world().resource::<Oxygen>().failed,
            "O₂ épuisé en trajet"
        );
        assert!(
            app.world()
                .resource::<MarsTerrain>()
                .globe
                .as_ref()
                .unwrap()
                .patches
                .len()
                <= 13
        );
    }
    panic!("Trajet bloqué : distance {}", target.distance(feet(app)));
}
fn home_waypoint(app: &App, x: f64, z: f64) -> DVec3 {
    let b = &app.world().resource::<Buildings>().shelters[0];
    b.position_m + b.orientation * DVec3::new(x, 0.0, z)
}
fn prepare_site(app: &mut App) {
    let site = app.world().resource::<Expedition>().sites
        [app.world().resource::<Expedition>().completed]
        .clone();
    app.world_mut()
        .resource_scope(|world, mut t: Mut<MarsTerrain>| {
            t.set_site(site.position_m.normalize());
            world.resource_mut::<Buildings>().publish_colliders(&mut t);
            let mut s = world.resource_mut::<SurfaceState>();
            s.feet = t.chart(site.position_m);
            s.feet.y = t.support_height(s.feet, 0.3) + 0.002;
            s.velocity = DVec3::ZERO;
        });
}
#[test]
fn three_real_ground_roundtrips_refill_risk_rewards_streaming_and_no_duplicate() {
    let mut a = app();
    #[derive(Resource, Default)]
    struct Cues(Vec<solar_survival::audio::SoundCue>);
    a.init_resource::<Cues>().add_observer(
        |event: On<solar_survival::audio::SoundCue>, mut cues: ResMut<Cues>| {
            cues.0.push(*event.event())
        },
    );
    press(&mut a, KeyCode::KeyM);
    let sites = a.world().resource::<Expedition>().sites.clone();
    let original_q = a.world().resource::<MarsTerrain>().orientation;
    let mut min_oxygen: f64 = 180.0;
    for (i, site) in sites.iter().enumerate() {
        // Quitter par la porte, puis contourner le mur avant de suivre la balise.
        let front = home_waypoint(&a, 0.0, 6.0);
        go(&mut a, front, 0.4);
        let waypoint = home_waypoint(&a, 12.0, 6.0);
        go(&mut a, waypoint, 0.4);
        go(&mut a, site.position_m, 1.0);
        assert!(site.position_m.distance(feet(&a)) < 1.1);
        press(&mut a, KeyCode::KeyF);
        assert!(a.world().resource::<Expedition>().carrying);
        assert!(a.world().resource::<Expedition>().discovered[i]);
        let inventory_before = a.world().resource::<Construction>().inventory.clone();
        press(&mut a, KeyCode::KeyF);
        assert!(a.world().resource::<Expedition>().carrying);
        assert_eq!(
            a.world().resource::<Construction>().inventory,
            inventory_before
        );
        let waypoint = home_waypoint(&a, 12.0, 6.0);
        go(&mut a, waypoint, 0.4);
        min_oxygen = min_oxygen.min(a.world().resource::<Oxygen>().remaining_seconds);
        let front = home_waypoint(&a, 0.0, 6.0);
        go(&mut a, front, 0.4);
        let center = home_waypoint(&a, 0.0, 0.0);
        go(&mut a, center, 0.4);
        press(&mut a, KeyCode::KeyF);
        assert_eq!(a.world().resource::<Expedition>().completed, i + 1);
        assert!(!a.world().resource::<Expedition>().carrying);
        let inventory = a.world().resource::<Construction>().inventory.clone();
        press(&mut a, KeyCode::KeyF);
        assert_eq!(a.world().resource::<Construction>().inventory, inventory);
        for _ in 0..180 {
            tick(&mut a);
        }
        assert_eq!(a.world().resource::<Oxygen>().remaining_seconds, 180.0);
    }
    assert!(
        (60.0..165.0).contains(&min_oxygen),
        "réserve minimale {min_oxygen}"
    );
    assert_ne!(original_q, a.world().resource::<MarsTerrain>().orientation);
    assert_eq!(
        a.world()
            .resource::<Construction>()
            .inventory
            .count(Item::Alloy),
        REWARDS.iter().map(|r| r.0).sum::<u32>()
    );
    assert_eq!(
        a.world()
            .resource::<Construction>()
            .inventory
            .count(Item::Circuit),
        REWARDS.iter().map(|r| r.1).sum::<u32>()
    );
    let cues = &a.world().resource::<Cues>().0;
    assert_eq!(
        cues.iter()
            .filter(|&&c| c == solar_survival::audio::SoundCue::Survey)
            .count(),
        3
    );
    assert_eq!(
        cues.iter()
            .filter(|&&c| c == solar_survival::audio::SoundCue::Report)
            .count(),
        3
    );
    assert!(cues.contains(&solar_survival::audio::SoundCue::Refused));
    println!("Trois allers-retours au sol ; O₂ minimal avant porte : {min_oxygen:.3} s");
}
#[test]
fn range_locked_sites_full_reward_failure_and_space_exits_keep_discovery_only() {
    let mut a = app();
    press(&mut a, KeyCode::KeyM);
    press(&mut a, KeyCode::KeyF);
    assert!(!a.world().resource::<Expedition>().carrying);
    let mut e = a.world().resource::<Expedition>().clone();
    assert!(e.scan(e.sites[1].position_m).is_err());
    assert!(e.scan(DVec3::NAN).is_err());
    e.scan(e.sites[0].position_m).unwrap();
    let mut inventory = Inventory::default();
    inventory.add(Item::Alloy, 152).unwrap(); // L'alliage de récompense tient, les circuits ne tiennent pas.
    let before = inventory.clone();
    assert!(e.report(&mut inventory).is_err());
    assert_eq!(inventory, before);
    assert!(e.carrying && e.completed == 0);
    for key in [KeyCode::KeyM, KeyCode::KeyG, KeyCode::KeyO, KeyCode::KeyU] {
        prepare_site(&mut a);
        press(&mut a, KeyCode::KeyF);
        assert!(a.world().resource::<Expedition>().carrying);
        press(&mut a, key);
        assert_eq!(a.world().resource::<Explorer>().mode, MovementMode::Space);
        let e = a.world().resource::<Expedition>();
        assert!(!e.carrying && e.completed == 0 && e.discovered[0]);
        press(&mut a, KeyCode::KeyM);
        press(&mut a, KeyCode::KeyF);
        assert_eq!(a.world().resource::<Construction>().inventory.used(), 0);
    }
    prepare_site(&mut a);
    press(&mut a, KeyCode::KeyF);
    a.world_mut().resource_mut::<Oxygen>().remaining_seconds = 0.01;
    tick(&mut a);
    assert!(a.world().resource::<Oxygen>().failed);
    assert!(!a.world().resource::<Expedition>().carrying);
    press(&mut a, KeyCode::KeyR);
    press(&mut a, KeyCode::KeyF);
    assert_eq!(a.world().resource::<Expedition>().completed, 0);
}
#[test]
fn screens_block_scan_while_pause_and_acceleration_keep_the_same_site_poses() {
    let mut a = app();
    press(&mut a, KeyCode::KeyM);
    prepare_site(&mut a);
    let site = a.world().resource::<Expedition>().sites[0].position_m;
    a.insert_resource(MenuState::default());
    press(&mut a, KeyCode::KeyF);
    assert!(!a.world().resource::<Expedition>().carrying);
    a.world_mut().remove_resource::<MenuState>();
    a.world_mut().resource_mut::<InventoryScreen>().open = true;
    press(&mut a, KeyCode::KeyF);
    assert!(!a.world().resource::<Expedition>().carrying);
    a.world_mut().resource_mut::<InventoryScreen>().open = false;
    a.world_mut().resource_mut::<Construction>().catalog_open = true;
    press(&mut a, KeyCode::KeyF);
    assert!(!a.world().resource::<Expedition>().carrying);
    a.world_mut().resource_mut::<Construction>().catalog_open = false;
    a.world_mut().resource_mut::<SimulationClock>().paused = false;
    a.world_mut().resource_mut::<SimulationClock>().speed_index = 3;
    for _ in 0..30 {
        tick(&mut a);
    }
    assert_eq!(a.world().resource::<Expedition>().sites[0].position_m, site);
    press(&mut a, KeyCode::KeyF);
    assert!(a.world().resource::<Expedition>().carrying);
}
#[test]
fn profiles_restart_with_pending_data_and_independent_discoveries_without_duplicate_rewards() {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let root = std::env::temp_dir().join(format!(
        "solar-p10-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let mut a = app();
    a.insert_resource(SaveControl {
        root: root.clone(),
        ..default()
    });
    profiles::create(a.world_mut(), "P10 Alpha").unwrap();
    press(&mut a, KeyCode::KeyM);
    prepare_site(&mut a);
    press(&mut a, KeyCode::KeyF);
    let expected_sites = a
        .world()
        .resource::<Expedition>()
        .sites
        .iter()
        .map(|s| (s.position_m, s.orientation))
        .collect::<Vec<_>>();
    profiles::create(a.world_mut(), "P10 Bravo").unwrap();
    assert!(a.world().resource::<Expedition>().sites.is_empty());
    assert_eq!(a.world().resource::<Expedition>().completed, 0);
    let mut b = app();
    b.insert_resource(SaveControl {
        root: root.clone(),
        ..default()
    });
    profiles::open(b.world_mut(), "P10 Alpha").unwrap();
    assert!(b.world().resource::<Expedition>().carrying);
    assert_eq!(
        b.world()
            .resource::<Expedition>()
            .sites
            .iter()
            .map(|s| (s.position_m, s.orientation))
            .collect::<Vec<_>>(),
        expected_sites
    );
    // Refus de progression incohérente, même avec un checksum valide (FNV documenté).
    let text = Session::capture(b.world()).unwrap().encode();
    for payload in [
        "4 false true true true",
        "0 true false false false",
        "1 false false false false",
        "0 false true true false",
    ] {
        let bad = text.replace(
            "expedition 0 true true false false",
            &format!("expedition {payload}"),
        );
        let (body, _) = bad.rsplit_once("checksum ").unwrap();
        let sum = body
            .as_bytes()
            .iter()
            .fold(0xcbf29ce484222325_u64, |h, byte| {
                (h ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
            });
        let bad = format!("{body}checksum {sum:016x}\n");
        assert!(Session::decode(&bad).is_err());
        assert_eq!(Session::capture(b.world()).unwrap().encode(), text);
    }
    let waypoint = home_waypoint(&b, 12.0, 6.0);
    go(&mut b, waypoint, 0.4);
    let front = home_waypoint(&b, 0.0, 6.0);
    go(&mut b, front, 0.4);
    let center = home_waypoint(&b, 0.0, 0.0);
    go(&mut b, center, 0.4);
    press(&mut b, KeyCode::KeyF);
    assert_eq!(b.world().resource::<Expedition>().completed, 1);
    profiles::open(b.world_mut(), "P10 Bravo").unwrap();
    assert_eq!(b.world().resource::<Expedition>().completed, 0);
    assert_eq!(
        b.world().resource::<Expedition>().discovered,
        [false; SITE_COUNT]
    );
    profiles::open(b.world_mut(), "P10 Alpha").unwrap();
    assert_eq!(b.world().resource::<Expedition>().completed, 1);
    let inventory = b.world().resource::<Construction>().inventory.clone();
    press(&mut b, KeyCode::KeyF);
    assert_eq!(b.world().resource::<Construction>().inventory, inventory);
    std::fs::remove_dir_all(root).unwrap();
}
#[test]
fn bearing_is_horizontal_and_consistent_after_rebase() {
    let mut a = app();
    press(&mut a, KeyCode::KeyM);
    let t = a.world().resource::<MarsTerrain>();
    let s = a.world().resource::<SurfaceState>();
    let origin = feet(&a);
    let forward = t.orientation * s.local_rotation(t) * -DVec3::Z;
    let right = forward.cross(origin.normalize()).normalize();
    assert_eq!(
        solar_survival::ui::expedition::bearing(origin + forward * 100.0, t, s),
        "devant"
    );
    assert_eq!(
        solar_survival::ui::expedition::bearing(origin - forward * 100.0, t, s),
        "derrière"
    );
    assert_eq!(
        solar_survival::ui::expedition::bearing(origin + right * 100.0, t, s),
        "droite 90°"
    );
    let mut state = s.clone();
    state.pitch = 1.5;
    assert_eq!(
        solar_survival::ui::expedition::bearing(origin + right * 100.0, t, &state),
        "droite 90°"
    );
    a.world_mut()
        .resource_scope(|world, mut terrain: Mut<MarsTerrain>| {
            let mut state = world.resource_mut::<SurfaceState>();
            state.feet.x = 64.1;
            state.feet.y = terrain.support_height(state.feet, 0.3) + 0.002;
            let before =
                solar_survival::ui::expedition::bearing(origin + right * 100.0, &terrain, &state);
            assert!(state.migrate(&mut terrain));
            assert_eq!(
                solar_survival::ui::expedition::bearing(origin + right * 100.0, &terrain, &state),
                before
            );
        });
}
