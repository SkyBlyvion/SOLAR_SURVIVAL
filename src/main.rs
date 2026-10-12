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
                    file_path: concat!(env!("CARGO_MANIFEST_DIR"), "/assets").into(),
                    ..default()
                }),
        )
        .add_plugins(SolarSurvivalPlugin)
        .add_plugins(ValidationPlugin)
        .add_plugins(ProfilingPlugin)
        .run();
}
