//! Parcours P6 : deux aventures créées par le menu, puis rechargées dans un second processus.
use crate::{
    GameSet,
    buildings::Buildings,
    camera::GameCamera,
    config::GameConfig,
    construction::Construction,
    persistence::{SaveControl, Session, profiles},
    planets::terrain::MarsTerrain,
    player::{Explorer, MovementMode, surface::SurfaceState},
    survival::Oxygen,
    ui::menu::{MenuInputSet, MenuState, PlayerSettings},
    universe::SimulationClock,
};
use bevy::{
    app::AppExit,
    input::{
        ButtonState,
        keyboard::{Key, KeyboardInput},
    },
    math::DVec3,
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
    window::PrimaryWindow,
};
#[derive(Resource)]
struct Progress {
    phase: Option<usize>,
    shot: usize,
    load: bool,
    camera: Option<Entity>,
}
pub(super) fn install(app: &mut App) {
    let c = app.world().resource::<GameConfig>();
    let Some(dir) = &c.persistence_validation_dir else {
        return;
    };
    assert!(
        c.mars_global
            && c.profile.is_none()
            && !c.cube_surface
            && c.construction_validation_dir.is_none()
            && c.sky_validation_dir.is_none()
            && c.survival_validation_dir.is_none()
            && c.globe_validation_dir.is_none()
            && c.smoke_seconds.is_none()
            && c.p2_seconds.is_none()
    );
    std::fs::create_dir(dir).expect("Dossier P6 neuf");
    let stage = std::env::var("SOLAR_P6_STAGE").unwrap();
    assert!(["save", "load"].contains(&stage.as_str()));
    app.insert_resource(Progress {
        phase: None,
        shot: 0,
        load: stage == "load",
        camera: None,
    })
    .add_systems(Update, input.before(MenuInputSet).in_set(GameSet::Input))
    .add_systems(Update, capture.after(GameSet::Interface));
}
fn press(w: &mut World, key: KeyCode) {
    w.resource_mut::<ButtonInput<KeyCode>>().press(key);
}
fn prepare(w: &mut World, x: f64, z: f64) {
    let y = w
        .resource::<MarsTerrain>()
        .support_height(DVec3::new(x, 0.0, z), 0.3)
        + 0.002;
    let mut s = w.resource_mut::<SurfaceState>();
    s.feet = DVec3::new(x, y, z);
    s.velocity = DVec3::ZERO;
    s.yaw = 0.0;
    s.pitch = 0.0;
}
fn type_name(w: &mut World, name: &str) {
    let window = w
        .query_filtered::<Entity, With<PrimaryWindow>>()
        .single(w)
        .unwrap();
    w.write_message(KeyboardInput {
        key_code: KeyCode::KeyA,
        logical_key: Key::Character(name.into()),
        state: ButtonState::Pressed,
        text: Some(name.into()),
        repeat: false,
        window,
    });
}
fn click(w: &mut World, title: &str) {
    let entity = w
        .query::<(Entity, &Button, &Children)>()
        .iter(w)
        .find_map(|(e, _, children)| {
            children
                .iter()
                .any(|child| {
                    w.get::<Text>(child)
                        .is_some_and(|text| text.0.trim().trim_start_matches('›').trim() == title)
                })
                .then_some(e)
        })
        .unwrap_or_else(|| panic!("Bouton {title} absent"));
    w.entity_mut(entity).insert(Interaction::Pressed);
}
fn verify(w: &World, name: &str) {
    let file = w
        .resource::<SaveControl>()
        .root
        .join(name)
        .join("session.solar");
    Session::decode(&std::fs::read_to_string(file).unwrap())
        .unwrap()
        .verify_restored(w);
}
fn input(w: &mut World) {
    let time = w.resource::<Time<Real>>().elapsed_secs_f64();
    w.resource_mut::<ButtonInput<KeyCode>>().reset_all();
    let phase = (time / 2.0) as usize;
    if w.resource::<Progress>().phase == Some(phase) {
        return;
    }
    let camera = w
        .query_filtered::<Entity, With<GameCamera>>()
        .single(w)
        .unwrap();
    if let Some(old) = w.resource::<Progress>().camera {
        assert_eq!(old, camera);
    } else {
        w.resource_mut::<Progress>().camera = Some(camera);
    }
    let load = w.resource::<Progress>().load;
    info!(
        "P6 {} phase {phase} t={time:.3} profil={:?} stock={} abris={} O2={:.6} mode={:?} camera={camera:?}",
        if load { "load" } else { "save" },
        w.resource::<SaveControl>().active_profile,
        w.resource::<Construction>()
            .inventory
            .count(crate::inventory::Item::Alloy),
        w.resource::<Buildings>().shelters.len(),
        w.resource::<Oxygen>().remaining_seconds,
        w.resource::<Explorer>().mode
    );
    if !load {
        match phase {
            0 => *w.resource_mut::<MenuState>() = MenuState::default(),
            1 => press(w, KeyCode::Enter),
            2 => type_name(w, "Alpha"),
            3 => press(w, KeyCode::Enter),
            4 => {
                assert_eq!(
                    w.resource::<SaveControl>().active_profile.as_deref(),
                    Some("Alpha")
                );
                w.resource_mut::<SimulationClock>().paused = true;
                press(w, KeyCode::KeyM);
            }
            5 => {
                prepare(w, 8.0, 0.0);
                press(w, KeyCode::KeyE);
            }
            6 => {
                prepare(w, 8.0, -6.0);
                press(w, KeyCode::KeyE);
            }
            7 => {
                assert_eq!(
                    w.resource::<Construction>()
                        .inventory
                        .count(crate::inventory::Item::Alloy),
                    12
                );
                prepare(w, 16.0, 0.0);
                press(w, KeyCode::KeyB);
            }
            8 => {
                assert!(w.resource::<Construction>().catalog_open);
                click(w, "Prévisualiser · Entrée");
            }
            9 => {
                assert!(
                    w.resource::<Construction>()
                        .preview
                        .as_ref()
                        .unwrap()
                        .verdict
                        .is_ok()
                );
                press(w, KeyCode::Enter);
            }
            10 => {
                assert_eq!(w.resource::<Buildings>().shelters.len(), 2);
                prepare(w, 64.1, 0.0);
                w.resource_scope(|w, mut t: Mut<MarsTerrain>| {
                    assert!(w.resource_mut::<SurfaceState>().migrate(&mut t));
                });
                let b = w.resource::<Buildings>().shelters[1].clone();
                let p = w.resource::<MarsTerrain>().chart(b.position_m);
                prepare(w, p.x, p.z);
                w.resource_mut::<Oxygen>().remaining_seconds = 60.0;
                w.resource_mut::<PlayerSettings>().fov_index = 2;
                w.resource_mut::<PlayerSettings>().msaa_index = 0;
                w.resource_mut::<PlayerSettings>().sensitivity_index = 2;
                w.resource_mut::<SimulationClock>().jd = 2451545.1;
            }
            11 => {
                assert!(w.resource::<Oxygen>().shelter.is_some());
                w.resource_mut::<MenuState>().open();
                press(w, KeyCode::F5);
            }
            12 => click(w, "Nouvelle aventure"),
            13 => type_name(w, "Bravo"),
            14 => press(w, KeyCode::Enter),
            15 => {
                assert_eq!(
                    w.resource::<SaveControl>().active_profile.as_deref(),
                    Some("Bravo")
                );
                assert!(w.resource::<Buildings>().shelters.is_empty());
                assert!(w.resource::<Construction>().deposits.is_empty());
                assert_eq!(w.resource::<Oxygen>().remaining_seconds, 180.0);
                w.resource_mut::<SimulationClock>().paused = true;
                press(w, KeyCode::KeyM);
            }
            16 => {
                prepare(w, 8.0, 0.0);
                press(w, KeyCode::KeyE);
            }
            17 => {
                assert_eq!(
                    w.resource::<Construction>()
                        .inventory
                        .count(crate::inventory::Item::Alloy),
                    6
                );
                w.resource_mut::<Oxygen>().remaining_seconds = 45.0;
                w.resource_mut::<MenuState>().open();
                press(w, KeyCode::F5);
            }
            18 => press(w, KeyCode::F9),
            19 => click(w, "Alpha"),
            20 => press(w, KeyCode::Enter),
            21 => {
                verify(w, "Alpha");
                assert_eq!(w.resource::<Buildings>().shelters.len(), 2);
                assert_eq!(
                    w.resource::<Construction>()
                        .inventory
                        .count(crate::inventory::Item::Alloy),
                    0
                );
                press(w, KeyCode::Enter);
            }
            22 => {
                assert!(w.resource::<Oxygen>().remaining_seconds > 170.0);
                w.resource_mut::<MenuState>().open();
                press(w, KeyCode::F5);
            }
            23 => press(w, KeyCode::F9),
            _ => {
                assert_eq!(
                    profiles::list(&w.resource::<SaveControl>().root).unwrap(),
                    vec!["Alpha", "Bravo"]
                );
                info!(
                    "P6 save terminé : Alpha construit/rebasé, Bravo indépendant, choix des profils et retour Alpha"
                );
                w.write_message(AppExit::Success);
            }
        }
    } else {
        match phase {
            0 => *w.resource_mut::<MenuState>() = MenuState::default(),
            1 => press(w, KeyCode::F9),
            2 => press(w, KeyCode::ArrowDown),
            3 => press(w, KeyCode::Enter),
            4 => {
                verify(w, "Bravo");
                assert_eq!(
                    w.resource::<SaveControl>().active_profile.as_deref(),
                    Some("Bravo")
                );
                assert_eq!(
                    w.resource::<Construction>()
                        .inventory
                        .count(crate::inventory::Item::Alloy),
                    6
                );
                assert_eq!(w.resource::<Buildings>().shelters.len(), 1);
                assert_eq!(w.resource::<Oxygen>().remaining_seconds, 45.0);
            }
            5 => press(w, KeyCode::Enter),
            6 => press(w, KeyCode::KeyB),
            7 => {
                assert!(w.resource::<Construction>().catalog_open);
                assert_eq!(
                    w.resource::<Construction>()
                        .inventory
                        .count(crate::inventory::Item::Alloy),
                    6
                );
                press(w, KeyCode::Escape);
            }
            8 => {
                w.resource_mut::<MenuState>().open();
                press(w, KeyCode::F9);
            }
            9 => press(w, KeyCode::ArrowUp),
            10 => press(w, KeyCode::Enter),
            11 => {
                verify(w, "Alpha");
                assert_eq!(w.resource::<Buildings>().shelters.len(), 2);
                assert_eq!(
                    w.resource::<Construction>()
                        .inventory
                        .count(crate::inventory::Item::Alloy),
                    0
                );
            }
            12 => {
                press(w, KeyCode::Enter);
                press(w, KeyCode::KeyM);
            }
            13 => {
                assert_eq!(w.resource::<Explorer>().mode, MovementMode::Space);
                w.resource_mut::<MenuState>().open();
                press(w, KeyCode::F5);
            }
            14 => press(w, KeyCode::F9),
            15 => press(w, KeyCode::Enter),
            16 => {
                assert_eq!(w.resource::<Explorer>().mode, MovementMode::Space);
                press(w, KeyCode::Enter);
                press(w, KeyCode::KeyM);
            }
            17 => {
                prepare(w, 16.0, 0.0);
                w.resource_mut::<Oxygen>().remaining_seconds = 0.001;
            }
            18 => {
                assert!(w.resource::<Oxygen>().failed);
                w.resource_mut::<MenuState>().open();
                press(w, KeyCode::F5);
            }
            19 => press(w, KeyCode::F9),
            20 => press(w, KeyCode::Enter),
            21 => assert!(w.resource::<Oxygen>().failed),
            22 => {
                press(w, KeyCode::Enter);
                press(w, KeyCode::KeyR);
            }
            23 => {
                assert!(!w.resource::<Oxygen>().failed);
                w.resource_mut::<MenuState>().open();
                let dir = w.resource::<SaveControl>().root.join("Broken");
                std::fs::create_dir(&dir).unwrap();
                std::fs::write(dir.join("session.solar"), "corrompu").unwrap();
                press(w, KeyCode::F9);
            }
            24 => click(w, "Broken"),
            25 => press(w, KeyCode::Enter),
            26 => {
                assert!(w.resource::<SaveControl>().status.contains("impossible"));
                assert_eq!(
                    w.resource::<SaveControl>().active_profile.as_deref(),
                    Some("Alpha")
                );
                assert_eq!(w.resource::<Buildings>().shelters.len(), 2);
                assert!(!w.resource::<Oxygen>().failed);
            }
            _ => {
                info!(
                    "P6 load terminé : redémarrage réel, profils indépendants, catalogue, espace, échec/R et refus corrompu"
                );
                w.write_message(AppExit::Success);
            }
        }
    }
    w.resource_mut::<Progress>().phase = Some(phase);
}
fn capture(w: &mut World) {
    let progress = w.resource::<Progress>();
    let shots: &[f64] = if progress.load {
        &[
            1.0, 3.0, 5.0, 9.0, 11.0, 13.0, 17.0, 23.0, 27.0, 31.0, 33.0, 37.0, 43.0, 45.0, 49.0,
            53.0,
        ]
    } else {
        &[
            1.0, 3.0, 5.0, 9.0, 15.0, 17.0, 19.0, 21.0, 23.0, 27.0, 29.0, 35.0, 37.0, 41.0, 45.0,
            47.0,
        ]
    };
    let n = progress.shot;
    if n >= shots.len() || w.resource::<Time<Real>>().elapsed_secs_f64() < shots[n] {
        return;
    }
    let dir = w
        .resource::<GameConfig>()
        .persistence_validation_dir
        .as_ref()
        .unwrap()
        .clone();
    std::fs::write(
        dir.join(format!("stage-{n:02}.solar")),
        Session::capture(w).unwrap().encode(),
    )
    .unwrap();
    w.spawn(Screenshot::primary_window())
        .observe(save_to_disk(dir.join(format!("stage-{n:02}.png"))));
    w.resource_mut::<Progress>().shot += 1;
}
