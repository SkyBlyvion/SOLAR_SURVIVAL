use crate::{
    GameSet,
    config::GameConfig,
    physics::stop_before_sphere,
    player::Explorer,
    universe::{
        SimulationClock, catalog::SolarSystem, celestial_body::BodyId, coordinates::RenderOrigin,
    },
};
use bevy::{
    camera::Hdr,
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    math::DVec3,
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
}
pub struct ExplorationPlugin;
impl Plugin for ExplorationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Explorer>()
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
        Transform::from_rotation(pilot.rotation),
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
        let (yaw, pitch, _) = pilot.rotation.to_euler(EulerRot::YXZ);
        pilot.rotation = Quat::from_euler(
            EulerRot::YXZ,
            yaw - motion.delta.x * 0.002,
            (pitch - motion.delta.y * 0.002).clamp(-1.55, 1.55),
            0.0,
        );
        let change = if scroll.delta.y > 0.0 {
            1
        } else if scroll.delta.y < 0.0 {
            -1
        } else {
            0
        };
        pilot.speed_index = (pilot.speed_index as i32 + change).clamp(0, 5) as usize;
    }
    if keys.just_pressed(KeyCode::Equal) || keys.just_pressed(KeyCode::PageUp) {
        pilot.speed_index = (pilot.speed_index + 1).min(5);
    }
    if keys.just_pressed(KeyCode::Minus) || keys.just_pressed(KeyCode::PageDown) {
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
fn navigate(
    time: Res<Time<Real>>,
    keys: Res<ButtonInput<KeyCode>>,
    solar: Res<SolarSystem>,
    clock: Res<SimulationClock>,
    mut pilot: ResMut<Explorer>,
    mut request: ResMut<NavigationRequest>,
    mut origin: ResMut<RenderOrigin>,
    mut camera: Single<&mut Transform, With<SpaceCamera>>,
) {
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
    if request.center {
        let direction = solar.position(pilot.selected, clock.jd) - pilot.global(&solar, clock.jd);
        pilot.look_toward(direction);
    }
    *request = NavigationRequest::default();
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
    let mut input = Vec3::ZERO;
    if pilot.captured {
        input.x = (keys.pressed(KeyCode::KeyD) as i32 - keys.pressed(KeyCode::KeyA) as i32) as f32;
        input.z = (keys.pressed(KeyCode::KeyS) as i32 - keys.pressed(KeyCode::KeyW) as i32) as f32;
        input.y = (keys.pressed(KeyCode::Space) as i32
            - (keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight)) as i32)
            as f32;
    }
    let delta = (pilot.rotation * input.normalize_or_zero()).as_dvec3()
        * pilot.speed()
        * time.delta_secs_f64();
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
    origin.0 = pilot.global(&solar, clock.jd);
    camera.translation = Vec3::ZERO;
    camera.rotation = pilot.rotation;
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
}
