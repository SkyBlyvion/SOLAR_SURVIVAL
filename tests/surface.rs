use bevy::{
    math::{DQuat, DVec3},
    prelude::*,
};
use solar_survival::{
    physics::surface::{self, HALF_WIDTH_M, STEP_SECONDS, SolidBox},
    planets::terrain::{MarsTerrain, SITE_LIMIT_M},
    player::{Explorer, surface::SurfaceState},
    universe::{catalog::SolarSystem, celestial_body::BodyId, units::J2000},
};

fn terrain() -> MarsTerrain {
    MarsTerrain {
        radius_m: 3389500.0,
        seed: 42,
        orientation: DQuat::IDENTITY,
        revision: 1,
    }
}
fn mu() -> f64 {
    SolarSystem::default()
        .body(BodyId::MARS)
        .gravitational_parameter
}
fn settle(
    terrain: &MarsTerrain,
    feet: &mut DVec3,
    velocity: &mut DVec3,
    grounded: &mut bool,
    steps: usize,
) {
    for _ in 0..steps {
        surface::step(terrain, feet, velocity, grounded, DVec3::ZERO, false, mu());
    }
}

#[test]
fn falls_twenty_metres_and_stays_on_the_triangle_surface() {
    let terrain = terrain();
    let mut feet = DVec3::new(20.0, 20.0, 20.0);
    let mut velocity = DVec3::ZERO;
    let mut grounded = false;
    settle(&terrain, &mut feet, &mut velocity, &mut grounded, 600);
    assert!(grounded);
    assert!((feet.y - terrain.support_height(feet, HALF_WIDTH_M) - 0.002).abs() < 1e-8);
    let rest = feet;
    settle(&terrain, &mut feet, &mut velocity, &mut grounded, 1200);
    assert!(feet.distance(rest) < 1e-6);
    assert!(velocity.length() < 1e-6);
}

#[test]
fn jump_has_martian_height_and_lands_without_penetration() {
    let terrain = terrain();
    let mut feet = DVec3::new(
        0.0,
        terrain.support_height(DVec3::ZERO, HALF_WIDTH_M) + 0.002,
        0.0,
    );
    let floor = feet.y;
    let mut velocity = DVec3::ZERO;
    let mut grounded = true;
    let mut peak: f64 = 0.0;
    for tick in 0..360 {
        surface::step(
            &terrain,
            &mut feet,
            &mut velocity,
            &mut grounded,
            DVec3::ZERO,
            tick == 0,
            mu(),
        );
        peak = peak.max(feet.y - floor);
        assert!(feet.y + 1e-9 >= terrain.support_height(feet, HALF_WIDTH_M));
    }
    assert!((2.0..2.2).contains(&peak), "apex {peak}");
    assert!(grounded);
    assert!((feet.y - floor).abs() < 1e-5);
}

#[test]
fn swept_player_volume_blocks_walls_and_slides_along_them() {
    let wall = SolidBox {
        min: DVec3::new(0.0, -5.0, -100.0),
        max: DVec3::new(2.0, 5.0, 100.0),
    };
    let (feet, velocity, _) = surface::move_and_slide(
        DVec3::new(-10.0, 0.0, 0.0),
        DVec3::new(100.0, 0.0, 10.0),
        DVec3::new(100.0, 0.0, 10.0),
        &[wall],
    );
    assert!(feet.x <= -HALF_WIDTH_M);
    assert!((feet.z - 10.0).abs() < 0.01);
    assert_eq!(velocity.x, 0.0);
}

#[test]
fn swept_volume_handles_box_tops_and_ceiling() {
    let box_ = SolidBox {
        min: DVec3::new(-2.0, 3.0, -2.0),
        max: DVec3::new(2.0, 4.0, 2.0),
    };
    let (feet, velocity, grounded) = surface::move_and_slide(
        DVec3::new(0.0, 10.0, 0.0),
        DVec3::new(0.0, -100.0, 0.0),
        DVec3::new(0.0, -100.0, 0.0),
        &[box_],
    );
    assert!(feet.y >= 4.0 && feet.y < 4.01);
    assert!(grounded);
    assert_eq!(velocity.y, 0.0);
    let (feet, velocity, _) =
        surface::move_and_slide(DVec3::ZERO, DVec3::Y * 10.0, DVec3::Y * 10.0, &[box_]);
    assert!(feet.y + surface::PLAYER_HEIGHT_M <= 3.0);
    assert_eq!(velocity.y, 0.0);
}

fn walk(fps: usize, boost: bool) -> DVec3 {
    let solar = SolarSystem::default();
    let mut terrain = terrain();
    let mut pilot = Explorer::default();
    let mut state = SurfaceState::default();
    state.enter(&mut pilot, &mut terrain, &solar, J2000);
    pilot.captured = true;
    pilot.boosted = boost;
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyD);
    for _ in 0..2 * fps {
        state.advance(&mut pilot, &terrain, &keys, 1.0 / fps as f64, mu());
        keys.clear();
    }
    assert!(state.grounded);
    state.feet
}
#[test]
fn surface_walk_and_run_are_independent_of_frame_rate() {
    let a = walk(30, false);
    let b = walk(60, false);
    let c = walk(144, false);
    assert!(a.distance(b) < 1e-8 && b.distance(c) < 1e-8);
    assert!((a.x - 9.0).abs() < 0.02);
    assert!((walk(144, true).x - 16.0).abs() < 0.02);
}

