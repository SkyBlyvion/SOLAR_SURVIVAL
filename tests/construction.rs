use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    math::{DQuat, DVec3},
    prelude::*,
    time::TimeUpdateStrategy,
    window::{CursorOptions, PrimaryWindow},
};
use solar_survival::{
    GameSet,
    buildings::{BuildingId, Buildings},
    camera::ExplorationPlugin,
    config::GameConfig,
    planets::terrain::MarsTerrain,
    player::{Explorer, surface::SurfaceState},
    survival::{CAPACITY_SECONDS, Oxygen, SurvivalPlugin},
    universe::{UniversePlugin, catalog::SolarSystem},
};
use std::time::Duration;
fn app(fps: u32) -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .insert_resource(GameConfig {
            mars_global: true,
            ..default()
        })
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(
            1.0 / fps as f64,
        )))
        .init_resource::<ButtonInput<KeyCode>>()
        .init_resource::<ButtonInput<MouseButton>>()
        .init_resource::<AccumulatedMouseMotion>()
        .init_resource::<AccumulatedMouseScroll>()
        .configure_sets(
            Update,
            (GameSet::Input, GameSet::Simulation, GameSet::Navigation).chain(),
        )
        .add_plugins((
            UniversePlugin,
            ExplorationPlugin,
            SurvivalPlugin,
            solar_survival::construction::ConstructionPlugin,
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
    press(&mut app, KeyCode::KeyM);
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
fn placement(app: &mut App) {
    press(app, KeyCode::KeyB);
    assert!(app.world().resource::<Construction>().catalog_open);
    press(app, KeyCode::Enter);
    assert!(app.world().resource::<Construction>().active);
}
fn tick(app: &mut App, n: u32) {
    for _ in 0..n {
        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .clear();
    }
}
use solar_survival::construction::{Construction, SHELTER_COST};
fn prepare(app: &mut App, x: f64, z: f64) {
    let t = app.world().resource::<MarsTerrain>();
    let h = t.support_height(DVec3::new(x, 0.0, z), 0.3);
    let mut state = app.world_mut().resource_mut::<SurfaceState>();
    state.feet = DVec3::new(x, h + 0.002, z);
    state.velocity = DVec3::ZERO;
}
fn collect(app: &mut App, index: usize) {
    let point = app.world().resource::<Construction>().deposits[index].position_m;
    let p = app.world().resource::<MarsTerrain>().chart(point);
    prepare(app, p.x, p.z);
    press(app, KeyCode::KeyE);
}
fn start_clear(app: &mut App) {
    prepare(app, 16.0, 0.0);
    app.world_mut().resource_mut::<SurfaceState>().yaw = 0.0;
    placement(app);
    assert!(
        app.world()
            .resource::<Construction>()
            .preview
            .as_ref()
            .unwrap()
            .verdict
            .is_ok()
    );
}
#[test]
fn harvest_range_depletion_inventory_and_session_returns() {
    let mut app = app(60);
    assert_eq!(app.world().resource::<Construction>().deposits.len(), 8);
    press(&mut app, KeyCode::KeyE);
    assert_eq!(
        app.world()
            .resource::<Construction>()
            .inventory
            .count(solar_survival::inventory::Item::Alloy),
        0
    );
    collect(&mut app, 4);
    assert_eq!(
        app.world()
            .resource::<Construction>()
            .inventory
            .count(solar_survival::inventory::Item::Alloy),
        6
    );
    press(&mut app, KeyCode::KeyE);
    assert_eq!(
        app.world()
            .resource::<Construction>()
            .inventory
            .count(solar_survival::inventory::Item::Alloy),
        6
    );
    for exit in [KeyCode::KeyM, KeyCode::KeyG, KeyCode::KeyO, KeyCode::KeyU] {
        press(&mut app, exit);
        press(&mut app, KeyCode::KeyM);
        assert_eq!(app.world().resource::<Construction>().deposits.len(), 8);
        assert_eq!(
            app.world().resource::<Construction>().deposits[4].remaining,
            0
        );
        assert_eq!(
            app.world()
                .resource::<Construction>()
                .inventory
                .count(solar_survival::inventory::Item::Alloy),
            6
        );
    }
}
#[test]
fn rejected_cancelled_and_confirmed_placement_pay_only_once() {
    let mut app = app(60);
    #[derive(Resource, Default)]
    struct Cues(Vec<solar_survival::audio::SoundCue>);
    app.init_resource::<Cues>().add_observer(
        |event: On<solar_survival::audio::SoundCue>, mut cues: ResMut<Cues>| {
            cues.0.push(*event.event())
        },
    );
    placement(&mut app);
    press(&mut app, KeyCode::Enter);
    assert_eq!(app.world().resource::<Buildings>().shelters.len(), 1);
    assert_eq!(
        app.world()
            .resource::<Construction>()
            .inventory
            .count(solar_survival::inventory::Item::Alloy),
        0
    );
    press(&mut app, KeyCode::KeyX);
    collect(&mut app, 4);
    collect(&mut app, 5);
    start_clear(&mut app);
    let oxygen = app.world().resource::<Oxygen>().remaining_seconds;
    tick(&mut app, 60);
    assert!((app.world().resource::<Oxygen>().remaining_seconds - oxygen + 1.0).abs() < 1e-6);
    press(&mut app, KeyCode::Escape);
    assert!(!app.world().resource::<Construction>().active);
    assert_eq!(
        app.world()
            .resource::<Construction>()
            .inventory
            .count(solar_survival::inventory::Item::Alloy),
        SHELTER_COST
    );
    start_clear(&mut app);
    press(&mut app, KeyCode::Enter);
    assert_eq!(app.world().resource::<Buildings>().shelters.len(), 2);
    assert_eq!(
        app.world()
            .resource::<Construction>()
            .inventory
            .count(solar_survival::inventory::Item::Alloy),
        0
    );
    assert_eq!(
        app.world().resource::<MarsTerrain>().building_solids.len(),
        12
    );
    press(&mut app, KeyCode::Enter);
    assert_eq!(app.world().resource::<Buildings>().shelters.len(), 2);
    let cues = &app.world().resource::<Cues>().0;
    assert_eq!(
        cues.iter()
            .filter(|&&c| c == solar_survival::audio::SoundCue::Built)
            .count(),
        1
    );
    assert_eq!(
        cues.iter()
            .filter(|&&c| c == solar_survival::audio::SoundCue::Harvest)
            .count(),
        2
    );
    assert!(cues.contains(&solar_survival::audio::SoundCue::Refused));
}
#[test]
fn placement_blocks_player_obstacle_building_deposit_and_uncovered_terrain() {
    let mut app = app(60);
    collect(&mut app, 0);
    collect(&mut app, 1);
    prepare(&mut app, 0.0, 0.0);
    let t = app.world().resource::<MarsTerrain>();
    let state = app.world().resource::<SurfaceState>();
    let buildings = app.world().resource::<Buildings>();
    let mut c = Construction::default();
    c.seed_site(t);
    c.inventory = {
        let mut inventory = solar_survival::inventory::Inventory::default();
        inventory
            .add(solar_survival::inventory::Item::Alloy, 12)
            .unwrap();
        inventory
    };
    for (distance, lateral) in [(4.0, 0.0), (6.0, 0.0), (12.0, 0.0), (6.0, 8.0)] {
        c.distance_m = distance;
        c.lateral_m = lateral;
        assert!(
            c.candidate(t, state, buildings, &solar_survival::base::Base::default())
                .verdict
                .is_err(),
            "{distance} {lateral} candidate {:?}",
            c.candidate(t, state, buildings, &solar_survival::base::Base::default())
        );
    }
    c.distance_m = 9.0;
    c.lateral_m = 16.0;
    assert!(
        c.candidate(t, state, buildings, &solar_survival::base::Base::default())
            .verdict
            .is_ok()
    );
    let far = SurfaceState {
        feet: DVec3::new(64.0, 0.0, 0.0),
        ..default()
    };
    assert_eq!(
        c.candidate(t, &far, buildings, &solar_survival::base::Base::default())
            .verdict
            .unwrap_err(),
        "Terrain hors de la couverture locale"
    );
}
#[test]
fn admission_covers_full_triangles_and_rejects_historical_rough_ground() {
    let solar = SolarSystem::default();
    let mut world = World::new();
    world.insert_resource(solar);
    let mut t = MarsTerrain::from_world(&mut world);
    t.set_site(DVec3::Y);
    let mut c = Construction::default();
    c.inventory = {
        let mut inventory = solar_survival::inventory::Inventory::default();
        inventory
            .add(solar_survival::inventory::Item::Alloy, 12)
            .unwrap();
        inventory
    };
    c.lateral_m = 16.0;
    let state = SurfaceState {
        feet: DVec3::new(-16.0, 0.0, 9.0),
        ..default()
    };
    assert_eq!(
        c.candidate(
            &t,
            &state,
            &Buildings::default(),
            &solar_survival::base::Base::default()
        )
        .verdict
        .unwrap_err(),
        "Terrain trop irrégulier (variation > 0,4 m)"
    );
    t.globe_enabled = true;
    t.set_site(DVec3::Y);
    let (low, high) = t
        .construction_height_range(DVec3::new(16.0, 0.0, -9.0), 4.5)
        .unwrap();
    // P8 : le fournisseur mondial est réellement vallonné ; les extrema doivent
    // encore envelopper tous les points du carré, sans supposer une sphère lisse.
    assert!(high > low && high - low < 2.0);
    for x in 0..19 {
        for z in 0..19 {
            let height = t.height(11.5 + x as f64 * 0.5, -13.5 + z as f64 * 0.5);
            assert!(height >= low - 1e-9 && height <= high + 1e-9);
        }
    }
}
#[test]
fn constructed_shelter_door_wall_service_and_rebased_instances() {
    let mut app = app(60);
    collect(&mut app, 4);
    collect(&mut app, 5);
    start_clear(&mut app);
    press(&mut app, KeyCode::Enter);
    let built = app.world().resource::<Buildings>().shelters[1].clone();
    let front = app
        .world()
        .resource::<MarsTerrain>()
        .chart(built.position_m + built.orientation * DVec3::Z * 5.0);
    prepare(&mut app, front.x, front.z);
    app.world_mut().resource_mut::<Explorer>().captured = true;
    app.world_mut().resource_mut::<Oxygen>().remaining_seconds = 60.0;
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyW);
    tick(&mut app, 60);
    assert_eq!(app.world().resource::<Oxygen>().shelter, Some(built.id));
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .reset_all();
    tick(&mut app, 300);
    assert_eq!(
        app.world().resource::<Oxygen>().remaining_seconds,
        CAPACITY_SECONDS
    );
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyD);
    tick(&mut app, 120);
    assert!(app.world().resource::<SurfaceState>().feet.x < front.x + 2.6);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .reset_all();
    prepare(&mut app, 64.1, 0.0);
    app.world_mut()
        .resource_scope(|world, mut t: Mut<MarsTerrain>| {
            world.resource_mut::<SurfaceState>().migrate(&mut t);
        });
    let q = app.world().resource::<MarsTerrain>().orientation;
    for b in app.world().resource::<Buildings>().shelters.clone() {
        let p = app.world().resource::<MarsTerrain>().chart(b.position_m);
        // Le premier abri est près de la bordure du disque ; rentrer un mètre côté +X.
        let p = if b.id == BuildingId(1) {
            p + DVec3::X
        } else {
            p
        };
        prepare(&mut app, p.x, p.z);
        app.world_mut().resource_mut::<Oxygen>().remaining_seconds = 60.0;
        tick(&mut app, 60);
        assert_eq!(app.world().resource::<MarsTerrain>().orientation, q);
        assert_eq!(app.world().resource::<Oxygen>().shelter, Some(b.id));
        assert!(app.world().resource::<Oxygen>().remaining_seconds > 89.9);
    }
    let actual = &app.world().resource::<Buildings>().shelters[1];
    assert_eq!(actual.position_m, built.position_m);
    assert_eq!(actual.orientation, built.orientation);
    assert_eq!(
        app.world().resource::<MarsTerrain>().building_solids.len(),
        12
    );
}
#[test]
fn preview_controls_use_delta_time_and_follow_rotation() {
    for fps in [30, 60, 144] {
        let mut app = app(fps);
        collect(&mut app, 4);
        collect(&mut app, 5);
        start_clear(&mut app);
        {
            let mut k = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            k.press(KeyCode::ArrowRight);
            k.press(KeyCode::KeyL);
        }
        tick(&mut app, fps);
        let c = app.world().resource::<Construction>();
        assert!((c.lateral_m - 4.0).abs() < 1e-6);
        assert!((c.angle_rad - 1.0).abs() < 1e-6);
        let p = c.preview.as_ref().unwrap();
        assert!(p.verdict.is_ok());
        assert!(
            (p.orientation * DVec3::Z).distance(
                app.world().resource::<MarsTerrain>().orientation
                    * DQuat::from_rotation_y(1.0)
                    * DVec3::Z
            ) < 1e-6
        );
    }
}
#[test]
fn stale_preview_is_revalidated_before_payment_and_failure_cancels() {
    let mut app = app(60);
    collect(&mut app, 4);
    collect(&mut app, 5);
    start_clear(&mut app);
    app.world_mut().resource_mut::<Construction>().inventory = {
        let mut inventory = solar_survival::inventory::Inventory::default();
        inventory
            .add(solar_survival::inventory::Item::Alloy, 6)
            .unwrap();
        inventory
    };
    press(&mut app, KeyCode::Enter);
    assert_eq!(app.world().resource::<Buildings>().shelters.len(), 1);
    assert_eq!(
        app.world()
            .resource::<Construction>()
            .inventory
            .count(solar_survival::inventory::Item::Alloy),
        6
    );
    app.world_mut().resource_mut::<Oxygen>().remaining_seconds = 0.001;
    app.update();
    assert!(app.world().resource::<Oxygen>().failed);
    assert!(!app.world().resource::<Construction>().active);
    press(&mut app, KeyCode::KeyR);
    assert_eq!(
        app.world()
            .resource::<Construction>()
            .inventory
            .count(solar_survival::inventory::Item::Alloy),
        6
    );
}
#[test]
fn salvage_cannot_be_collected_through_a_solid_wall() {
    let mut app = app(60);
    let d = app.world().resource::<Construction>().deposits[4].clone();
    app.world_mut()
        .resource_mut::<Buildings>()
        .install_shelter(d.position_m + d.orientation * DVec3::X * 3.0, d.orientation);
    app.world_mut()
        .resource_scope(|world, mut t: Mut<MarsTerrain>| {
            world.resource::<Buildings>().publish_colliders(&mut t);
        });
    prepare(&mut app, 6.0, 0.0);
    press(&mut app, KeyCode::KeyE);
    assert_eq!(
        app.world()
            .resource::<Construction>()
            .inventory
            .count(solar_survival::inventory::Item::Alloy),
        0
    );
    assert_eq!(
        app.world().resource::<Construction>().deposits[4].remaining,
        6
    );
}
#[test]
fn two_player_built_instances_have_distinct_poses_and_services() {
    let mut app = app(60);
    for index in [4, 5, 0, 1] {
        collect(&mut app, index);
    }
    start_clear(&mut app);
    press(&mut app, KeyCode::Enter);
    prepare(&mut app, -16.0, 0.0);
    placement(&mut app);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyL);
    tick(&mut app, 45);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .reset_all();
    assert!(
        app.world()
            .resource::<Construction>()
            .preview
            .as_ref()
            .unwrap()
            .verdict
            .is_ok()
    );
    press(&mut app, KeyCode::Enter);
    assert_eq!(
        app.world()
            .resource::<Construction>()
            .inventory
            .count(solar_survival::inventory::Item::Alloy),
        0
    );
    let all = app.world().resource::<Buildings>().shelters.clone();
    assert_eq!(all.len(), 3);
    assert_ne!(all[1].id, all[2].id);
    assert_ne!(all[1].orientation, all[2].orientation);
    for b in &all[1..] {
        let p = app.world().resource::<MarsTerrain>().chart(b.position_m);
        prepare(&mut app, p.x, p.z);
        app.world_mut().resource_mut::<Oxygen>().remaining_seconds = 60.0;
        tick(&mut app, 60);
        assert_eq!(app.world().resource::<Oxygen>().shelter, Some(b.id));
        assert!(app.world().resource::<Oxygen>().remaining_seconds > 89.9);
    }
    for exit in [KeyCode::KeyM, KeyCode::KeyG, KeyCode::KeyO, KeyCode::KeyU] {
        press(&mut app, exit);
        press(&mut app, KeyCode::KeyM);
        assert_eq!(
            app.world().resource::<MarsTerrain>().building_solids.len(),
            18
        );
        for (a, b) in app
            .world()
            .resource::<Buildings>()
            .shelters
            .iter()
            .zip(&all)
        {
            assert_eq!(a.id, b.id);
            assert_eq!(a.position_m, b.position_m);
            assert_eq!(a.orientation, b.orientation);
        }
    }
}

#[test]
fn construction_catalog_releases_controls_consumes_oxygen_and_cancels_without_cost() {
    let mut app = app(60);
    app.world_mut().resource_mut::<Explorer>().captured = true;
    press(&mut app, KeyCode::KeyB);
    assert!(app.world().resource::<Construction>().catalog_open);
    let feet = app.world().resource::<SurfaceState>().feet;
    let oxygen = app.world().resource::<Oxygen>().remaining_seconds;
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyW);
    tick(&mut app, 60);
    assert!(!app.world().resource::<Explorer>().captured);
    assert!(app.world().resource::<SurfaceState>().feet.distance(feet) < 1e-6);
    assert!((app.world().resource::<Oxygen>().remaining_seconds - oxygen + 1.0).abs() < 1e-6);
    press(&mut app, KeyCode::Escape);
    assert!(!app.world().resource::<Construction>().catalog_open);
    assert!(!app.world().resource::<Construction>().active);
    assert_eq!(
        app.world()
            .resource::<Construction>()
            .inventory
            .count(solar_survival::inventory::Item::Alloy),
        0
    );
    assert_eq!(app.world().resource::<Buildings>().shelters.len(), 1);
}
