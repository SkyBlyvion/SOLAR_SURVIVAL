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
    player::{Explorer, MovementMode, surface::SurfaceState},
    survival::{CAPACITY_SECONDS, Oxygen, SurvivalPlugin},
    universe::{SimulationClock, UniversePlugin, catalog::SolarSystem, celestial_body::BodyId},
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
        .add_plugins((UniversePlugin, ExplorationPlugin, SurvivalPlugin));
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
fn tick(app: &mut App, n: u32) {
    for _ in 0..n {
        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .clear();
    }
}
#[test]
fn oxygen_policy_full_delta_menu_space_saturation_and_latched_failure() {
    let mut o = Oxygen::default();
    o.advance(12.0, true, false, None);
    assert_eq!(o.remaining_seconds, 168.0);
    o.advance(999.0, true, true, None);
    o.advance(999.0, false, false, None);
    o.advance(999.0, true, true, Some(BuildingId(8)));
    assert_eq!(o.remaining_seconds, 168.0);
    o.advance(1.0, true, false, Some(BuildingId(8)));
    assert_eq!(o.remaining_seconds, CAPACITY_SECONDS);
    for invalid in [f64::NAN, f64::INFINITY, -5.0] {
        o.advance(invalid, true, false, None);
    }
    assert_eq!(o.remaining_seconds, CAPACITY_SECONDS);
    o.advance(200.0, true, false, None);
    assert!(o.failed);
    assert_eq!(o.remaining_seconds, 0.0);
    o.advance(10.0, true, false, Some(BuildingId(8)));
    assert!(o.failed);
    assert_eq!(o.remaining_seconds, 0.0);
    o.restart();
    assert!(!o.failed);
    assert_eq!(o.remaining_seconds, CAPACITY_SECONDS);
}
#[test]
fn ecs_consumption_real_time_matches_fps_and_orbital_controls() {
    for fps in [30, 60, 144] {
        let mut app = app(fps);
        let initial = app.world().resource::<Oxygen>().remaining_seconds;
        app.world_mut().resource_mut::<SimulationClock>().paused = true;
        tick(&mut app, fps * 2);
        assert!((app.world().resource::<Oxygen>().remaining_seconds - initial + 2.0).abs() < 1e-6);
        press(&mut app, KeyCode::KeyP);
        for _ in 0..3 {
            press(&mut app, KeyCode::KeyT);
        }
        let initial = app.world().resource::<Oxygen>().remaining_seconds;
        tick(&mut app, fps * 2);
        assert!((app.world().resource::<Oxygen>().remaining_seconds - initial + 2.0).abs() < 1e-6);
        app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs(12)));
        let initial = app.world().resource::<Oxygen>().remaining_seconds;
        app.update();
        assert!((app.world().resource::<Oxygen>().remaining_seconds - initial + 12.0).abs() < 1e-9);
    }
}
#[test]
fn playable_door_refill_wall_collision_failure_and_restart() {
    let mut app = app(60);
    app.world_mut().resource_mut::<Oxygen>().remaining_seconds = 80.0;
    app.world_mut().resource_mut::<Explorer>().captured = true;
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyW);
    tick(&mut app, 80); // Six metres by normal walking through the doorway.
    assert!(app.world().resource::<Oxygen>().shelter.is_some());
    assert!(app.world().resource::<Oxygen>().remaining_seconds > 90.0);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .reset_all();
    tick(&mut app, 240);
    assert_eq!(
        app.world().resource::<Oxygen>().remaining_seconds,
        CAPACITY_SECONDS
    );
    // Wall collision, then back through the same open door.
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyD);
    tick(&mut app, 120);
    let x = app.world().resource::<SurfaceState>().feet.x;
    assert!((2.5..2.6).contains(&x), "wall x={x}");
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .reset_all();
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyA);
    tick(&mut app, 34);
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .reset_all();
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyS);
    tick(&mut app, 120);
    assert!(app.world().resource::<Oxygen>().shelter.is_none());
    app.world_mut().resource_mut::<Oxygen>().remaining_seconds = 0.1;
    tick(&mut app, 30);
    assert!(app.world().resource::<Oxygen>().failed);
    assert!(!app.world().resource::<Explorer>().captured);
    let before = app.world().resource::<SurfaceState>().feet;
    tick(&mut app, 120);
    assert!(before.distance(app.world().resource::<SurfaceState>().feet) < 1e-7);
    press(&mut app, KeyCode::KeyM);
    assert_eq!(app.world().resource::<Explorer>().mode, MovementMode::Space);
    tick(&mut app, 60);
    assert!(app.world().resource::<Oxygen>().failed);
    assert_eq!(app.world().resource::<Oxygen>().remaining_seconds, 0.0);
    press(&mut app, KeyCode::KeyR);
    assert!(!app.world().resource::<Oxygen>().failed);
    assert_eq!(
        app.world().resource::<Oxygen>().remaining_seconds,
        CAPACITY_SECONDS
    );
    assert!(app.world().resource::<Oxygen>().shelter.is_some());
}
#[test]
fn home_and_oxygen_persist_through_every_space_exit_and_orbital_rotation() {
    let mut app = app(60);
    let building = app.world().resource::<Buildings>().shelters[0].clone();
    app.world_mut()
        .resource_mut::<SimulationClock>()
        .speed_index = 3;
    for exit in [KeyCode::KeyM, KeyCode::KeyG, KeyCode::KeyO, KeyCode::KeyU] {
        press(&mut app, exit);
        let before = app.world().resource::<Oxygen>().remaining_seconds;
        tick(&mut app, 60);
        assert_eq!(app.world().resource::<Oxygen>().remaining_seconds, before);
        press(&mut app, KeyCode::KeyM);
        assert_eq!(
            app.world().resource::<Explorer>().mode,
            MovementMode::Surface
        );
        let b = &app.world().resource::<Buildings>().shelters[0];
        assert_eq!(
            (b.id, b.position_m, b.orientation),
            (building.id, building.position_m, building.orientation)
        );
        assert_eq!(app.world().resource::<Buildings>().shelters.len(), 1);
        assert!(
            (app.world()
                .resource::<MarsTerrain>()
                .body_relative(app.world().resource::<SurfaceState>().feet)
                - b.position_m)
                .length()
                < 6.01
        );
    }
}
#[test]
fn second_instance_and_rebase_keep_body_poses_contacts_and_recharge() {
    let mut app = app(60);
    let mut point = DVec3::new(50.0, 0.0, 25.0);
    let t = app.world().resource::<MarsTerrain>();
    point.y = t.height(point.x, point.z);
    let body = t.body_relative(point);
    let orientation = t.orientation;
    let id = app
        .world_mut()
        .resource_mut::<Buildings>()
        .install_shelter(body, orientation);
    app.world_mut().resource_scope(|world, b: Mut<Buildings>| {
        b.publish_colliders(&mut world.resource_mut::<MarsTerrain>());
    });
    let poses: Vec<_> = app
        .world()
        .resource::<Buildings>()
        .shelters
        .iter()
        .map(|b| (b.id, b.position_m, b.orientation))
        .collect();
    // Move beyond both a patch boundary and the 64 m threshold by real running.
    app.world_mut().resource_mut::<Explorer>().captured = true;
    app.world_mut().resource_mut::<SurfaceState>().yaw = -std::f64::consts::FRAC_PI_2;
    {
        let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        keys.press(KeyCode::KeyW);
        keys.press(KeyCode::ShiftLeft);
    }
    let before_key = {
        let t = app.world().resource::<MarsTerrain>();
        solar_survival::planets::globe_terrain::key_at(
            t.body_relative(app.world().resource::<SurfaceState>().feet),
        )
    };
    tick(&mut app, 900);
    let after_key = {
        let t = app.world().resource::<MarsTerrain>();
        solar_survival::planets::globe_terrain::key_at(
            t.body_relative(app.world().resource::<SurfaceState>().feet),
        )
    };
    assert_ne!(before_key, after_key);
    assert!(
        app.world()
            .resource::<MarsTerrain>()
            .globe
            .as_ref()
            .unwrap()
            .rebases
            > 0
    );
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .reset_all();
    let t = app.world().resource::<MarsTerrain>();
    let local = t.chart(body);
    // Centre of second instance after rebase, with the new collider frame.
    app.world_mut().resource_mut::<SurfaceState>().feet = local;
    app.world_mut().resource_mut::<Oxygen>().remaining_seconds = 40.0;
    tick(&mut app, 1);
    assert_eq!(app.world().resource::<Oxygen>().shelter, Some(id));
    assert!(app.world().resource::<Oxygen>().remaining_seconds > 40.0);
    let t = app.world().resource::<MarsTerrain>();
    for (solid, building) in t.building_solids.iter().zip(
        app.world()
            .resource::<Buildings>()
            .shelters
            .iter()
            .flat_map(|b| b.solids()),
    ) {
        let (center, _, rotation) = solid.chart_pose(t);
        assert!(
            t.body_relative(center).distance(building.center) < 2e-9,
            "écart métrique {}",
            t.body_relative(center).distance(building.center)
        );
        assert!(
            (t.orientation * rotation * DVec3::Y).distance(building.orientation * DVec3::Y) < 1e-12
        );
    }
    let after: Vec<_> = app
        .world()
        .resource::<Buildings>()
        .shelters
        .iter()
        .map(|b| (b.id, b.position_m, b.orientation))
        .collect();
    assert_eq!(poses, after);
    // Recharge the ORIGINAL building in the still-rebased chart, without resetting the site.
    let first = app.world().resource::<Buildings>().shelters[0].clone();
    let t = app.world().resource::<MarsTerrain>();
    let active_orientation = t.orientation;
    let rebases = t.globe.as_ref().unwrap().rebases;
    let local = t.chart(first.position_m + first.orientation * DVec3::X);
    assert!(
        t.globe
            .as_ref()
            .unwrap()
            .support_height(local, 0.3)
            .is_some()
    );
    app.world_mut().resource_mut::<SurfaceState>().feet = local;
    app.world_mut().resource_mut::<Oxygen>().remaining_seconds = 40.0;
    tick(&mut app, 1);
    assert_eq!(app.world().resource::<Oxygen>().shelter, Some(first.id));
    assert!(app.world().resource::<Oxygen>().remaining_seconds > 40.0);
    assert_eq!(
        app.world().resource::<MarsTerrain>().orientation,
        active_orientation
    );
    assert_eq!(
        app.world()
            .resource::<MarsTerrain>()
            .globe
            .as_ref()
            .unwrap()
            .rebases,
        rebases
    );
}
#[test]
fn refill_requires_entire_player_inside_no_through_wall_or_roof() {
    let mut b = Buildings::default();
    let q = DQuat::from_rotation_y(0.8) * DQuat::from_rotation_z(0.3);
    b.install_shelter(DVec3::Y * 3389500.0, q);
    let shelter = &b.shelters[0];
    for (point, expected) in [
        (DVec3::ZERO, true),
        (DVec3::new(2.6, 0.0, 0.0), false),
        (DVec3::new(0.0, 3.5, 0.0), false),
        (DVec3::new(0.0, 0.0, 3.2), false),
    ] {
        assert_eq!(
            shelter.contains(shelter.position_m + q * point, q),
            expected
        );
    }
}
#[test]
fn legacy_normal_game_keeps_a_usable_shelter() {
    let mut app = app(60);
    // Fresh normal session configured with artistic ground.
    app.world_mut().resource_mut::<MarsTerrain>().globe_enabled = false;
    *app.world_mut().resource_mut::<Buildings>() = Buildings::default();
    app.world_mut()
        .resource_mut::<MarsTerrain>()
        .building_solids
        .clear();
    press(&mut app, KeyCode::KeyM);
    press(&mut app, KeyCode::KeyM);
    app.world_mut().resource_mut::<Explorer>().captured = true;
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyW);
    tick(&mut app, 80);
    assert!(app.world().resource::<Oxygen>().shelter.is_some());
    let _ = app.world().resource::<SolarSystem>().body(BodyId::MARS);
}
