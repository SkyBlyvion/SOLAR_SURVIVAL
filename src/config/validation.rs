//! Scénarios graphiques optionnels ; jamais activés pendant une partie normale.
use crate::{
    GameSet,
    camera::NavigationRequest,
    config::GameConfig,
    player::{Explorer, surface::SurfaceState},
    universe::{SimulationClock, celestial_body::BodyId},
};
use bevy::{
    app::AppExit,
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};
pub struct ValidationPlugin;
impl Plugin for ValidationPlugin {
    fn build(&self, app: &mut App) {
        super::base_validation::install(app);
        super::persistence_validation::install(app);
        super::sky_validation::install(app);
        super::construction_validation::install(app);
        super::survival_validation::install(app);
        super::cube_validation::install(app);
        super::globe_validation::install(app);
        if let Some(directory) = std::env::var_os("SOLAR_CAPTURE_DIR") {
            let directory = std::path::PathBuf::from(directory);
            app.add_systems(
                Last,
                move |keys: Res<ButtonInput<KeyCode>>,
                      mut commands: Commands,
                      mut index: Local<u32>| {
                    if keys.just_pressed(KeyCode::F12) {
                        commands
                            .spawn(Screenshot::primary_window())
                            .observe(save_to_disk(
                                directory.join(format!("capture-{:02}.png", *index)),
                            ));
                        *index += 1;
                    }
                },
            );
        }
        app.add_systems(
            Update,
            (smoke_test, surface_smoke)
                .chain()
                .in_set(GameSet::Input)
                .after(crate::ui::HudInputSet),
        );
        app.add_systems(
            Update,
            p2_input
                .before(crate::camera::controls)
                .in_set(GameSet::Input),
        )
        .add_systems(Update, p2_capture.after(GameSet::Interface));
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
    mut clock: ResMut<SimulationClock>,
    mut commands: Commands,
    mut exit: MessageWriter<AppExit>,
) {
    if config.profile.is_some()
        || config.p2_seconds.is_some()
        || ((config.cube_surface && config.cube_validation_dir.is_some())
            || (config.mars_global && config.globe_validation_dir.is_some()))
    {
        return;
    }
    let Some(seconds) = config
        .rotation_smoke_seconds
        .or(config.surface_smoke_seconds)
    else {
        return;
    };
    let elapsed = time.elapsed_secs_f64();
    let duration = seconds / 10.0;
    let stage = ((elapsed / duration) as usize).min(9);
    keys.reset_all();
    pilot.captured = stage < 7;
    pilot.boosted = (4..=6).contains(&stage);
    if progress.stage != Some(stage) {
        if config.rotation_smoke_seconds.is_some() {
            match stage {
                0 | 4 | 6 => {
                    clock.speed_index = 3;
                    clock.paused = false;
                }
                3 | 7 => clock.paused = true,
                5 | 8 => {
                    clock.speed_index = 0;
                    clock.paused = false;
                }
                _ => {}
            }
        }
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
            "Validation surface étape {} / 10 : pieds {:?}, vitesse {:?}, au sol {}, JD {:.8}, temps ×{} pause {}",
            stage + 1,
            surface.feet,
            surface.velocity,
            surface.grounded,
            clock.jd,
            SimulationClock::SPEEDS[clock.speed_index],
            clock.paused
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
        let prefix = if config.rotation_smoke_seconds.is_some() {
            "rotation"
        } else {
            "v02"
        };
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!(
                "docs/{prefix}-surface-{stage:02}.png"
            )));
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
    if ((config.cube_surface && config.cube_validation_dir.is_some())
        || (config.mars_global && config.globe_validation_dir.is_some()))
        || config.profile.is_some()
        || config.p2_seconds.is_some()
        || config.surface_smoke_seconds.is_some()
        || config.rotation_smoke_seconds.is_some()
    {
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

/// 96 s à ×1000 dépassent une rotation de Mars (≈88,64 s réelles).
/// Puis P/T via les vrais contrôles, et deux séries de sorties M/G/O/U.
fn p2_stage(elapsed: f64) -> (usize, f64, f64) {
    if elapsed < 96.0 {
        let stage = (elapsed / 8.0) as usize;
        (stage, elapsed - stage as f64 * 8.0, 8.0)
    } else if elapsed < 114.0 {
        let index = ((elapsed - 96.0) / 3.0) as usize;
        (12 + index, elapsed - 96.0 - index as f64 * 3.0, 3.0)
    } else if elapsed < 146.0 {
        let index = ((elapsed - 114.0) / 2.0) as usize;
        (18 + index, elapsed - 114.0 - index as f64 * 2.0, 2.0)
    } else {
        (34, elapsed - 146.0, 8.0)
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "Paramètres ECS du scénario borné P2."
)]
pub(crate) fn p2_input(
    config: Res<GameConfig>,
    time: Res<Time<Real>>,
    mut progress: Local<Option<usize>>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut pilot: ResMut<Explorer>,
    mut surface: ResMut<SurfaceState>,
    mut clock: ResMut<SimulationClock>,
    mut exit: MessageWriter<AppExit>,
) {
    if (config.cube_surface && config.cube_validation_dir.is_some())
        || (config.mars_global && config.globe_validation_dir.is_some())
    {
        return;
    }
    let Some(seconds) = config.p2_seconds.filter(|_| config.profile.is_none()) else {
        return;
    };
    keys.reset_all();
    let elapsed = time.elapsed_secs_f64();
    let (stage, _, _) = p2_stage(elapsed);
    pilot.captured = stage < 18;
    if *progress != Some(stage) {
        match stage {
            0 => {
                clock.jd = crate::universe::units::J2000;
                clock.speed_index = 3;
                clock.paused = false;
                keys.press(KeyCode::KeyM);
            }
            12 => keys.press(KeyCode::KeyP),
            13 => keys.press(KeyCode::Space),
            14 => {
                keys.press(KeyCode::KeyP);
                keys.press(KeyCode::KeyT);
            }
            15..=17 => keys.press(KeyCode::KeyT),
            // La première paire part du site conservé depuis le cycle ;
            // les suivantes rentrent depuis l'espace avant leur sortie.
            19..=33 => {
                let index = stage - 18;
                keys.press(if index % 2 == 0 {
                    KeyCode::KeyM
                } else {
                    [KeyCode::KeyM, KeyCode::KeyG, KeyCode::KeyO, KeyCode::KeyU][(index / 2) % 4]
                });
            }
            34 => keys.press(KeyCode::KeyM),
            _ => {}
        }
        *progress = Some(stage);
    }
    if stage == 12 {
        keys.press(KeyCode::KeyD);
    }
    if stage < 12 {
        surface.pitch = -0.12;
    }
    if time.delta_secs_f64() > 1.0 {
        info!(
            "P2 reprise après interruption : delta réel {:.3} s ; physique locale plafonnée à 0,25 s",
            time.delta_secs_f64()
        );
    }
    if elapsed >= seconds {
        keys.reset_all();
        exit.write(AppExit::Success);
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "Diagnostic et capture ECS du scénario P2."
)]
fn p2_capture(
    config: Res<GameConfig>,
    time: Res<Time<Real>>,
    mut captured: Local<Option<usize>>,
    pilot: Res<Explorer>,
    surface: Res<SurfaceState>,
    terrain: Res<crate::planets::terrain::MarsTerrain>,
    solar: Res<crate::universe::catalog::SolarSystem>,
    clock: Res<SimulationClock>,
    mut commands: Commands,
) {
    if config.p2_seconds.is_none()
        || config.profile.is_some()
        || ((config.cube_surface && config.cube_validation_dir.is_some())
            || (config.mars_global && config.globe_validation_dir.is_some()))
    {
        return;
    }
    let (stage, phase, duration) = p2_stage(time.elapsed_secs_f64());
    let capture_fraction = if stage == 13 { 0.25 } else { 0.75 };
    if phase < duration * capture_fraction || *captured == Some(stage) {
        return;
    }
    *captured = Some(stage);
    let frame = terrain.inertial_frame(
        solar
            .body(BodyId::MARS)
            .rotation(clock.seconds_since_epoch()),
    );
    let sun = frame.orientation.inverse() * (-pilot.global(&solar, clock.jd)).normalize();
    info!(
        "P2 étape {stage:02} : mode {:?}, JD {:.8}, ×{}, pause {}, Soleil local {:?}, pieds {:?}, vitesse {:?}, sol {}, révision {}",
        pilot.mode,
        clock.jd,
        SimulationClock::SPEEDS[clock.speed_index],
        clock.paused,
        sun,
        surface.feet,
        surface.velocity,
        surface.grounded,
        terrain.revision
    );
    commands
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(format!("docs/p2-stage-{stage:02}.png")));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p2_cycle_exceeds_rotation_before_pause_and_repeats_all_exits() {
        let solar = crate::universe::catalog::SolarSystem::default();
        assert!(96_000.0 > solar.body(BodyId::MARS).rotation_period_s);
        assert_eq!(p2_stage(95.9).0, 11);
        assert_eq!(p2_stage(96.0).0, 12);
        assert_eq!(p2_stage(114.0).0, 18);
        assert_eq!(p2_stage(145.9).0, 33);
        assert_eq!(p2_stage(146.0).0, 34);
    }
}