#[test]
fn footprint_contact_is_continuous_across_negative_grid_boundaries() {
    let terrain = terrain();
    for x in [-128.0, -4.0, 0.0, 4.0, 128.0] {
        let a = terrain.support_height(DVec3::new(x - 1e-6, 0.0, 2.7), HALF_WIDTH_M);
        let b = terrain.support_height(DVec3::new(x + 1e-6, 0.0, 2.7), HALF_WIDTH_M);
        assert!((a - b).abs() < 1e-6);
        assert!(a >= terrain.height(x - 1e-6, 2.7) - 1e-8);
    }
}

#[test]
fn radial_up_and_eye_height_follow_an_arbitrarily_oriented_mars_site() {
    let mut terrain = terrain();
    terrain.set_site(DVec3::new(1.0, 0.2, -0.8).normalize());
    let state = SurfaceState {
        feet: DVec3::new(500.0, 2.0, 300.0),
        ..default()
    };
    let mut pilot = Explorer::default();
    state.sync(&mut pilot, &terrain);
    let feet = terrain.body_relative(state.feet);
    assert!((pilot.local_m.distance(feet) - surface::EYE_HEIGHT_M).abs() < 1e-8);
    assert!((pilot.rotation * DVec3::Y).distance(feet.normalize()) < 1e-12);
    assert!(terrain.chart(feet).distance(state.feet) < 1e-8);
}

#[test]
fn release_stops_input_but_gravity_continues_and_long_stalls_are_bounded() {
    let solar = SolarSystem::default();
    let mut terrain = terrain();
    let mut pilot = Explorer::default();
    let mut state = SurfaceState::default();
    state.enter(&mut pilot, &mut terrain, &solar, J2000);
    state.feet.y += 10.0;
    state.grounded = false;
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyW);
    keys.press(KeyCode::Space);
    state.advance(&mut pilot, &terrain, &keys, 100.0, mu());
    assert!(state.feet.y > 9.0);
    assert!(state.velocity.y < 0.0);
    for _ in 0..600 {
        state.advance(&mut pilot, &terrain, &keys, STEP_SECONDS, mu());
    }
    assert!(state.grounded);
    assert!(state.feet.z.abs() < 0.001);
}

#[test]
fn bounded_site_cannot_leave_supported_terrain() {
    let terrain = terrain();
    let mut feet = DVec3::new(SITE_LIMIT_M - 0.31, 30.0, 0.0);
    let mut velocity = DVec3::ZERO;
    let mut grounded = false;
    for _ in 0..120 {
        surface::step(
            &terrain,
            &mut feet,
            &mut velocity,
            &mut grounded,
            DVec3::X * surface::RUN_MPS,
            false,
            mu(),
        );
    }
    assert!(feet.x <= SITE_LIMIT_M - HALF_WIDTH_M);
    assert!(feet.is_finite());
}

#[test]
fn walking_into_an_obstacle_reports_zero_resolved_forward_velocity() {
    let solar = SolarSystem::default();
    let mut terrain = terrain();
    let mut pilot = Explorer::default();
    let mut state = SurfaceState::default();
    state.enter(&mut pilot, &mut terrain, &solar, J2000);
    pilot.captured = true;
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::KeyW);
    for _ in 0..600 {
        state.advance(&mut pilot, &terrain, &keys, STEP_SECONDS, mu());
        keys.clear();
    }
    assert!(
        state.feet.z >= -10.7 && state.feet.z < -10.69,
        "{:?}",
        state.feet
    );
    assert!(state.velocity.z.abs() < 1e-8, "{:?}", state.velocity);
}

#[test]
fn box_top_rest_is_stable_and_can_jump_on_every_physics_tick() {
    let terrain = terrain();
    let top = surface::obstacles(&terrain)[0].max.y;
    let mut feet = DVec3::new(0.0, top + 10.0, -12.0);
    let mut velocity = DVec3::ZERO;
    let mut grounded = false;
    settle(&terrain, &mut feet, &mut velocity, &mut grounded, 600);
    for _ in 0..240 {
        surface::step(
            &terrain,
            &mut feet,
            &mut velocity,
            &mut grounded,
            DVec3::ZERO,
            false,
            mu(),
        );
        assert!(grounded);
        assert!((feet.y - top - 0.002).abs() < 1e-7);
    }
    surface::step(
        &terrain,
        &mut feet,
        &mut velocity,
        &mut grounded,
        DVec3::ZERO,
        true,
        mu(),
    );
    assert!(!grounded);
    assert!(velocity.y > 3.9);
}

#[test]
fn jump_pressed_between_two_physics_ticks_is_not_lost_or_repeated() {
    let solar = SolarSystem::default();
    let mut terrain = terrain();
    let mut pilot = Explorer::default();
    let mut state = SurfaceState::default();
    state.enter(&mut pilot, &mut terrain, &solar, J2000);
    pilot.captured = true;
    let mut keys = ButtonInput::<KeyCode>::default();
    keys.press(KeyCode::Space);
    state.advance(&mut pilot, &terrain, &keys, STEP_SECONDS * 0.5, mu());
    assert!(state.jump_pending && state.grounded);
    keys.clear();
    state.advance(&mut pilot, &terrain, &keys, STEP_SECONDS * 0.5, mu());
    assert!(!state.grounded && !state.jump_pending && state.velocity.y > 3.9);
    for _ in 0..480 {
        state.advance(&mut pilot, &terrain, &keys, STEP_SECONDS, mu());
    }
    assert!(state.grounded && state.velocity.length() < 1e-6);
}
