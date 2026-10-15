//! Parcours P4 de 56 s. Préparations de réserve/2e pose explicites ; taux de production inchangés.
use crate::{
    GameSet,
    buildings::Buildings,
    camera,
    config::GameConfig,
    planets::terrain::MarsTerrain,
    player::{Explorer, surface::SurfaceState},
    survival::Oxygen,
    ui::menu::MenuState,
    universe::SimulationClock,
};
use bevy::{
    app::AppExit,
    math::DVec3,
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};
#[derive(Resource, Default)]
struct Progress {
    phase: Option<usize>,
    capture: usize,
    menu_reserve: f64,
    menu_jd: f64,
    space_reserve: f64,
    poses: Vec<(crate::buildings::BuildingId, DVec3, bevy::math::DQuat)>,
}
pub(super) fn install(app: &mut App) {
    let c = app.world().resource::<GameConfig>();
    if c.profile.is_some() {
        return;
    }
    let Some(dir) = &c.survival_validation_dir else {
        return;
    };
    assert!(
        c.mars_global
            && !c.cube_surface
            && c.globe_validation_dir.is_none()
            && c.p2_seconds.is_none()
            && c.smoke_seconds.is_none()
            && c.surface_smoke_seconds.is_none()
            && c.rotation_smoke_seconds.is_none(),
        "P4 exige un scénario mondial isolé"
    );
    std::fs::create_dir(dir).expect("Dossier P4 nouveau, parent existant");
    app.init_resource::<Progress>()
        .add_systems(
            Update,
            input
                .before(crate::controls::translate)
                .before(camera::controls)
                .in_set(GameSet::Input),
        )
        .add_systems(Update, capture.after(GameSet::Interface));
}
const BOUNDS: [f64; 15] = [
    0.0, 2.0, 3.4, 9.0, 10.4, 12.0, 23.0, 34.0, 36.0, 38.0, 40.0, 42.0, 44.0, 46.0, 47.4,
];
#[expect(clippy::too_many_arguments, reason = "Scénario fonctionnel ECS P4.")]
fn input(
    time: Res<Time<Real>>,
    mut p: ResMut<Progress>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut pilot: ResMut<Explorer>,
    mut state: ResMut<SurfaceState>,
    mut terrain: ResMut<MarsTerrain>,
    mut buildings: ResMut<Buildings>,
    mut oxygen: ResMut<Oxygen>,
    mut menu: ResMut<MenuState>,
    mut clock: ResMut<SimulationClock>,
    mut exit: MessageWriter<AppExit>,
) {
    let elapsed = time.elapsed_secs_f64();
    keys.reset_all();
    if elapsed >= 56.0 {
        assert_eq!(buildings.shelters.len(), 2);
        assert!(oxygen.shelter == Some(buildings.shelters[1].id) && !oxygen.failed);
        assert_eq!(oxygen.remaining_seconds, crate::survival::CAPACITY_SECONDS);
        assert_eq!(
            (
                buildings.shelters[0].id,
                buildings.shelters[0].position_m,
                buildings.shelters[0].orientation
            ),
            p.poses[0]
        );
        info!(
            "P4 terminé : consommation, recharge, menu, rebasage aller/retour, échec, R, espace, deuxième instance"
        );
        exit.write(AppExit::Success);
        return;
    }
    let phase = BOUNDS.iter().rposition(|&v| elapsed >= v).unwrap();
    if p.phase != Some(phase) {
        info!(
            "P4 phase {phase} t={elapsed:.3} O2={:.3} shelter={:?} feet={:?}",
            oxygen.remaining_seconds, oxygen.shelter, state.feet
        );
        match phase {
            0 => {
                keys.press(KeyCode::KeyM);
                oxygen.remaining_seconds = 80.0;
                clock.paused = true;
            }
            2 => assert!(oxygen.shelter.is_some(), "Entrée par la porte"),
            3 => assert_eq!(oxygen.remaining_seconds, crate::survival::CAPACITY_SECONDS),
            4 => {
                assert!(oxygen.shelter.is_none());
                p.menu_reserve = oxygen.remaining_seconds;
                p.menu_jd = clock.jd;
                p.poses = buildings
                    .shelters
                    .iter()
                    .map(|b| (b.id, b.position_m, b.orientation))
                    .collect();
                menu.open();
                clock.paused = false;
                clock.speed_index = 3;
            }
            5 => {
                assert_eq!(oxygen.remaining_seconds, p.menu_reserve);
                assert!(clock.jd > p.menu_jd);
                menu.play();
            }
            7 => {
                assert!(terrain.globe.as_ref().unwrap().rebases >= 1);
                let b = &buildings.shelters[0];
                assert_eq!((b.id, b.position_m, b.orientation), p.poses[0]);
            }
            8 => {
                oxygen.remaining_seconds = 1.0;
            }
            9 => {
                assert!(oxygen.failed);
                keys.press(KeyCode::KeyR);
            }
            10 => {
                assert!(!oxygen.failed && oxygen.shelter.is_some());
                keys.press(KeyCode::KeyM);
                p.space_reserve = oxygen.remaining_seconds;
            }
            11 => {
                assert_eq!(oxygen.remaining_seconds, p.space_reserve);
                keys.press(KeyCode::KeyM);
            }
            12 => {
                let mut point = DVec3::new(16.0, 0.0, -6.0);
                point.y = terrain.height(point.x, point.z);
                buildings.install_shelter(terrain.body_relative(point), terrain.orientation);
                buildings.publish_colliders(&mut terrain);
                // Diagnostic de 2e instance : pose de départ devant sa porte, pas une construction joueur.
                let b = &buildings.shelters[1];
                state.feet = terrain.chart(b.position_m + b.orientation * DVec3::Z * 6.0);
                state.feet.y = terrain.support_height(state.feet, 0.3) + 0.002;
                state.velocity = DVec3::ZERO;
                state.yaw = 0.0;
                state.pitch = 0.0;
                oxygen.remaining_seconds = 60.0;
            }
            14 => assert_eq!(oxygen.shelter, Some(buildings.shelters[1].id)),
            _ => {}
        }
        p.phase = Some(phase);
    }
    pilot.captured = elapsed > 1.0 && matches!(phase, 1 | 3 | 5 | 6 | 13);
    if phase == 1 || phase == 13 {
        keys.press(KeyCode::KeyW);
    }
    if phase == 3 {
        keys.press(KeyCode::KeyS);
    }
    if phase == 5 || phase == 6 {
        keys.press(if phase == 5 {
            KeyCode::KeyD
        } else {
            KeyCode::KeyA
        });
        keys.press(KeyCode::ShiftLeft);
    }
}
const SHOTS: [f64; 13] = [
    1.5, 4.0, 8.5, 11.3, 22.5, 34.8, 37.6, 39.3, 41.0, 43.0, 45.0, 48.0, 54.0,
];
#[expect(clippy::too_many_arguments, reason = "Captures et télémétrie P4.")]
fn capture(
    time: Res<Time<Real>>,
    config: Res<GameConfig>,
    mut p: ResMut<Progress>,
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
    let dir = config.survival_validation_dir.as_ref().unwrap();
    let record = format!(
        "t={:.6}, mode={:?}, O2={:.6}, failed={}, shelter={:?}, rebases={}, feet={:?}, buildings={}\n",
        time.elapsed_secs_f64(),
        pilot.mode,
        oxygen.remaining_seconds,
        oxygen.failed,
        oxygen.shelter,
        terrain.globe.as_ref().map_or(0, |g| g.rebases),
        state.feet,
        buildings.shelters.len()
    );
    std::fs::write(dir.join(format!("stage-{:02}.txt", p.capture)), &record).unwrap();
    info!("P4 capture {} {}", p.capture, record.trim());
    commands
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(
            dir.join(format!("stage-{:02}.png", p.capture)),
        ));
    p.capture += 1;
}
