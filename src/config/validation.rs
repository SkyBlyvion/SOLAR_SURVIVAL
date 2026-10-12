//! Scénarios graphiques optionnels ; jamais activés pendant une partie normale.
use crate::{
    GameSet,
    camera::NavigationRequest,
    config::GameConfig,
    player::{Explorer, surface::SurfaceState},
    universe::celestial_body::BodyId,
};
use bevy::{
    app::AppExit,
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};
pub struct ValidationPlugin;
impl Plugin for ValidationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (smoke_test, surface_smoke)
                .chain()
                .in_set(GameSet::Input)
                .after(crate::ui::HudInputSet),
        );
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "Paramètres de système injectés par Bevy ECS."
)]
fn surface_smoke(
    time: Res<Time<Real>>,
    config: Res<GameConfig>,
    mut progress: Local<SmokeProgress>,
    mut pilot: ResMut<Explorer>,
    mut surface: ResMut<SurfaceState>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut navigation: ResMut<NavigationRequest>,
    mut commands: Commands,
    mut exit: MessageWriter<AppExit>,
) {
    let Some(seconds) = config.surface_smoke_seconds else {
        return;
    };
    let elapsed = time.elapsed_secs_f64();
    let duration = seconds / 10.0;
    let stage = ((elapsed / duration) as usize).min(9);
    keys.reset_all();
    pilot.captured = stage < 7;
    pilot.boosted = (4..=6).contains(&stage);
    if progress.stage != Some(stage) {
        match stage {
            0 | 8 => navigation.surface = true,
            2 => keys.press(KeyCode::Space),
            9 => {
                pilot.selected = BodyId::EARTH;
                navigation.visit = true;
            }
            _ => {}
        }
        progress.stage = Some(stage);
        info!(
            "Validation surface étape {} / 10 : pieds {:?}, vitesse {:?}, au sol {}",
            stage + 1,
            surface.feet,
            surface.velocity,
            surface.grounded
        );
    }
    match stage {
        1 | 2 | 4..=6 => keys.press(KeyCode::KeyW),
        3 => keys.press(KeyCode::KeyD),
        _ => {}
    }
    if stage == 0 {
        surface.pitch = -0.12;
    }
    let capture_fraction = if stage == 2 { 0.15 } else { 0.7 };
    if elapsed % duration >= duration * capture_fraction && progress.captured_stage != Some(stage) {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!("docs/v02-surface-{stage:02}.png")));
        progress.captured_stage = Some(stage);
    }
    if elapsed >= seconds {
        keys.reset_all();
        exit.write(AppExit::Success);
    }
}
/// Option de validation explicite : visite les dix corps puis la vue d'ensemble.
#[derive(Default)]
struct SmokeProgress {
    stage: Option<usize>,
    captured_stage: Option<usize>,
}
fn smoke_test(
    time: Res<Time<Real>>,
    config: Res<GameConfig>,
    mut progress: Local<SmokeProgress>,
    mut pilot: ResMut<Explorer>,
    mut navigation: ResMut<NavigationRequest>,
    mut commands: Commands,
    mut exit: MessageWriter<AppExit>,
) {
    if config.surface_smoke_seconds.is_some() {
        return;
    }
    let Some(seconds) = config.smoke_seconds else {
        return;
    };
    let elapsed = time.elapsed_secs_f64();
    let duration = seconds / 11.0;
    let stage = ((elapsed / duration) as usize).min(10);
    let order = [3, 4, 5, 6, 7, 8, 9, 1, 2, 0];
    if progress.stage != Some(stage) {
        if stage < 10 {
            pilot.selected = BodyId(order[stage]);
            navigation.visit = true;
        } else {
            navigation.overview = true;
        }
        progress.stage = Some(stage);
        info!("Validation graphique : étape {} / 11", stage + 1);
    }
    if elapsed % duration >= duration * 0.65 && progress.captured_stage != Some(stage) {
        let path = if stage == 0 {
            "docs/v02-space-00.png".to_owned()
        } else {
            format!("docs/v02-space-{stage:02}.png")
        };
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
        progress.captured_stage = Some(stage);
    }
    if elapsed >= seconds {
        exit.write(AppExit::Success);
    }
}
