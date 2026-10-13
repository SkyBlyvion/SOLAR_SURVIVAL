//! Parcours reproductible de la paire P3.2 ; touches ECS, pas de périphériques OS.
use crate::{
    GameSet, camera,
    config::GameConfig,
    physics::surface::HALF_WIDTH_M,
    planets::terrain::MarsTerrain,
    player::{Explorer, MovementMode, surface::SurfaceState},
    universe::{SimulationClock, celestial_body::BodyId},
};
use bevy::{
    app::AppExit,
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};

pub(super) fn install(app: &mut App) {
    let config = app.world().resource::<GameConfig>();
    if !config.cube_surface || config.profile.is_some() {
        return;
    }
    let Some(directory) = &config.cube_validation_dir else {
        return;
    };
    std::fs::create_dir(directory)
        .expect("P3.2 exige un nouveau dossier de captures dans un parent existant");
    app.init_resource::<Progress>()
        .add_systems(
            Update,
            input.before(camera::controls).in_set(GameSet::Input),
        )
        .add_systems(Update, capture.after(GameSet::Interface));
}
#[derive(Resource, Default)]
struct Progress {
    stage: Option<usize>,
    captured: Option<usize>,
    previous_x: Option<f64>,
    crossings: usize,
    max_air_m: f64,
    previous_revision: Option<u64>,
    migrations: usize,
    air_migrations: usize,
}
const PHASE_S: f64 = 4.0;
const STAGES: usize = 22;

