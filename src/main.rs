use bevy::{prelude::*, window::PresentMode};
use solar_survival::{
    SolarSurvivalPlugin,
    config::{GameConfig, validation::ValidationPlugin},
};

fn main() {
    App::new()
        .insert_resource(GameConfig::default())
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Solar Survival · 0.2 · Mars".into(),
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
        .add_plugins(ValidationPlugin)
        .run();
}
