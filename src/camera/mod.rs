use crate::{
    GameSet,
    config::GameConfig,
    physics::stop_before_sphere,
    planets::terrain::MarsTerrain,
    player::{Explorer, MovementMode, surface::SurfaceState},
    universe::{
        SimulationClock, catalog::SolarSystem, celestial_body::BodyId, coordinates::RenderOrigin,
    },
};
use bevy::{
    camera::Hdr,
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    math::{DQuat, DVec3},
    post_process::bloom::{Bloom, BloomCompositeMode, BloomPrefilter},
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

#[derive(Component)]
pub struct SpaceCamera;
#[derive(Resource, Default)]
pub struct NavigationRequest {
    pub visit: bool,
    pub center: bool,
    pub overview: bool,
    pub detach: bool,
    pub surface: bool,
}
pub struct ExplorationPlugin;
impl Plugin for ExplorationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Explorer>()
            .init_resource::<SurfaceState>()
            .init_resource::<MarsTerrain>()
            .init_resource::<NavigationRequest>()
            .init_resource::<RenderOrigin>()
            .add_systems(Startup, setup)
            .add_systems(Update, controls.in_set(GameSet::Input))
            .add_systems(Update, navigate.in_set(GameSet::Navigation));
    }
}
fn setup(
    mut commands: Commands,
    solar: Res<SolarSystem>,
    config: Res<GameConfig>,
    mut pilot: ResMut<Explorer>,
) {
    pilot.visit(&solar, config.start_jd);
    commands.spawn((
        SpaceCamera,
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: 65_f32.to_radians(),
            near: 0.001,
            far: 30_000.0,
            ..default()
        }),
        Transform::from_rotation(pilot.rotation.as_quat()),
        Hdr,
        Bloom {
            intensity: 0.12,
            low_frequency_boost: 0.4,
            prefilter: BloomPrefilter {
                threshold: 1.8,
                threshold_softness: 0.1,
            },
            composite_mode: BloomCompositeMode::Additive,
            ..Bloom::NATURAL
        },
        Msaa::Sample4,
    ));
}
#[expect(
    clippy::too_many_arguments,
    reason = "Paramètres de système injectés par Bevy ECS."
)]
pub(crate) fn controls(
    menu: Option<Res<crate::ui::menu::MenuState>>,
    settings: Option<Res<crate::ui::menu::PlayerSettings>>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    scroll: Res<AccumulatedMouseScroll>,
    window: Single<(&Window, &mut CursorOptions), With<PrimaryWindow>>,
    mut pilot: ResMut<Explorer>,
    mut request: ResMut<NavigationRequest>,
    mut clock: ResMut<SimulationClock>,
    mut surface: ResMut<SurfaceState>,
) {
    let (window, mut cursor) = window.into_inner();
    if menu.is_some_and(|m| m.is_open()) {
        pilot.captured = false;
        pilot.boosted = false;
        cursor.visible = true;
        cursor.grab_mode = CursorGrabMode::None;
        *request = NavigationRequest::default();
        return;
    }
    let sensitivity = settings.map_or(1.0, |s| s.sensitivity());
    if keys.just_pressed(KeyCode::Escape) || !window.focused {
        pilot.captured = false;
    }
    if mouse.just_pressed(MouseButton::Right) && window.focused {
        pilot.captured = true;
    }
    cursor.visible = !pilot.captured;
    cursor.grab_mode = if pilot.captured {
        CursorGrabMode::Locked
    } else {
        CursorGrabMode::None
    };
    if pilot.captured {
        if pilot.mode == MovementMode::Surface {
            surface.look(motion.delta * sensitivity as f32);
        } else {
            let (yaw, pitch, _) = pilot.rotation.to_euler(EulerRot::YXZ);
            pilot.rotation = DQuat::from_euler(
                EulerRot::YXZ,
                yaw - motion.delta.x as f64 * 0.002 * sensitivity,
                (pitch - motion.delta.y as f64 * 0.002 * sensitivity).clamp(-1.55, 1.55),
                0.0,
            );
        }
        let change = if scroll.delta.y > 0.0 {
            1
        } else if scroll.delta.y < 0.0 {
            -1
        } else {
            0
        };
        if pilot.mode == MovementMode::Space {
            pilot.speed_index = (pilot.speed_index as i32 + change).clamp(0, 5) as usize;
        }
    }
    if pilot.mode == MovementMode::Space
        && (keys.just_pressed(KeyCode::Equal) || keys.just_pressed(KeyCode::PageUp))
    {
        pilot.speed_index = (pilot.speed_index + 1).min(5);
    }
    if pilot.mode == MovementMode::Space
        && (keys.just_pressed(KeyCode::Minus) || keys.just_pressed(KeyCode::PageDown))
    {
        pilot.speed_index = pilot.speed_index.saturating_sub(1);
    }
    pilot.boosted = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
    if keys.just_pressed(KeyCode::Tab) {
        pilot.selected = BodyId((pilot.selected.0 + 1) % 10);
    }
    for (i, key) in [
        KeyCode::Digit0,
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Digit7,
        KeyCode::Digit8,
        KeyCode::Digit9,
    ]
    .into_iter()
    .enumerate()
    {
        if keys.just_pressed(key) {
            pilot.selected = BodyId(i);
        }
    }
    request.surface |= keys.just_pressed(KeyCode::KeyM);
    request.visit |= keys.just_pressed(KeyCode::KeyG);
    request.center |= keys.just_pressed(KeyCode::KeyF);
    request.overview |= keys.just_pressed(KeyCode::KeyO);
    request.detach |= keys.just_pressed(KeyCode::KeyU);
    if keys.just_pressed(KeyCode::KeyT) {
        clock.speed_index = (clock.speed_index + 1) % 4;
    }
    if keys.just_pressed(KeyCode::KeyP) {
        clock.paused = !clock.paused;
    }
}
#[expect(
    clippy::too_many_arguments,
    reason = "Paramètres de système injectés par Bevy ECS."
)]
pub(crate) fn navigate(
    time: Res<Time<Real>>,
    keys: Res<ButtonInput<KeyCode>>,
    solar: Res<SolarSystem>,
    clock: Res<SimulationClock>,
    mut pilot: ResMut<Explorer>,
    mut request: ResMut<NavigationRequest>,
    mut origin: ResMut<RenderOrigin>,
    mut camera: Single<&mut Transform, With<SpaceCamera>>,
    mut surface: ResMut<SurfaceState>,
    mut terrain: ResMut<MarsTerrain>,
) {
    if request.surface {
        if pilot.mode == MovementMode::Surface {
            pilot.selected = BodyId::MARS;
            pilot.visit(&solar, clock.jd);
        } else {
            surface.enter(&mut pilot, &mut terrain, &solar, clock.jd);
        }
    }
    if pilot.mode == MovementMode::Surface && (request.visit || request.overview || request.detach)
    {
        let selected = pilot.selected;
        pilot.selected = BodyId::MARS;
        pilot.visit(&solar, clock.jd);
        pilot.selected = selected;
    }
    if request.visit {
        pilot.visit(&solar, clock.jd);
    }
    if request.overview {
        pilot.anchor = None;
        pilot.local_m = DVec3::new(0.0, 35.0, 18.0) * crate::universe::units::AU;
        let towards_sun = -pilot.local_m;
        pilot.look_toward(towards_sun);
    }
    if request.detach {
        let position = pilot.global(&solar, clock.jd);
        pilot.anchor = None;
        pilot.local_m = position;
    }
    if request.center && pilot.mode == MovementMode::Space {
        let direction = solar.position(pilot.selected, clock.jd) - pilot.global(&solar, clock.jd);
        pilot.look_toward(direction);
    }
    *request = NavigationRequest::default();
    if pilot.mode == MovementMode::Surface {
        surface.advance(
            &pilot,
            &terrain,
            &keys,
            time.delta_secs_f64(),
            solar.body(BodyId::MARS).gravitational_parameter,
        );
        surface.sync(
            &mut pilot,
            &terrain,
            solar
                .body(BodyId::MARS)
                .rotation(clock.seconds_since_epoch()),
        );
    } else {
        let mut position = pilot.global(&solar, clock.jd);
        // Expulser une caméra rattrapée par un corps en mouvement; rayons+zone de sécurité.
        for body in &solar.bodies {
            let center = solar.position(body.id, clock.jd);
            let relative = position - center;
            let radius = body.radius_m + 1000.0;
            if relative.length() < radius {
                position = center + relative.try_normalize().unwrap_or(DVec3::Y) * radius;
            }
        }
        let mut input = DVec3::ZERO;
        if pilot.captured {
            input.x =
                (keys.pressed(KeyCode::KeyD) as i32 - keys.pressed(KeyCode::KeyA) as i32) as f64;
            input.z =
                (keys.pressed(KeyCode::KeyS) as i32 - keys.pressed(KeyCode::KeyW) as i32) as f64;
            input.y = (keys.pressed(KeyCode::Space) as i32
                - (keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight))
                    as i32) as f64;
        }
        let delta =
            (pilot.rotation * input.normalize_or_zero()) * pilot.speed() * time.delta_secs_f64();
        let mut fraction: f64 = 1.0;
        for body in &solar.bodies {
            if let Some(hit) = stop_before_sphere(
                position,
                delta,
                solar.position(body.id, clock.jd),
                body.radius_m + 1000.0,
            ) {
                fraction = fraction.min(hit);
            }
        }
        let next = position + delta * fraction;
        if next.is_finite() && next.length() <= 1e16 {
            pilot.set_global(next, &solar, clock.jd);
        }
    }
    origin.0 = pilot.global(&solar, clock.jd);
    camera.translation = Vec3::ZERO;
    camera.rotation = pilot.rotation.as_quat();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::universe::UniversePlugin;
    use bevy::time::TimeUpdateStrategy;
    use std::time::Duration;

    fn app(fps: u32) -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(GameConfig::default())
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
            .add_plugins((UniversePlugin, ExplorationPlugin));
        app.world_mut().spawn((
            Window {
                focused: true,
                ..default()
            },
            CursorOptions::default(),
            PrimaryWindow,
        ));
        app.update();
        app
    }

    fn fly_one_second(fps: u32) -> f64 {
        let mut app = app(fps);
        let start = app.world().resource::<Explorer>().local_m;
        app.world_mut().resource_mut::<Explorer>().speed_index = 1;
        app.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .press(MouseButton::Right);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyW);
        for _ in 0..fps {
            app.update();
            app.world_mut()
                .resource_mut::<ButtonInput<MouseButton>>()
                .clear();
            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .clear();
        }
        let pilot = app.world().resource::<Explorer>();
        assert!(pilot.captured);
        assert_eq!(pilot.anchor, Some(BodyId::EARTH));
        pilot.local_m.distance(start)
    }

    #[test]
    fn actual_ecs_camera_movement_is_frame_rate_independent() {
        let sixty = fly_one_second(60);
        let high = fly_one_second(144);
        assert!((sixty - 250.0).abs() < 0.02, "60 FPS: {sixty}");
        assert!((high - 250.0).abs() < 0.02, "144 FPS: {high}");
        assert!((sixty - high).abs() < 0.02);
    }

    #[test]
    fn actual_ecs_controls_select_visit_and_release_cursor() {
        let mut app = app(60);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Digit4);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyG);
        app.update();
        let pilot = app.world().resource::<Explorer>();
        assert_eq!(pilot.selected, BodyId::MARS);
        assert_eq!(pilot.anchor, Some(BodyId::MARS));
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .reset_all();
        app.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .press(MouseButton::Right);
        app.update();
        assert!(app.world().resource::<Explorer>().captured);
        app.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .reset_all();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Escape);
        app.update();
        assert!(!app.world().resource::<Explorer>().captured);
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
    #[test]
    fn surface_entry_and_all_space_exits_preserve_navigation() {
        for exit in [KeyCode::KeyM, KeyCode::KeyG, KeyCode::KeyO, KeyCode::KeyU] {
            let mut app = app(60);
            app.world_mut()
                .resource_mut::<SimulationClock>()
                .speed_index = 3;
            press(&mut app, KeyCode::KeyM);
            let pilot = app.world().resource::<Explorer>();
            assert_eq!(pilot.mode, MovementMode::Surface);
            assert_eq!(pilot.anchor, Some(BodyId::MARS));
            assert!(pilot.local_m.length() < 3_389_510.0);
            for _ in 0..60 {
                app.update();
            }
            press(&mut app, exit);
            let pilot = app.world().resource::<Explorer>();
            assert_eq!(pilot.mode, MovementMode::Space);
            let solar = app.world().resource::<SolarSystem>();
            let jd = app.world().resource::<SimulationClock>().jd;
            let global = pilot.global(solar, jd);
            for body in &solar.bodies {
                assert!(global.distance(solar.position(body.id, jd)) >= body.radius_m + 999.999);
            }
            if exit == KeyCode::KeyU || exit == KeyCode::KeyO {
                assert_eq!(pilot.anchor, None);
            }
            press(&mut app, KeyCode::Digit3);
            press(&mut app, KeyCode::KeyG);
            assert_eq!(
                app.world().resource::<Explorer>().anchor,
                Some(BodyId::EARTH)
            );
        }
    }
    #[test]
    fn orbital_pause_and_acceleration_do_not_change_surface_physics() {
        let mut a = app(60);
        let mut b = app(60);
        press(&mut a, KeyCode::KeyM);
        press(&mut b, KeyCode::KeyM);
        a.world_mut().resource_mut::<SimulationClock>().paused = true;
        b.world_mut().resource_mut::<SimulationClock>().speed_index = 3;
        for app in [&mut a, &mut b] {
            let mut state = app.world_mut().resource_mut::<SurfaceState>();
            state.feet.y += 10.0;
            state.grounded = false;
        }
        for _ in 0..240 {
            a.update();
            b.update();
        }
        let a_state = a.world().resource::<SurfaceState>();
        let b_state = b.world().resource::<SurfaceState>();
        assert!(a_state.grounded && b_state.grounded);
        assert!(a_state.feet.distance(b_state.feet) < 1e-9);
        assert!(a_state.velocity.distance(b_state.velocity) < 1e-9);
        assert!(
            b.world().resource::<SimulationClock>().jd > a.world().resource::<SimulationClock>().jd
        );
    }
    #[test]
    fn surface_mouse_delta_is_integrated_once_without_delta_time() {
        for fps in [30, 60, 144] {
            let mut app = app(fps);
            press(&mut app, KeyCode::KeyM);
            app.world_mut().resource_mut::<Explorer>().captured = true;
            app.world_mut()
                .resource_mut::<AccumulatedMouseMotion>()
                .delta = Vec2::new(100.0, 50.0);
            app.update();
            let state = app.world().resource::<SurfaceState>();
            assert!((state.yaw - (std::f64::consts::TAU - 0.2)).abs() < 1e-12);
            assert!((state.pitch + 0.1).abs() < 1e-12);
        }
    }

    #[test]
    fn surface_pose_and_render_origin_follow_rotation_without_a_physics_tick() {
        let mut app = app(144);
        press(&mut app, KeyCode::KeyM);
        let local_m = app.world().resource::<Explorer>().local_m;
        let rotation = app.world().resource::<Explorer>().rotation;
        let feet = app.world().resource::<SurfaceState>().feet;
        app.world_mut().resource_mut::<SurfaceState>().accumulator = 0.0;
        app.world_mut()
            .resource_mut::<SimulationClock>()
            .speed_index = 3;
        app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(1)));
        app.update();
        let world = app.world_mut();
        let pilot = world.resource::<Explorer>();
        let terrain = world.resource::<MarsTerrain>();
        let state = world.resource::<SurfaceState>();
        let solar = world.resource::<SolarSystem>();
        let clock = world.resource::<SimulationClock>();
        assert_eq!(state.feet, feet);
        assert!(pilot.local_m.distance(local_m) > 100.0);
        assert!((pilot.rotation * DVec3::Y).distance(rotation * DVec3::Y) > 1e-5);
        let frame = terrain.inertial_frame(
            solar
                .body(BodyId::MARS)
                .rotation(clock.seconds_since_epoch()),
        );
        assert!(pilot.local_m.distance(frame.to_global(state.eye(terrain))) < 1e-9);
        assert_eq!(
            world.resource::<RenderOrigin>().0,
            pilot.global(solar, clock.jd)
        );
        let expected_rotation = pilot.rotation.as_quat();
        let camera = world
            .query_filtered::<&Transform, With<SpaceCamera>>()
            .single(world)
            .unwrap();
        assert_eq!(camera.rotation, expected_rotation);
        assert_eq!(camera.translation, Vec3::ZERO);
    }

    #[test]
    fn pause_freezes_site_rotation_but_walk_and_jump_match_accelerated_time() {
        let mut paused = app(60);
        let mut fast = app(60);
        for app in [&mut paused, &mut fast] {
            press(app, KeyCode::KeyM);
            app.world_mut().resource_mut::<Explorer>().captured = true;
            let mut keys = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
            keys.press(KeyCode::KeyD);
            keys.press(KeyCode::Space);
        }
        paused.world_mut().resource_mut::<SimulationClock>().paused = true;
        fast.world_mut()
            .resource_mut::<SimulationClock>()
            .speed_index = 3;
        let start = paused.world().resource::<Explorer>().local_m;
        let jd = paused.world().resource::<SimulationClock>().jd;
        let mut peak: f64 = 0.0;
        for _ in 0..240 {
            for app in [&mut paused, &mut fast] {
                app.update();
                app.world_mut()
                    .resource_mut::<ButtonInput<KeyCode>>()
                    .clear();
            }
            let a = paused.world().resource::<SurfaceState>();
            let b = fast.world().resource::<SurfaceState>();
            assert!(a.feet.distance(b.feet) < 1e-10);
            assert!(a.velocity.distance(b.velocity) < 1e-10);
            assert_eq!(a.grounded, b.grounded);
            peak = peak.max(a.feet.y);
        }
        assert!(peak > 2.0);
        assert!(paused.world().resource::<SurfaceState>().grounded);
        assert!(paused.world().resource::<SurfaceState>().feet.x > 17.9);
        assert_eq!(paused.world().resource::<SimulationClock>().jd, jd);
        assert!(
            paused
                .world()
                .resource::<Explorer>()
                .local_m
                .distance(start)
                < 20.0
        );
        assert!(fast.world().resource::<Explorer>().local_m.distance(start) > 100_000.0);
        // Les vraies touches P/T reprennent le calendrier puis changent sa vitesse.
        press(&mut paused, KeyCode::KeyP);
        press(&mut paused, KeyCode::KeyT);
        assert!(!paused.world().resource::<SimulationClock>().paused);
        assert_eq!(paused.world().resource::<SimulationClock>().speed_index, 1);
        assert!(paused.world().resource::<SimulationClock>().jd > jd);
    }

    #[test]
    fn long_interruption_updates_orbit_and_pose_but_caps_airborne_local_physics() {
        let mut interrupted = app(60);
        let mut capped = app(60);
        for app in [&mut interrupted, &mut capped] {
            press(app, KeyCode::KeyM);
            app.world_mut()
                .resource_mut::<SimulationClock>()
                .speed_index = 3;
            let mut state = app.world_mut().resource_mut::<SurfaceState>();
            state.feet.y += 10.0;
            state.grounded = false;
            state.accumulator = 0.0;
        }
        let jd = interrupted.world().resource::<SimulationClock>().jd;
        let revision = interrupted.world().resource::<MarsTerrain>().revision;
        interrupted.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs(12)));
        capped.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(
            250,
        )));
        interrupted.update();
        capped.update();
        let a = interrupted.world().resource::<SurfaceState>();
        let b = capped.world().resource::<SurfaceState>();
        assert!(!a.grounded);
        assert!(a.feet.distance(b.feet) < 1e-10);
        assert!(a.velocity.distance(b.velocity) < 1e-10);
        let clock = interrupted.world().resource::<SimulationClock>();
        assert!(((clock.jd - jd) * crate::universe::units::DAY - 12_000.0).abs() < 0.001);
        let terrain = interrupted.world().resource::<MarsTerrain>();
        assert_eq!(terrain.revision, revision);
        let solar = interrupted.world().resource::<SolarSystem>();
        let frame = terrain.inertial_frame(
            solar
                .body(BodyId::MARS)
                .rotation(clock.seconds_since_epoch()),
        );
        let pilot = interrupted.world().resource::<Explorer>();
        assert!(pilot.local_m.distance(frame.to_global(a.eye(terrain))) < 1e-9);
        assert_eq!(
            interrupted.world().resource::<RenderOrigin>().0,
            pilot.global(solar, clock.jd)
        );
        interrupted.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(
            1.0 / 60.0,
        )));
        for _ in 0..2 {
            for (phase, key) in [KeyCode::KeyM, KeyCode::KeyG, KeyCode::KeyO, KeyCode::KeyU]
                .into_iter()
                .enumerate()
            {
                interrupted.world_mut().resource_mut::<SimulationClock>().jd += phase as f64 * 0.25;
                press(&mut interrupted, key);
                assert_eq!(
                    interrupted.world().resource::<Explorer>().mode,
                    MovementMode::Space
                );
                press(&mut interrupted, KeyCode::KeyM);
                assert_eq!(
                    interrupted.world().resource::<Explorer>().mode,
                    MovementMode::Surface
                );
            }
        }
    }

    #[test]
    fn cube_pair_keeps_real_controls_clocks_pose_and_all_four_space_exits() {
        let mut app = app(60);
        app.world_mut().resource_mut::<MarsTerrain>().cube_enabled = true;
        press(&mut app, KeyCode::KeyM);
        app.world_mut().resource_mut::<Explorer>().captured = true;
        app.world_mut().resource_mut::<SurfaceState>().yaw = 0.0;
        app.world_mut().resource_mut::<SimulationClock>().paused = true;
        let jd = app.world().resource::<SimulationClock>().jd;
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyD);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Space);
        for _ in 0..240 {
            app.update();
            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .clear();
        }
        assert!(app.world().resource::<SurfaceState>().feet.x > 9.9);
        assert!(app.world().resource::<SurfaceState>().grounded);
        assert_eq!(app.world().resource::<SimulationClock>().jd, jd);
        press(&mut app, KeyCode::KeyP);
        press(&mut app, KeyCode::KeyT);
        let feet = app.world().resource::<SurfaceState>().feet;
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .reset_all();
        app.world_mut().resource_mut::<SurfaceState>().accumulator = 0.0;
        app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(1)));
        app.update();
        assert_eq!(app.world().resource::<SurfaceState>().feet, feet);
        let world = app.world();
        let terrain = world.resource::<MarsTerrain>();
        let solar = world.resource::<SolarSystem>();
        let clock = world.resource::<SimulationClock>();
        let state = world.resource::<SurfaceState>();
        let frame = terrain.inertial_frame(
            solar
                .body(BodyId::MARS)
                .rotation(clock.seconds_since_epoch()),
        );
        assert!(
            world
                .resource::<Explorer>()
                .local_m
                .distance(frame.to_global(state.eye(terrain)))
                < 1e-9
        );
        for key in [KeyCode::KeyM, KeyCode::KeyG, KeyCode::KeyO, KeyCode::KeyU] {
            press(&mut app, key);
            assert_eq!(app.world().resource::<Explorer>().mode, MovementMode::Space);
            press(&mut app, KeyCode::KeyM);
            assert_eq!(
                app.world().resource::<Explorer>().mode,
                MovementMode::Surface
            );
            assert!(app.world().resource::<MarsTerrain>().cube.is_some());
        }
    }

    #[test]
    fn p2_scenario_exits_surface_twice_with_each_navigation_command() {
        let mut app = app(60);
        app.world_mut().resource_mut::<GameConfig>().p2_seconds = Some(160.0);
        app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(
            500,
        )));
        app.add_systems(
            Update,
            crate::config::validation::p2_input
                .before(controls)
                .in_set(GameSet::Input),
        );
        let mut exits = [false; 8];
        for _ in 0..296 {
            app.update();
            let elapsed = app.world().resource::<Time<Real>>().elapsed_secs_f64();
            if (114.0..146.0).contains(&elapsed) {
                let index = ((elapsed - 114.0) / 2.0) as usize;
                let pilot = app.world().resource::<Explorer>();
                if index.is_multiple_of(2) {
                    assert_eq!(pilot.mode, MovementMode::Surface, "entrée {index}");
                } else {
                    assert_eq!(pilot.mode, MovementMode::Space, "sortie {index}");
                    assert_eq!(
                        pilot.anchor,
                        if (index / 2) % 4 < 2 {
                            Some(BodyId::MARS)
                        } else {
                            None
                        }
                    );
                    exits[index / 2] = true;
                }
            }
        }
        assert!(exits.into_iter().all(|seen| seen));
        assert_eq!(
            app.world().resource::<Explorer>().mode,
            MovementMode::Surface
        );
    }
}
