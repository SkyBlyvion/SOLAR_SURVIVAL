use bevy::{prelude::*, window::PresentMode};
use solar_survival::{
    SolarSurvivalPlugin,
    config::{GameConfig, profiling::ProfilingPlugin, validation::ValidationPlugin},
};

fn main() {
    let config = GameConfig::default();
    let resolution = if config
        .profile
        .as_ref()
        .is_some_and(|p| p.variant == "half_resolution")
    {
        (720, 450)
    } else if config.persistence_validation_dir.is_some()
        && std::env::var_os("SOLAR_P6_SMALL_WINDOW").is_some()
    {
        (1280, 720)
    } else {
        (1440, 900)
    };
    App::new()
        .insert_resource(config)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Solar Survival · 0.2 · Mars".into(),
                        resolution: resolution.into(),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: solar_survival::config::asset_directory()
                        .to_string_lossy()
                        .into_owned(),
                    ..default()
                }),
        )
        .add_plugins(SolarSurvivalPlugin)
        .add_plugins(ValidationPlugin)
        .add_plugins(ProfilingPlugin)
        .run();
}
