//! Parcours graphique P9 opt-in. Poses de départ préparées ; transactions par les vraies touches/boutons.
use crate::{
    GameSet,
    base::{Base, BuildKind},
    buildings::Buildings,
    construction::Construction,
    persistence::{SaveControl, Session, profiles, save},
    planets::terrain::MarsTerrain,
    player::{Explorer, surface::SurfaceState},
    survival::Oxygen,
    ui::inventory::InventoryScreen,
    ui::menu::{MenuInputSet, MenuState},
    universe::{SimulationClock, catalog::SolarSystem, celestial_body::BodyId},
};
use bevy::{
    app::AppExit,
    math::DVec3,
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};
use std::path::PathBuf;
#[derive(Resource, Default)]
struct Progress {
    phase: Option<usize>,
    captured: Option<usize>,
    menu_snapshot: Option<(f64, f64, f64)>,
    frames: Vec<(usize, f64)>,
}
fn press(w: &mut World, key: KeyCode) {
    w.resource_mut::<ButtonInput<KeyCode>>().press(key);
}
fn place(w: &mut World, position: DVec3) {
    w.resource_scope(|w, mut t: Mut<MarsTerrain>| {
        let mut state = w.resource_mut::<SurfaceState>();
        state.feet = t.chart(position);
        state.velocity = DVec3::ZERO;
        if state.feet.x.hypot(state.feet.z) > 64.0 {
            state.migrate(&mut t);
        }
        state.feet.y = t.support_height(state.feet, 0.3) + 0.002;
        state.yaw = 0.0;
        state.pitch = -0.12;
    });
}
fn click(w: &mut World, title: &str) {
    let e = w
        .query::<(Entity, &Button, &Children)>()
        .iter(w)
        .find_map(|(e, _, children)| {
            children
                .iter()
                .any(|child| w.get::<Text>(child).is_some_and(|t| t.0.trim() == title))
                .then_some(e)
        })
        .unwrap_or_else(|| panic!("Bouton absent : {title}"));
    w.entity_mut(e).insert(Interaction::Pressed);
}
fn free_candidate(w: &mut World) {
    w.resource_scope(|w, mut c: Mut<Construction>| {
        for distance in [7.0, 11.0, 15.0, 19.0, 23.0] {
            for lateral in [-16.0, -12.0, -4.0, 4.0, 12.0, 16.0] {
                c.distance_m = distance;
                c.lateral_m = lateral;
                if c.candidate(
                    w.resource::<MarsTerrain>(),
                    w.resource::<SurfaceState>(),
                    w.resource::<Buildings>(),
                    w.resource::<Base>(),
                )
                .verdict
                .is_ok()
                {
                    return;
                }
            }
        }
        panic!("Pas de pose libre pour {:?}", c.kind);
    });
}
fn origin(w: &mut World) {
    let t = w.resource::<MarsTerrain>();
    let home = w.resource::<Buildings>().home_orientation.unwrap() * DVec3::Y * t.radius_m;
    place(w, home);
}
fn chest(w: &mut World) {
    let f = w
        .resource::<Buildings>()
        .facilities
        .iter()
        .find(|f| f.kind == BuildKind::Chest)
        .unwrap();
    let p = f.position_m + f.orientation * DVec3::Z * 1.7;
    place(w, p);
}
fn load_input(w: &mut World, phase: usize) {
    match phase {
        0 => {
            profiles::open(w, "P9 Alpha").unwrap();
            let path = w.resource::<SaveControl>().path.clone();
            let text = std::fs::read_to_string(path).unwrap();
            assert_eq!(Session::capture(w).unwrap().encode(), text);
            assert_eq!(w.resource::<Buildings>().facilities.len(), 5);
        }
        1 => {
            press(w, KeyCode::Enter);
        }
        2 => {
            chest(w);
            press(w, KeyCode::KeyI);
        }
        3 => {
            assert!(w.resource::<InventoryScreen>().open);
            assert!(w.resource::<InventoryScreen>().chest.is_some());
        }
        4 => {
            press(w, KeyCode::KeyI);
            w.resource_mut::<MenuState>().open();
            profiles::open(w, "P9 Bravo").unwrap();
            assert!(w.resource::<Buildings>().facilities.is_empty());
        }
        5 => {
            let before = Session::capture(w).unwrap().encode();
            let root = w.resource::<SaveControl>().root.clone();
            std::fs::create_dir(root.join("Broken")).unwrap();
            std::fs::write(
                root.join("Broken/session.solar"),
                "SOLAR_SURVIVAL 3\ninvalid",
            )
            .unwrap();
            assert!(profiles::open(w, "Broken").is_err());
            assert_eq!(Session::capture(w).unwrap().encode(), before);
        }
        6 => {
            profiles::open(w, "P9 Alpha").unwrap();
            press(w, KeyCode::Enter);
        }
        7 => {
            w.resource_mut::<Oxygen>().remaining_seconds = 0.0;
            w.resource_mut::<Oxygen>().failed = true;
        }
        8 => {
            press(w, KeyCode::KeyR);
        }
        9 => {
            assert!(!w.resource::<Oxygen>().failed);
            assert_eq!(w.resource::<Buildings>().facilities.len(), 5);
            press(w, KeyCode::KeyM);
        }
        10 => {
            press(w, KeyCode::KeyM);
        }
        11 => {
            chest(w);
            press(w, KeyCode::KeyI);
        }
        13 => {
            info!(
                "P9 load terminé : deux profils, contenus/charges/renouvellement, refus transactionnel, R et M"
            );
            w.write_message(AppExit::Success);
        }
        _ => {}
    }
}
fn input(w: &mut World) {
    let elapsed = w.resource::<Time<Real>>().elapsed_secs_f64();
    let phase = (elapsed / 2.0) as usize;
    w.resource_mut::<ButtonInput<KeyCode>>().reset_all();
    w.resource_mut::<Explorer>().captured = false;
    if w.resource::<Progress>().phase == Some(phase) {
        return;
    }
    assert!(
        w.resource::<Progress>()
            .phase
            .is_none_or(|p| phase == p + 1),
        "Phase sautée à {phase}"
    );
    for mut interaction in w.query::<&mut Interaction>().iter_mut(w) {
        *interaction = Interaction::None;
    }
    info!(
        "P9 phase {phase} t={elapsed:.3} équipements={} O2={:.3} énergie={:.3}",
        w.resource::<Buildings>().facilities.len(),
        w.resource::<Base>().oxygen_s,
        w.resource::<Base>().energy_j
    );
    if std::env::var("SOLAR_P9_STAGE").as_deref() == Ok("load") {
        load_input(w, phase);
        w.resource_mut::<Progress>().phase = Some(phase);
        return;
    }
    match phase {
        0 => {
            profiles::create(w, "P9 Alpha").unwrap();
            w.resource_mut::<SimulationClock>().paused = true;
            press(w, KeyCode::KeyM);
        }
        1..=8 => {
            let p = w.resource::<Construction>().deposits[phase - 1].position_m;
            place(w, p);
            press(w, KeyCode::KeyE);
        }
        9 | 10 => {
            let i = if phase == 9 { 1 } else { 3 };
            let p = w.resource::<Base>().sources[i].position_m;
            place(w, p);
            press(w, KeyCode::KeyE);
        }
        11 => {
            origin(w);
            press(w, KeyCode::KeyB);
        }
        12 => click(
            w,
            &format!(
                "{} · {}",
                BuildKind::Chest.name(),
                BuildKind::Chest.recipe_label()
            ),
        ),
        13 => press(w, KeyCode::Enter),
        14 => {
            free_candidate(w);
            press(w, KeyCode::Enter);
        }
        15 => {
            assert_eq!(w.resource::<Buildings>().facilities.len(), 1);
            chest(w);
            press(w, KeyCode::KeyI);
        }
        16 => click(w, "Quantité : 1 (1/5/20)"),
        17 => click(w, "Transférer quantité"),
        18 => {
            assert_eq!(
                w.resource::<Buildings>().facilities[0]
                    .inventory
                    .count(crate::inventory::Item::Alloy),
                5
            );
            click(w, "Transférer pile");
        }
        19 => {
            assert_eq!(
                w.resource::<Buildings>().facilities[0]
                    .inventory
                    .count(crate::inventory::Item::Alloy),
                14
            );
            click(w, "01  Alliage ×14");
        }
        20 => click(w, "Maximum admissible"),
        21 => {
            assert_eq!(w.resource::<Buildings>().facilities[0].inventory.used(), 0);
            click(w, "Regrouper");
        }
        22 => press(w, KeyCode::KeyI),
        23 => {
            origin(w);
        }
        24 => {}
        25 => {
            let p = w.resource::<Base>().sources[1].position_m;
            place(w, p);
            assert_eq!(w.resource::<Base>().sources[1].remaining, 6);
            press(w, KeyCode::KeyE);
        }
        26..=45 => {
            let kind = [
                BuildKind::Panel,
                BuildKind::Battery,
                BuildKind::Generator,
                BuildKind::Tank,
            ][(phase - 26) / 5];
            match (phase - 26) % 5 {
                0 => {
                    origin(w);
                    press(w, KeyCode::KeyB);
                }
                1 => click(w, &format!("{} · {}", kind.name(), kind.recipe_label())),
                2 => press(w, KeyCode::Enter),
                3 => {
                    free_candidate(w);
                    press(w, KeyCode::Enter);
                }
                _ => {
                    assert!(
                        w.resource::<Buildings>()
                            .facilities
                            .iter()
                            .any(|f| f.kind == kind)
                    );
                    let f = w
                        .resource::<Buildings>()
                        .facilities
                        .iter()
                        .find(|f| f.kind == kind)
                        .unwrap();
                    let p = f.position_m + f.orientation * DVec3::Z * 5.0;
                    place(w, p);
                }
            }
        }
        46 => {
            assert_eq!(w.resource::<Buildings>().facilities.len(), 5);
            let p = w.resource::<Buildings>().shelters[0].position_m;
            place(w, p);
            w.resource_mut::<Oxygen>().remaining_seconds = 60.0;
        }
        47 => {
            assert!(w.resource::<Oxygen>().remaining_seconds > 90.0);
            w.resource_mut::<MenuState>().open();
            let snapshot = (
                w.resource::<Base>().energy_j,
                w.resource::<Base>().oxygen_s,
                w.resource::<Oxygen>().remaining_seconds,
            );
            w.resource_mut::<Progress>().menu_snapshot = Some(snapshot);
        }
        48 => {
            assert_eq!(
                w.resource::<Progress>().menu_snapshot.unwrap(),
                (
                    w.resource::<Base>().energy_j,
                    w.resource::<Base>().oxygen_s,
                    w.resource::<Oxygen>().remaining_seconds
                )
            );
            press(w, KeyCode::Enter);
        }
        49 => {
            chest(w);
            press(w, KeyCode::KeyI);
        }
        50 => {
            click(w, "03  Alliage ×8");
            click(w, "Transférer quantité");
        }
        51 => {
            press(w, KeyCode::KeyI);
            let t = w.resource::<MarsTerrain>();
            let p = t.body_relative(DVec3::new(64.1, 5.0, 0.0));
            place(w, p);
        }
        52 => {
            assert!(w.resource::<MarsTerrain>().globe.as_ref().unwrap().rebases > 0);
            chest(w);
            press(w, KeyCode::KeyI);
        }
        53 => {
            press(w, KeyCode::KeyI);
            let jd = w.resource::<SimulationClock>().jd;
            let solar = w.resource::<SolarSystem>();
            let period = solar.body(BodyId::MARS).rotation_period_s / 86400.0;
            let mut clock = w.resource_mut::<SimulationClock>();
            clock.jd = jd + period * 0.5;
        }
        54 => {
            assert_eq!(w.resource::<Base>().solar_w, 0.0);
        }
        55 => {
            let p = w.resource::<Buildings>().shelters[0].position_m;
            place(w, p);
            w.resource_mut::<MenuState>().open();
            let path = w.resource::<SaveControl>().path.clone();
            save(w, &path).unwrap();
        }
        56 => {
            profiles::create(w, "P9 Bravo").unwrap();
            press(w, KeyCode::KeyM);
        }
        57 => {
            w.resource_mut::<MenuState>().open();
            let path = w.resource::<SaveControl>().path.clone();
            save(w, &path).unwrap();
        }
        58 => {
            info!(
                "P9 save terminé : récolte, renouvellement, coffre/transferts, quatre équipements, production/recharge, menu/rebasage/nuit et profils"
            );
            w.write_message(AppExit::Success);
        }
        _ => {}
    }
    w.resource_mut::<Progress>().phase = Some(phase);
}
fn capture(time: Res<Time<Real>>, mut p: ResMut<Progress>, mut commands: Commands) {
    let elapsed = time.elapsed_secs_f64();
    let phase = (elapsed / 2.0) as usize;
    p.frames.push((phase, time.delta_secs_f64() * 1000.0));
    let dir = PathBuf::from(std::env::var_os("SOLAR_P9_VALIDATE_DIR").unwrap());
    let selected = if std::env::var("SOLAR_P9_STAGE").as_deref() == Ok("load") {
        [0, 3, 5, 7, 8, 9, 10, 11, 12].contains(&phase)
    } else {
        [
            0, 8, 10, 12, 14, 15, 17, 18, 19, 20, 21, 25, 29, 34, 39, 44, 46, 47, 49, 50, 51, 52,
            53, 54, 55, 56, 57,
        ]
        .contains(&phase)
    };
    if selected
        && std::env::var_os("SOLAR_P9_NO_CAPTURES").is_none()
        && elapsed % 2.0 > 0.9
        && p.captured != Some(phase)
    {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(dir.join(format!("stage-{phase:02}.png"))));
        p.captured = Some(phase);
    }
    let finish = if std::env::var("SOLAR_P9_STAGE").as_deref() == Ok("load") {
        13
    } else {
        58
    };
    if phase == finish {
        let csv = std::iter::once("phase,frame_ms".to_string())
            .chain(
                p.frames
                    .iter()
                    .map(|(phase, ms)| format!("{phase},{ms:.6}")),
            )
            .collect::<Vec<_>>()
            .join("\n");
        std::fs::write(dir.join("frames.csv"), csv).unwrap();
    }
}
pub(super) fn install(app: &mut App) {
    let Some(directory) = std::env::var_os("SOLAR_P9_VALIDATE_DIR") else {
        return;
    };
    assert!(
        !app.world()
            .resource::<crate::config::GameConfig>()
            .automated(),
        "P9 exige un diagnostic isolé"
    );
    std::fs::create_dir(PathBuf::from(directory)).expect("Dossier P9 neuf");
    app.init_resource::<Progress>()
        .add_systems(Update, input.before(MenuInputSet).in_set(GameSet::Input))
        .add_systems(Update, capture.after(GameSet::Interface));
}
