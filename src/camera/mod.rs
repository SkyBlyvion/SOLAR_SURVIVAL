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
            surface.look(motion.delta);
        } else {
            let (yaw, pitch, _) = pilot.rotation.to_euler(EulerRot::YXZ);
            pilot.rotation = DQuat::from_euler(
                EulerRot::YXZ,
                yaw - motion.delta.x as f64 * 0.002,
                (pitch - motion.delta.y as f64 * 0.002).clamp(-1.55, 1.55),
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
            &mut pilot,
            &terrain,
            &keys,
            time.delta_secs_f64(),
            solar.body(BodyId::MARS).gravitational_parameter,
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
            press(&mut app, KeyCode::KeyM);
            let pilot = app.world().resource::<Explorer>();
            assert_eq!(pilot.mode, MovementMode::Surface);
            assert_eq!(pilot.anchor, Some(BodyId::MARS));
            assert!(pilot.local_m.length() < 3_389_510.0);
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
}
