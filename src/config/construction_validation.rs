//! Diagnostic P5 : vraies touches de récolte/preview/confirmation, départs physiques préparés explicitement.
use crate::{
    GameSet,
    buildings::Buildings,
    camera,
    config::GameConfig,
    construction::Construction,
    planets::terrain::MarsTerrain,
    player::{Explorer, MovementMode, surface::SurfaceState},
    survival::Oxygen,
    ui::menu::MenuState,
    universe::SimulationClock,
};
use bevy::{
    app::AppExit,
    math::{DQuat, DVec3},
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};
#[derive(Resource, Default)]
struct Progress {
    phase: Option<usize>,
    capture: usize,
    pose: Option<(DVec3, DQuat)>,
    reserve: f64,
    menu_alloy: u32,
}
pub(super) fn install(app: &mut App) {
    let c = app.world().resource::<GameConfig>();
    if c.profile.is_some() {
        return;
    }
    let Some(dir) = &c.construction_validation_dir else {
        return;
    };
    assert!(
        c.mars_global
            && !c.cube_surface
            && c.globe_validation_dir.is_none()
            && c.survival_validation_dir.is_none()
            && c.p2_seconds.is_none()
            && c.smoke_seconds.is_none()
            && c.surface_smoke_seconds.is_none()
            && c.rotation_smoke_seconds.is_none(),
        "P5 exige un scénario mondial isolé"
    );
    std::fs::create_dir(dir).expect("Nouveau dossier P5, parent existant");
    let progress = Progress {
        capture: if std::env::var_os("SOLAR_P5_TIMING_OUTPUT").is_some() {
            SHOTS.len()
        } else {
            0
        },
        ..default()
    };
    app.insert_resource(progress)
        .add_systems(
            Update,
            input
                .before(crate::controls::translate)
                .before(camera::controls)
                .in_set(GameSet::Input),
        )
        .add_systems(Update, capture.after(GameSet::Interface));
}
pub(super) const BOUNDS: [f64; 29] = [
    0.0, 2.0, 4.0, 6.0, 7.5, 8.2, 9.2, 10.0, 12.0, 15.0, 16.0, 18.0, 21.0, 22.0, 23.0, 24.0, 25.0,
    26.0, 30.0, 32.0, 33.0, 41.0, 43.0, 45.0, 48.0, 50.0, 52.0, 54.0, 56.0,
];
#[expect(
    clippy::too_many_arguments,
    reason = "Parcours P5 isolé avec assertions de session."
)]
fn input(
    time: Res<Time<Real>>,
    mut p: ResMut<Progress>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut motion: ResMut<bevy::input::mouse::AccumulatedMouseMotion>,
    mut pilot: ResMut<Explorer>,
    mut state: ResMut<SurfaceState>,
    terrain: Res<MarsTerrain>,
    buildings: Res<Buildings>,
    c: Res<Construction>,
    mut oxygen: ResMut<Oxygen>,
    mut clock: ResMut<SimulationClock>,
    mut menu: ResMut<MenuState>,
    mut exit: MessageWriter<AppExit>,
) {
    let elapsed = time.elapsed_secs_f64();
    keys.reset_all();
    motion.delta = Vec2::ZERO;
    if c.catalog_open {
        keys.press(KeyCode::Enter);
    }
    if elapsed >= 59.0 {
        assert_eq!(buildings.shelters.len(), 2);
        assert_eq!(c.inventory.count(crate::inventory::Item::Alloy), 0);
        assert_eq!(c.deposits.iter().filter(|d| d.remaining == 0).count(), 2);
        assert_eq!(oxygen.shelter, Some(buildings.shelters[1].id));
        assert!(!oxygen.failed);
        assert_eq!(
            (
                buildings.shelters[1].position_m,
                buildings.shelters[1].orientation
            ),
            p.pose.unwrap()
        );
        info!(
            "P5 terminé : récolte, refus, déplacement/orientation, annulation, construction, porte/mur/recharge, rebasage, espace, menu"
        );
        exit.write(AppExit::Success);
        return;
    }
    let phase = BOUNDS.iter().rposition(|&v| elapsed >= v).unwrap();
    if p.phase != Some(phase) {
        info!(
            "P5 phase {phase} t={elapsed:.3} alloy={} active={} preview={:?} feet={:?} O2={:.3}",
            c.inventory.count(crate::inventory::Item::Alloy),
            c.active,
            c.preview.as_ref().map(|p| p.verdict),
            state.feet,
            oxygen.remaining_seconds
        );
        match phase {
            0 => {
                keys.press(KeyCode::KeyM);
                clock.paused = true;
            }
            1 => {
                keys.press(KeyCode::KeyB);
            }
            2 => {
                assert!(c.preview.as_ref().unwrap().verdict.is_err());
                keys.press(KeyCode::Enter);
            }
            3 => {
                assert_eq!(buildings.shelters.len(), 1);
                keys.press(KeyCode::KeyX);
                pilot.captured = true;
                keys.press(KeyCode::KeyH);
            }
            4 => {
                keys.press(KeyCode::KeyE);
            }
            5 => {
                assert_eq!(c.inventory.count(crate::inventory::Item::Alloy), 6);
            }
            6 => {
                keys.press(KeyCode::KeyE);
            }
            7 => {
                assert_eq!(c.inventory.count(crate::inventory::Item::Alloy), 12);
                keys.press(KeyCode::KeyB);
            }
            9 => {
                assert!(c.preview.as_ref().unwrap().verdict.is_ok());
            }
            10 => {
                keys.press(KeyCode::KeyX);
            }
            11 => {
                assert_eq!(c.inventory.count(crate::inventory::Item::Alloy), 12);
                assert_eq!(buildings.shelters.len(), 1);
                keys.press(KeyCode::KeyB);
            }
            12 => {
                assert!(c.preview.as_ref().unwrap().verdict.is_ok());
            }
            13 => {
                keys.press(KeyCode::Enter);
            }
            14 => {
                assert_eq!(buildings.shelters.len(), 2);
                assert_eq!(c.inventory.count(crate::inventory::Item::Alloy), 0);
                let b = &buildings.shelters[1];
                p.pose = Some((b.position_m, b.orientation));
            }
            15 => {
                let b = &buildings.shelters[1];
                state.feet = terrain.chart(b.position_m + b.orientation * DVec3::Z * 5.0);
                state.feet.y = terrain.support_height(state.feet, 0.3) + 0.002;
                state.velocity = DVec3::ZERO;
                state.yaw = (terrain.orientation.inverse() * b.orientation)
                    .to_euler(EulerRot::YXZ)
                    .0;
                state.pitch = 0.0;
                oxygen.remaining_seconds = 60.0;
                pilot.captured = true;
            }
            17 => {
                assert_eq!(oxygen.shelter, Some(buildings.shelters[1].id));
            }
            20 => {
                state.feet = DVec3::new(20.0, terrain.height(20.0, 0.0) + 0.002, 0.0);
                state.velocity = DVec3::ZERO;
                state.yaw = -std::f64::consts::FRAC_PI_2;
            }
            21 => {
                assert!(terrain.globe.as_ref().unwrap().rebases >= 1);
            }
            22 => {
                keys.press(KeyCode::KeyM);
                p.reserve = oxygen.remaining_seconds;
            }
            23 => {
                assert_eq!(pilot.mode, MovementMode::Space);
                assert_eq!(oxygen.remaining_seconds, p.reserve);
                keys.press(KeyCode::KeyM);
            }
            24 => {
                keys.press(KeyCode::KeyB);
            }
            25 => {
                p.reserve = oxygen.remaining_seconds;
                p.menu_alloy = c.inventory.count(crate::inventory::Item::Alloy);
                menu.open();
            }
            26 => {
                assert_eq!(oxygen.remaining_seconds, p.reserve);
                assert_eq!(
                    c.inventory.count(crate::inventory::Item::Alloy),
                    p.menu_alloy
                );
                assert!(!c.active);
                menu.play();
            }
            27 => {
                let b = &buildings.shelters[1];
                state.feet = terrain.chart(b.position_m + b.orientation * DVec3::Z * 5.0);
                state.feet.y = terrain.support_height(state.feet, 0.3) + 0.002;
                state.velocity = DVec3::ZERO;
                state.yaw = (terrain.orientation.inverse() * b.orientation)
                    .to_euler(EulerRot::YXZ)
                    .0;
                state.pitch = 0.0;
                oxygen.remaining_seconds = 60.0;
                pilot.captured = true;
            }
            _ => {}
        }
        p.phase = Some(phase);
    }
    if phase == 3 {
        keys.press(KeyCode::KeyD);
    }
    if phase == 5 || phase == 15 || phase == 27 && elapsed < 55.0 {
        keys.press(KeyCode::KeyW);
    }
    if phase == 8 || phase == 11 {
        if elapsed < BOUNDS[phase] + 2.25 {
            keys.press(KeyCode::ArrowRight);
            keys.press(KeyCode::ArrowUp);
        }
        if elapsed < BOUNDS[phase] + 0.8 {
            keys.press(KeyCode::KeyL);
        }
    }
    if phase == 18 {
        keys.press(KeyCode::KeyD);
    }
    if phase == 20 {
        keys.press(KeyCode::KeyW);
        keys.press(KeyCode::ShiftLeft);
    }
}
const SHOTS: [f64; 16] = [
    1.0, 3.0, 5.0, 8.0, 9.8, 11.0, 15.5, 17.0, 21.5, 23.5, 27.0, 31.5, 42.0, 44.0, 51.0, 57.5,
];
#[expect(
    clippy::too_many_arguments,
    reason = "Captures et télémétrie de construction."
)]
fn capture(
    time: Res<Time<Real>>,
    config: Res<GameConfig>,
    mut p: ResMut<Progress>,
    c: Res<Construction>,
    oxygen: Res<Oxygen>,
    terrain: Res<MarsTerrain>,
    state: Res<SurfaceState>,
    pilot: Res<Explorer>,
    buildings: Res<Buildings>,
    mut commands: Commands,
) {
    if p.capture >= SHOTS.len() || time.elapsed_secs_f64() < SHOTS[p.capture] {
        return;
    }
    let dir = config.construction_validation_dir.as_ref().unwrap();
    let record = format!(
        "t={:.6} mode={:?} alloy={} active={} preview={:?} buildings={} O2={:.6} shelter={:?} rebases={} feet={:?}\n",
        time.elapsed_secs_f64(),
        pilot.mode,
        c.inventory.count(crate::inventory::Item::Alloy),
        c.active,
        c.preview,
        buildings.shelters.len(),
        oxygen.remaining_seconds,
        oxygen.shelter,
        terrain.globe.as_ref().map_or(0, |g| g.rebases),
        state.feet
    );
    std::fs::write(dir.join(format!("stage-{:02}.txt", p.capture)), &record).unwrap();
    info!("P5 capture {} {}", p.capture, record.trim());
    commands
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(
            dir.join(format!("stage-{:02}.png", p.capture)),
        ));
    p.capture += 1;
}
