use bevy::{
    app::AppExit,
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
    window::PresentMode,
};
use solar_survival::{
    GameSet, SolarSurvivalPlugin, camera::NavigationRequest, config::GameConfig, player::Explorer,
    universe::celestial_body::BodyId,
};

fn main() {
    App::new()
        .insert_resource(GameConfig::default())
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Solar Survival · 0.1".into(),
                        resolution: (1440, 900).into(),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: concat!(env!("CARGO_MANIFEST_DIR"), "/assets").into(),
                    ..default()
                }),
        )
        .add_plugins(SolarSurvivalPlugin)
        .add_systems(
            Update,
            smoke_test
                .in_set(GameSet::Input)
                .after(solar_survival::ui::HudInputSet),
        )
        .run();
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
            "docs/screenshot.png".to_owned()
        } else {
            format!("docs/smoke-{stage:02}.png")
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