#[expect(
    clippy::too_many_arguments,
    reason = "Entrées ECS du parcours borné P3.2."
)]
fn input(
    time: Res<Time<Real>>,
    mut progress: ResMut<Progress>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut pilot: ResMut<Explorer>,
    mut state: ResMut<SurfaceState>,
    mut terrain: ResMut<MarsTerrain>,
    mut clock: ResMut<SimulationClock>,
    mut exit: MessageWriter<AppExit>,
) {
    let elapsed = time.elapsed_secs_f64();
    if elapsed >= PHASE_S * STAGES as f64 {
        if terrain.migration_enabled {
            assert!(
                progress.migrations >= 8 && progress.air_migrations >= 1,
                "Migrations {} / en l'air {}",
                progress.migrations,
                progress.air_migrations
            );
            info!(
                "{} terminé : {} migrations, {} en l'air, apex {:.6} m",
                if terrain.offcenter_enabled {
                    "P3.4"
                } else {
                    "P3.3"
                },
                progress.migrations,
                progress.air_migrations,
                progress.max_air_m
            );
        }
        assert!(
            progress.crossings >= 6 || terrain.migration_enabled,
            "Franchissements réellement observés : {}",
            progress.crossings
        );
        assert!(progress.max_air_m > 1.9, "Saut réellement observé");
        if !terrain.migration_enabled {
            info!(
                "P3.2 terminé : {} franchissements, apex {:.6} m",
                progress.crossings, progress.max_air_m
            );
        }
        keys.reset_all();
        exit.write(AppExit::Success);
        return;
    }
    let stage = (elapsed / PHASE_S) as usize;
    keys.reset_all();
    // Phase initiale sans capture : la fenêtre X11 doit devenir visible
    // avant de demander le confinement du curseur.
    pilot.captured = (1..=12).contains(&stage);
    if progress.stage != Some(stage) {
        progress.previous_x = None;
        if terrain.migration_enabled && matches!(stage, 1 | 3 | 6 | 9) {
            if let Some(plan) = &terrain.migration {
                state.feet = plan.approach(terrain.cube.as_ref().unwrap().pair.keys[0]);
            } else {
                state.feet.x = 96.0;
                state.feet.z = 30.0;
            }
            state.feet.y = terrain.support_height(state.feet, HALF_WIDTH_M) + 0.002;
            state.velocity = bevy::math::DVec3::ZERO;
            state.grounded = true;
        }
        match stage {
            0 => {
                clock.jd = crate::universe::units::J2000;
                clock.speed_index = 0;
                clock.paused = true;
                keys.press(KeyCode::KeyM);
            }
            5 | 8 => {
                if let Some(plan) = &terrain.migration {
                    state.feet = plan.approach(terrain.cube.as_ref().unwrap().pair.keys[0]);
                } else {
                    state.feet.x = if terrain.migration_enabled { 96.0 } else { 0.0 };
                }
                state.feet.y = terrain.support_height(state.feet, HALF_WIDTH_M) + 0.002;
                state.velocity = bevy::math::DVec3::ZERO;
                state.grounded = true;
            }
            6 => {
                if !terrain.migration_enabled {
                    state.feet.x = -4.0;
                }
                keys.press(KeyCode::Space);
            }
            9 => {
                keys.press(KeyCode::KeyP);
                clock.speed_index = 3;
            }
            11 | 12 => {
                if terrain.migration_enabled {
                    if let Some(plan) = &terrain.migration {
                        state.feet = plan.preparation(terrain.cube.as_ref().unwrap().pair.keys[0]);
                    } else {
                        state.feet.x = if stage == 11 { 101.0 } else { -111.0 };
                    }
                    assert!(
                        state.migrate(&mut terrain),
                        "Préparation explicite de limite"
                    );
                }
                let cube = terrain.cube.as_ref().unwrap();
                state.feet.x = if stage == 11 {
                    cube.max.x - 2.0
                } else {
                    cube.min.x + 2.0
                };
                state.feet.y = terrain.support_height(state.feet, HALF_WIDTH_M) + 0.002;
            }
            13 | 15 | 17 | 19 => keys.press(
                [KeyCode::KeyM, KeyCode::KeyG, KeyCode::KeyO, KeyCode::KeyU][(stage - 13) / 2],
            ),
            14 | 16 | 18 | 20 => keys.press(KeyCode::KeyM),
            21 => {
                pilot.selected = BodyId::EARTH;
                keys.press(KeyCode::KeyG);
            }
            _ => {}
        }
        progress.stage = Some(stage);
    }
    if stage <= 12 {
        state.yaw = 0.0;
        state.pitch = -0.35;
        match stage {
            1 | 3 | 6 | 9 | 11 => keys.press(KeyCode::KeyD),
            2 | 4 | 7 | 10 | 12 => keys.press(KeyCode::KeyA),
            _ => {}
        }
        if matches!(stage, 3 | 4 | 6 | 7 | 9..=12) {
            keys.press(KeyCode::ShiftLeft);
        }
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "Preuves ECS/graphique du parcours P3.2."
)]
fn capture(
    time: Res<Time<Real>>,
    config: Res<GameConfig>,
    mut progress: ResMut<Progress>,
    pilot: Res<Explorer>,
    state: Res<SurfaceState>,
    terrain: Res<MarsTerrain>,
    clock: Res<SimulationClock>,
    mut commands: Commands,
) {
    let elapsed = time.elapsed_secs_f64();
    let stage = ((elapsed / PHASE_S) as usize).min(STAGES - 1);
    if pilot.mode == MovementMode::Surface {
        let floor = terrain.support_height(state.feet, HALF_WIDTH_M);
        assert!(state.feet.is_finite() && state.feet.y >= floor - 1e-8);
        if matches!(stage, 1..=4 | 6 | 7 | 9 | 10)
            && terrain.migration_enabled
            && let Some(revision) = progress.previous_revision
            && revision != terrain.revision
        {
            assert_eq!(terrain.revision, revision + 1);
            progress.migrations += 1;
            progress.air_migrations += (!state.grounded) as usize;
            info!(
                "{} migration {} : pieds {:?}, sol {}",
                if terrain.offcenter_enabled {
                    "P3.4"
                } else {
                    "P3.3"
                },
                progress.migrations,
                state.feet,
                state.grounded
            );
        }
        progress.previous_revision = Some(terrain.revision);
        if stage == 6 {
            progress.max_air_m = progress.max_air_m.max(state.feet.y - floor);
        }
        if matches!(stage, 1..=4 | 6 | 7 | 9 | 10) && !terrain.migration_enabled {
            if let Some(previous) = progress.previous_x
                && previous * state.feet.x < 0.0
            {
                progress.crossings += 1;
            }
            progress.previous_x = Some(state.feet.x);
        }
    }
    let fraction = if stage == 6 { 0.25 } else { 0.75 };
    if elapsed % PHASE_S < PHASE_S * fraction || progress.captured == Some(stage) {
        return;
    }
    progress.captured = Some(stage);
    info!(
        "{} étape {stage:02} : {:?}, pieds {:?}, vitesse {:?}, sol {}, JD {:.8}, pause {}, crossings {}, migrations {}",
        if terrain.offcenter_enabled {
            "P3.4"
        } else if terrain.migration_enabled {
            "P3.3"
        } else {
            "P3.2"
        },
        pilot.mode,
        state.feet,
        state.velocity,
        state.grounded,
        clock.jd,
        clock.paused,
        progress.crossings,
        progress.migrations
    );
    if let Some(plan) = &terrain.migration {
        info!(
            "P3.4 constructions CPU {}, dérive boîte {:.9} m",
            plan.constructions, plan.box_drift_m
        );
    }
    if let Some(cube) = &terrain.cube {
        info!(
            "Paire : {:?}, {} sommets, {} triangles, index {:?}, bornes {:?} / {:?}",
            cube.pair.keys,
            cube.local_positions.len(),
            cube.pair.indices.iter().map(|i| i.len() / 3).sum::<usize>(),
            cube.index_counts(),
            cube.min,
            cube.max
        );
    }
    commands
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(
            config
                .cube_validation_dir
                .as_ref()
                .unwrap()
                .join(format!("crossing-{stage:02}.png")),
        ));
}
