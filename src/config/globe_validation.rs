//! Une campagne fonctionnelle globale : six faces, arêtes, saut et streaming.
use crate::{
    GameSet, camera,
    config::GameConfig,
    physics::surface::{self, HALF_WIDTH_M},
    planets::{CubeFace, globe_terrain::GlobeTerrain, terrain::MarsTerrain},
    player::{Explorer, MovementMode, surface::SurfaceState},
    universe::{SimulationClock, celestial_body::BodyId},
};
use bevy::{
    app::AppExit,
    math::{DMat3, DQuat},
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};
pub(super) fn install(app: &mut App) {
    let config = app.world().resource::<GameConfig>();
    if !config.mars_global || config.profile.is_some() {
        return;
    }
    let Some(directory) = &config.globe_validation_dir else {
        return;
    };
    std::fs::create_dir(directory).expect("Nouveau dossier de validation globale requis");
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
    capture: Option<usize>,
    jumped: bool,
    previous_face: Option<CubeFace>,
    crossings: usize,
    rebases: usize,
    apex: f64,
}
const PHASE_S: f64 = 10.0;
const STAGES: usize = 12;
#[expect(
    clippy::too_many_arguments,
    reason = "Paramètres ECS du parcours global."
)]
fn input(
    time: Res<Time<Real>>,
    mut progress: ResMut<Progress>,
    mut terrain: ResMut<MarsTerrain>,
    mut state: ResMut<SurfaceState>,
    mut pilot: ResMut<Explorer>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut clock: ResMut<SimulationClock>,
    mut exit: MessageWriter<AppExit>,
) {
    let elapsed = time.elapsed_secs_f64();
    if elapsed >= PHASE_S * STAGES as f64 {
        progress.rebases += terrain.globe.as_ref().unwrap().rebases;
        assert!(progress.crossings >= STAGES && progress.rebases >= STAGES);
        assert!((2.0..2.2).contains(&progress.apex));
        info!(
            "GLOBAL terminé : {} franchissements, {} rebasages, apex {:.6} m",
            progress.crossings, progress.rebases, progress.apex
        );
        exit.write(AppExit::Success);
        return;
    }
    let stage = (elapsed / PHASE_S) as usize;
    keys.reset_all();
    if progress.stage != Some(stage) {
        if let Some(globe) = &terrain.globe {
            progress.rebases += globe.rebases;
        }
        let face = CubeFace::ALL[stage / 2];
        let seam = face
            .direction(
                if stage.is_multiple_of(2) { 1.0 } else { 0.23 },
                if stage.is_multiple_of(2) { 0.23 } else { 1.0 },
            )
            .unwrap();
        let inside = face
            .direction(
                if stage.is_multiple_of(2) {
                    0.99999
                } else {
                    0.23
                },
                if stage.is_multiple_of(2) {
                    0.23
                } else {
                    0.99999
                },
            )
            .unwrap();
        let tangent = (seam - inside - seam * (seam - inside).dot(seam)).normalize();
        let up = (seam - tangent * 6.0 / terrain.radius_m).normalize();
        terrain.set_site(up);
        let x = (tangent - up * tangent.dot(up)).normalize();
        let q = DQuat::from_mat3(&DMat3::from_cols(x, up, x.cross(up)));
        terrain.orientation = q;
        terrain.globe = Some(GlobeTerrain::new(up, q, terrain.radius_m));
        terrain.site_obstacles = None;
        terrain.site_obstacles = Some((q, surface::obstacles(&terrain)));
        *state = SurfaceState::default();
        state.feet.y = terrain.support_height(state.feet, HALF_WIDTH_M) + 0.002;
        state.grounded = true;
        state.yaw = -std::f64::consts::FRAC_PI_2;
        pilot.mode = MovementMode::Surface;
        pilot.anchor = Some(BodyId::MARS);
        pilot.selected = BodyId::MARS;
        progress.stage = Some(stage);
        progress.jumped = false;
        progress.previous_face = Some(face);
        info!(
            "GLOBAL préparation {stage} {face:?} : départ 6 m avant arête, parcours par touches ECS"
        );
    }
    clock.paused = true;
    pilot.captured = elapsed > 1.0;
    keys.press(KeyCode::ShiftLeft);
    keys.press(KeyCode::KeyW);
    if elapsed % PHASE_S > 0.6 && !progress.jumped && pilot.captured {
        keys.press(KeyCode::Space);
        progress.jumped = true;
    }
}
fn capture(
    time: Res<Time<Real>>,
    config: Res<GameConfig>,
    terrain: Res<MarsTerrain>,
    state: Res<SurfaceState>,
    mut progress: ResMut<Progress>,
    mut commands: Commands,
) {
    if let Some(previous) = progress.previous_face {
        let current = crate::planets::globe_terrain::key_at(terrain.body_relative(state.feet)).face;
        if previous != current {
            progress.crossings += 1;
            info!(
                "GLOBAL arête {:?} → {:?}, au sol {}",
                previous, current, state.grounded
            );
            progress.previous_face = Some(current);
        }
    }
    progress.apex = progress
        .apex
        .max(state.feet.y - terrain.support_height(state.feet, HALF_WIDTH_M));
    let elapsed = time.elapsed_secs_f64();
    let phase = elapsed % PHASE_S;
    let slot = if phase >= 8.5 {
        2
    } else if phase >= 4.0 {
        1
    } else if phase >= 1.7 {
        0
    } else {
        return;
    };
    let index = (elapsed / PHASE_S) as usize * 3 + slot;
    if progress.capture == Some(index) || elapsed >= PHASE_S * STAGES as f64 {
        return;
    }
    progress.capture = Some(index);
    let globe = terrain.globe.as_ref().unwrap();
    info!(
        "GLOBAL capture {index} face {:?}, patches {}, constructions {}, rebasages {}, index {:?}",
        globe.center.face,
        globe.patches.len(),
        globe.constructions,
        globe.rebases,
        globe.index_counts()
    );
    commands
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(
            config
                .globe_validation_dir
                .as_ref()
                .unwrap()
                .join(format!("global-{index:02}.png")),
        ));
}
