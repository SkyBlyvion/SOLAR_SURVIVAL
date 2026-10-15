//! Campagne explicite avec vrai renderer et backend audio. Les entrées sont ECS,
//! la visée des trajets est automatique et les cas ciblés préparent leurs poses.
//! Usage : grouped_validation DOSSIER_NEUF [720|900] [load|perf|cases].
use bevy::{
    app::AppExit,
    math::DVec3,
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
    window::PresentMode,
};
use solar_survival::{
    GameSet, SolarSurvivalPlugin,
    audio::SoundCue,
    base::{Base, BuildKind},
    buildings::Buildings,
    config::GameConfig,
    construction::Construction,
    controls::ACTIONS,
    expedition::Expedition,
    inventory::{Inventory, Item},
    persistence::{SaveAction, SaveControl, Session},
    planets::terrain::MarsTerrain,
    player::{Explorer, MovementMode, surface::SurfaceState},
    survival::Oxygen,
    ui::menu::{MenuState, PlayerSettings},
    universe::{SimulationClock, catalog::SolarSystem, celestial_body::BodyId},
};
use std::{collections::VecDeque, fs, path::PathBuf};

enum Step {
    Wait(f64, Option<f64>),
    Key(KeyCode),
    Click(&'static str),
    Shot(String),
    Do(&'static str, fn(&mut World)),
    Go(Target, f64, Option<f64>),
    Cue(SoundCue),
    Measure(&'static str, f64, Option<f64>),
}
enum Target {
    Home(f64, f64),
    Site(usize),
}
#[derive(Resource)]
struct Campaign {
    out: PathBuf,
    steps: VecDeque<Step>,
    previous_button: Option<Entity>,
    log: String,
    frames: String,
    max_voices: usize,
    max_sinks: usize,
}
#[derive(Resource, Default)]
struct CueLog(String);
#[derive(Resource)]
struct Baseline(Session);
fn wait(seconds: f64) -> Step {
    Step::Wait(seconds, None)
}
fn shot(name: &str) -> Step {
    Step::Shot(name.to_owned())
}
fn key(code: KeyCode) -> Step {
    Step::Key(code)
}
fn click(label: &'static str) -> Step {
    Step::Click(label)
}
fn go(target: Target) -> Step {
    let range = if matches!(target, Target::Site(_)) {
        2.2
    } else {
        0.4
    };
    Step::Go(target, range, None)
}
fn raw(w: &mut World, canonical: KeyCode) {
    let physical = ACTIONS
        .iter()
        .position(|a| a.0 == canonical)
        .map_or(canonical, |i| {
            w.resource::<PlayerSettings>().bindings.key(i)
        });
    w.resource_mut::<ButtonInput<KeyCode>>().press(physical);
}
fn feet(w: &World) -> DVec3 {
    w.resource::<MarsTerrain>()
        .body_relative(w.resource::<SurfaceState>().feet)
}
fn home(w: &World, x: f64, z: f64) -> DVec3 {
    let b = &w.resource::<Buildings>().shelters[0];
    b.position_m + b.orientation * DVec3::new(x, 0., z)
}
fn pose(w: &mut World, position: DVec3) {
    w.resource_scope(|w, mut terrain: Mut<MarsTerrain>| {
        terrain.set_site(position.normalize());
        w.resource::<Buildings>().publish_colliders(&mut terrain);
        let mut state = w.resource_mut::<SurfaceState>();
        state.feet = terrain.chart(position);
        state.feet.y = terrain.support_height(state.feet, 0.3) + 0.002;
        state.velocity = DVec3::ZERO;
        state.grounded = true;
    });
}
fn site(w: &mut World) {
    let e = w.resource::<Expedition>();
    pose(w, e.sites[e.completed].position_m);
}
fn inside(w: &mut World) {
    pose(w, home(w, 0., 0.));
}
fn restore(w: &mut World) {
    let saved = w.resource::<Baseline>().0.clone();
    saved.restore(w).unwrap();
}
fn checks(w: &mut World) {
    assert!(!w.resource::<Oxygen>().failed);
    assert!(
        w.resource::<MarsTerrain>()
            .globe
            .as_ref()
            .unwrap()
            .patches
            .len()
            <= 13
    );
}
fn drive(w: &mut World) {
    w.resource_mut::<ButtonInput<KeyCode>>().reset_all();
    w.resource_scope(|w, mut c: Mut<Campaign>| {
        let now = w.resource::<Time<Real>>().elapsed_secs_f64();
        assert!(now < 700., "Campagne bloquée au-delà de 700 s");
        if let Some(entity) = c.previous_button.take()
            && let Some(mut interaction) = w.get_mut::<Interaction>(entity)
        {
            *interaction = Interaction::None;
        }
        let Some(mut step) = c.steps.pop_front() else {
            let max_voices = c.max_voices;
            let max_sinks = c.max_sinks;
            c.log.push_str(&format!(
                "DONE max_voices={max_voices} max_sinks={max_sinks}\n"
            ));
            fs::write(c.out.join("cues.txt"), &w.resource::<CueLog>().0).unwrap();
            fs::write(c.out.join("assertions.txt"), &c.log).unwrap();
            fs::write(c.out.join("frames.csv"), &c.frames).unwrap();
            w.write_message(AppExit::Success);
            return;
        };
        let mut done = true;
        match &mut step {
            Step::Wait(seconds, start) => {
                let start = *start.get_or_insert(now);
                done = now - start >= *seconds;
            }
            Step::Key(code) => {
                raw(w, *code);
            }
            Step::Click(prefix) => {
                let target = w
                    .query::<(&Text, &ChildOf)>()
                    .iter(w)
                    .filter(|(text, _)| text.starts_with(*prefix))
                    .map(|(_, parent)| parent.parent())
                    .find(|e| {
                        w.get::<Button>(*e).is_some()
                            && w.get::<ComputedNode>(*e).is_some_and(|n| n.size().y > 0.)
                    });
                let target = target.unwrap_or_else(|| panic!("Bouton visible absent : {prefix}"));
                *w.get_mut::<Interaction>(target).unwrap() = Interaction::Pressed;
                c.previous_button = Some(target);
            }
            Step::Shot(name) => {
                let path = c.out.join(format!("{name}.png"));
                w.spawn(Screenshot::primary_window())
                    .observe(save_to_disk(path));
                let lines = w
                    .query::<(&Text, &ComputedNode)>()
                    .iter(w)
                    .filter(|(_, n)| n.size().y > 0.)
                    .map(|(text, node)| format!("{:?} {}\n", node.size(), text.0))
                    .collect::<String>();
                assert!(!lines.contains("<key:"), "Aide non résolue");
                fs::write(c.out.join(format!("{name}.txt")), lines).unwrap();
                c.log.push_str(&format!("{now:0.3} SHOT {name}\n"));
            }
            Step::Do(name, action) => {
                action(w);
                c.log.push_str(&format!("{now:0.3} PASS {name}\n"));
                fs::write(c.out.join("assertions.txt"), &c.log).unwrap();
            }
            Step::Go(target, range, start) => {
                let start = *start.get_or_insert(now);
                assert!(now - start < 85., "Trajet bloqué");
                checks(w);
                let target = match target {
                    Target::Home(x, z) => home(w, *x, *z),
                    Target::Site(i) => w.resource::<Expedition>().sites[*i].position_m,
                };
                let terrain = w.resource::<MarsTerrain>();
                let state = w.resource::<SurfaceState>();
                let local =
                    state.heading.inverse() * terrain.orientation.inverse() * (target - feet(w));
                if local.x.hypot(local.z) >= *range {
                    w.resource_mut::<SurfaceState>().yaw = (-local.x).atan2(-local.z);
                    w.resource_mut::<Explorer>().captured = true;
                    raw(w, KeyCode::KeyW);
                    raw(w, KeyCode::ShiftLeft);
                    done = false;
                }
            }
            Step::Cue(cue) => {
                w.trigger(*cue);
            }
            Step::Measure(phase, seconds, start) => {
                let start = *start.get_or_insert(now);
                done = now - start >= *seconds;
                if now - start > 2. {
                    let ms = w.resource::<Time<Real>>().delta_secs_f64() * 1000.;
                    let voices = w.query::<&AudioPlayer>().iter(w).count();
                    let sinks = w.query::<&AudioSink>().iter(w).count();
                    c.frames
                        .push_str(&format!("{phase},{ms:0.6},{voices},{sinks}\n"));
                }
                if phase.starts_with("active") || phase.starts_with("muted") {
                    for cue in SoundCue::ALL {
                        w.trigger(cue);
                    }
                }
            }
        }
        if !done {
            c.steps.push_front(step);
        }
        let voices = w.query::<&AudioPlayer>().iter(w).count();
        c.max_voices = c.max_voices.max(voices);
        let sinks = w.query::<&AudioSink>().iter(w).count();
        c.max_sinks = c.max_sinks.max(sinks);
        assert!(voices <= 8);
    });
}

fn loaded_base(w: &mut World) {
    for i in 0..64 {
        let candidate = home(w, 8. + (i % 8) as f64 * 3., -12. + (i / 8) as f64 * 3.);
        let t = w.resource::<MarsTerrain>();
        let mut local = t.chart(candidate);
        local.y = t.support_height(local, 0.3);
        let body = t.body_relative(local);
        let q = t.orientation;
        w.resource_mut::<Buildings>().install_facility(
            [
                BuildKind::Chest,
                BuildKind::Panel,
                BuildKind::Battery,
                BuildKind::Generator,
                BuildKind::Tank,
            ][i % 5],
            body,
            q,
        );
    }
    w.resource_scope(|w, mut t: Mut<MarsTerrain>| {
        w.resource::<Buildings>().publish_colliders(&mut t)
    });
    assert_eq!(w.resource::<Buildings>().facilities.len(), 64);
}
fn look_at_site(w: &mut World) {
    let site = w.resource::<Expedition>().sites[0].clone();
    pose(
        w,
        site.position_m + site.orientation * DVec3::new(0., 0., 12.),
    );
    let t = w.resource::<MarsTerrain>();
    let state = w.resource::<SurfaceState>();
    let local = state.heading.inverse() * t.orientation.inverse() * (site.position_m - feet(w));
    let mut state = w.resource_mut::<SurfaceState>();
    state.yaw = (-local.x).atan2(-local.z);
    state.pitch = 0.;
}
fn extra_cases() -> Vec<Step> {
    let mut steps = vec![
        Step::Do("restore-for-range-and-screens", restore),
        key(KeyCode::Enter),
        wait(0.2),
        Step::Do("prepare-locked-future-site", |w| {
            let p = w.resource::<Expedition>().sites[1].position_m;
            pose(w, p);
        }),
        wait(0.2),
        key(KeyCode::KeyF),
        wait(0.2),
        Step::Do("future-site-refused", |w| {
            assert!(!w.resource::<Expedition>().carrying)
        }),
        Step::Do("prepare-out-of-range", look_at_site),
        wait(0.2),
        key(KeyCode::KeyF),
        wait(0.2),
        Step::Do("out-of-range-refused", |w| {
            assert!(!w.resource::<Expedition>().carrying)
        }),
        Step::Do("prepare-active-site", site),
        wait(0.2),
        key(KeyCode::KeyI),
        wait(0.3),
        shot("inventory-at-beacon"),
        key(KeyCode::KeyF),
        wait(0.2),
        Step::Do("inventory-blocks-survey", |w| {
            assert!(!w.resource::<Expedition>().carrying)
        }),
        key(KeyCode::KeyI),
        wait(0.2),
        key(KeyCode::KeyB),
        wait(0.3),
        shot("catalog-at-beacon"),
        key(KeyCode::KeyF),
        wait(0.2),
        Step::Do("catalog-blocks-survey", |w| {
            assert!(!w.resource::<Expedition>().carrying)
        }),
        key(KeyCode::KeyB),
        wait(0.2),
        Step::Do("open-main-menu-at-beacon", |w| {
            w.resource_mut::<MenuState>().open()
        }),
        wait(0.2),
        key(KeyCode::KeyF),
        wait(0.2),
        Step::Do("menu-blocks-survey", |w| {
            assert!(!w.resource::<Expedition>().carrying)
        }),
        key(KeyCode::Enter),
        wait(0.2),
        key(KeyCode::KeyF),
        wait(0.2),
        Step::Do("survey-after-screens", |w| {
            assert!(w.resource::<Expedition>().carrying)
        }),
        Step::Do("prepare-outside-home-wall", |w| pose(w, home(w, 4., 0.))),
        wait(0.2),
        key(KeyCode::KeyF),
        wait(0.2),
        Step::Do("report-through-wall-refused", |w| {
            assert!(w.resource::<Expedition>().carrying)
        }),
        Step::Do("second-shelter-around-beacon-prepared", |w| {
            let s = w.resource::<Expedition>().sites[0].clone();
            w.resource_mut::<Buildings>()
                .install_shelter(s.position_m, s.orientation);
            w.resource_scope(|w, mut t: Mut<MarsTerrain>| {
                w.resource::<Buildings>().publish_colliders(&mut t)
            });
            site(w);
        }),
        wait(0.3),
        key(KeyCode::KeyF),
        wait(0.2),
        Step::Do("other-shelter-cannot-report", |w| {
            assert!(w.resource::<Expedition>().carrying)
        }),
        Step::Do("return-with-empty-base", |w| {
            inside(w);
            w.resource_mut::<Base>().oxygen_s = 0.;
        }),
        wait(0.2),
        key(KeyCode::KeyF),
        wait(0.2),
        Step::Do("report-does-not-require-base-oxygen", |w| {
            assert_eq!(w.resource::<Expedition>().completed, 1)
        }),
        Step::Do("restore-for-visual-conditions", restore),
        key(KeyCode::Enter),
        wait(0.2),
        Step::Do("look-at-mast-12m", look_at_site),
        wait(0.4),
        shot("mast-day"),
    ];
    for (name, delta) in [
        ("mast-sunset", 0.25648919),
        ("mast-night", 0.51297838),
        ("mast-sunrise", 0.76946757),
    ] {
        // Décalages du modèle, un quart/demi/trois quarts de période martienne.
        fn clock_set(w: &mut World, delta: f64) {
            w.resource_mut::<SimulationClock>().jd = 2451545. + delta;
        }
        let action: fn(&mut World) = match delta {
            d if d < 0.3 => |w| clock_set(w, 0.25648919),
            d if d < 0.6 => |w| clock_set(w, 0.51297838),
            _ => |w| clock_set(w, 0.76946757),
        };
        steps.extend([Step::Do(name, action), wait(0.5), shot(name)]);
    }
    steps.extend([
        Step::Do("search-prepared-storm-date", |w| {
            let period = w
                .resource::<SolarSystem>()
                .body(BodyId::MARS)
                .rotation_period_s;
            let t = w.resource::<MarsTerrain>();
            let body = feet(w);
            let seed = t.seed;
            let seconds = (0..6000)
                .map(|i| i as f64 * period / 100.)
                .find(|s| {
                    solar_survival::rendering::atmosphere::sample(*s, period, body, seed).storm
                        > 0.95
                })
                .unwrap();
            w.resource_mut::<SimulationClock>().jd = 2451545. + seconds / 86400.;
        }),
        wait(0.5),
        shot("mast-storm"),
        Step::Do("restore-before-further-cases", restore),
        key(KeyCode::Enter),
        wait(0.2),
    ]);
    steps
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let out = PathBuf::from(args.get(1).expect("Dossier neuf requis"));
    fs::create_dir(&out).expect("Dossier neuf, parent existant");
    let height = args.get(2).map_or(900, |s| s.parse::<u32>().unwrap());
    let mode = args.get(3).map_or("full", String::as_str);
    let width = if height == 720 { 1280 } else { 1440 };
    let mut tasks = vec![wait(3.)];
    if mode == "load" {
        tasks.extend([
            Step::Do("open-pending-after-process-restart", |w| {
                w.resource_mut::<SaveControl>().request =
                    Some(SaveAction::Open("Grouped Pending".into()));
            }),
            wait(1.),
            Step::Do("pending-restored-with-remapping", |w| {
                assert!(w.resource::<Expedition>().carrying);
                assert_eq!(
                    w.resource::<PlayerSettings>().bindings.key(0),
                    KeyCode::KeyC
                );
                assert_eq!(
                    w.resource::<PlayerSettings>().bindings.key(8),
                    KeyCode::KeyV
                );
            }),
            key(KeyCode::Enter),
            wait(1.),
            shot("load-pending"),
            Step::Do("return-inside-prepared", inside),
            wait(0.5),
            key(KeyCode::KeyF),
            wait(0.3),
            Step::Do("report-paid-once", |w| {
                assert_eq!(w.resource::<Expedition>().completed, 1);
            }),
            key(KeyCode::KeyF),
            wait(0.3),
            shot("load-report"),
            Step::Do("save-paid-report", |w| {
                w.resource_mut::<SaveControl>().request = Some(SaveAction::Save)
            }),
            wait(0.5),
        ]);
    } else {
        tasks.extend([
            Step::Do("create-independent-adventure", |w| {
                w.resource_mut::<SaveControl>().request =
                    Some(SaveAction::Create("Grouped Pending".into()));
            }),
            wait(1.),
            key(KeyCode::KeyM),
            wait(1.),
            key(KeyCode::KeyP),
            wait(0.3),
            Step::Do("base-established", |w| {
                checks(w);
                assert!(!w.resource::<Expedition>().sites.is_empty());
                w.insert_resource(Baseline(Session::capture(w).unwrap()));
            }),
        ]);
        if mode == "full" || mode == "menus" {
            tasks.extend([
                Step::Do("open-menu", |w| w.resource_mut::<MenuState>().open()),
                wait(0.3),
                click("Options"),
                wait(0.3),
                shot("options"),
                click("Commandes /"),
                wait(0.3),
                shot("bindings-first"),
                click("Avancer :"),
                key(KeyCode::KeyS),
                wait(0.3),
                shot("bindings-conflict"),
                key(KeyCode::KeyC),
                wait(0.3),
                shot("bindings-changed"),
                click("Reculer :"),
                key(KeyCode::Escape),
                wait(0.3),
                click("Suivant ›"),
                wait(0.3),
                click("Relevé / rapport / centrer :"),
                key(KeyCode::KeyV),
                wait(0.3),
                shot("bindings-second"),
                click("Suivant ›"),
                wait(0.2),
                click("Suivant ›"),
                wait(0.2),
                click("Suivant ›"),
                wait(0.3),
                shot("bindings-last"),
                Step::Do("bindings-real-buttons", |w| {
                    assert_eq!(
                        w.resource::<PlayerSettings>().bindings.key(0),
                        KeyCode::KeyC
                    );
                    assert_eq!(
                        w.resource::<PlayerSettings>().bindings.key(8),
                        KeyCode::KeyV
                    );
                }),
                key(KeyCode::Escape),
                wait(0.2),
                key(KeyCode::Escape),
                wait(0.2),
                key(KeyCode::Enter),
                wait(0.5),
                shot("departure"),
            ]);
            if mode == "full" {
                for i in 0..3 {
                    tasks.extend([
                        go(Target::Home(0., 6.)),
                        go(Target::Home(12., 6.)),
                        go(Target::Site(i)),
                        wait(0.3),
                        shot(&format!("site-{i}")),
                        key(KeyCode::KeyF),
                        wait(0.2),
                        Step::Do("survey-pending", |w| {
                            assert!(w.resource::<Expedition>().carrying)
                        }),
                        key(KeyCode::KeyF),
                        go(Target::Home(12., 6.)),
                        go(Target::Home(0., 6.)),
                        go(Target::Home(0., 0.)),
                        key(KeyCode::KeyF),
                        wait(0.3),
                        shot(&format!("report-{i}")),
                        Step::Do("no-pending-after-report", |w| {
                            assert!(!w.resource::<Expedition>().carrying)
                        }),
                        key(KeyCode::KeyF),
                        wait(6.),
                        Step::Do("full-refill", |w| {
                            assert_eq!(w.resource::<Oxygen>().remaining_seconds, 180.)
                        }),
                    ]);
                }
                tasks.push(Step::Do("three-ground-roundtrips-reward-total", |w| {
                    assert_eq!(w.resource::<Expedition>().completed, 3);
                    assert_eq!(
                        w.resource::<Construction>().inventory.count(Item::Alloy),
                        36
                    );
                    assert_eq!(
                        w.resource::<Construction>().inventory.count(Item::Circuit),
                        18
                    );
                }));
            }
        }
        if mode == "perf" {
            tasks.extend([
                go(Target::Home(0., 6.)),
                go(Target::Home(12., 6.)),
                Step::Do("enough-oxygen-for-audio-benchmark", |w| {
                    w.resource_mut::<Oxygen>().remaining_seconds = 180.
                }),
            ]);
            tasks.push(Step::Do("prepared-64-facilities", loaded_base));
            for repeat in 0..2 {
                tasks.extend([
                    Step::Measure(if repeat == 0 { "active-1" } else { "active-2" }, 22., None),
                    Step::Do("menu-for-mute", |w| w.resource_mut::<MenuState>().open()),
                    wait(0.3),
                    click("Options"),
                    wait(0.2),
                    click("Son :"),
                    wait(0.2),
                    key(KeyCode::Escape),
                    wait(0.2),
                    key(KeyCode::Enter),
                    wait(0.3),
                    Step::Measure(if repeat == 0 { "muted-1" } else { "muted-2" }, 22., None),
                    Step::Do("menu-for-unmute", |w| w.resource_mut::<MenuState>().open()),
                    wait(0.3),
                    click("Options"),
                    wait(0.2),
                    click("Son :"),
                    wait(0.2),
                    key(KeyCode::Escape),
                    wait(0.2),
                    key(KeyCode::Enter),
                    wait(0.3),
                ]);
            }
        } else {
            tasks.extend(extra_cases());
            tasks.extend([
                Step::Do("restore-prepared-baseline", restore),
                key(KeyCode::Enter),
                wait(0.3),
                Step::Do("prepared-site-for-refusal-tests", |w| {
                    site(w);
                    w.resource_mut::<Construction>()
                        .inventory
                        .add(Item::Alloy, 152)
                        .unwrap();
                }),
                wait(0.3),
                key(KeyCode::KeyF),
                wait(0.3),
                Step::Do("prepared-inside-full-inventory", inside),
                wait(0.3),
                key(KeyCode::KeyF),
                wait(0.3),
                shot("reward-refused"),
                Step::Do("reward-atomic-on-full-portable", |w| {
                    assert!(w.resource::<Expedition>().carrying);
                    assert_eq!(w.resource::<Expedition>().completed, 0);
                    assert_eq!(
                        w.resource::<Construction>().inventory.count(Item::Alloy),
                        152
                    );
                    w.resource_mut::<Construction>().inventory = Inventory::default();
                }),
                key(KeyCode::KeyF),
                wait(0.3),
                Step::Do("retry-pays-single-lot", |w| {
                    assert_eq!(w.resource::<Expedition>().completed, 1);
                    assert_eq!(w.resource::<Construction>().inventory.count(Item::Alloy), 8);
                }),
            ]);
            for exit in [KeyCode::KeyM, KeyCode::KeyG, KeyCode::KeyO, KeyCode::KeyU] {
                tasks.extend([
                    Step::Do("restore-before-space-exit", restore),
                    key(KeyCode::Enter),
                    wait(0.2),
                    Step::Do("prepare-site", site),
                    wait(0.2),
                    key(KeyCode::KeyF),
                    wait(0.2),
                    key(exit),
                    wait(0.2),
                    Step::Do("exit-discards-data-keeps-discovery", |w| {
                        assert_eq!(w.resource::<Explorer>().mode, MovementMode::Space);
                        assert!(!w.resource::<Expedition>().carrying);
                        assert!(w.resource::<Expedition>().discovered[0]);
                    }),
                ]);
            }
            tasks.extend([
                Step::Do("restore-for-oxygen-audio", restore),
                key(KeyCode::Enter),
                wait(0.2),
                Step::Do("outside-prepared", site),
                wait(0.3),
                key(KeyCode::KeyF),
                wait(0.2),
                Step::Do("low-threshold-real-seconds", |w| {
                    w.resource_mut::<Oxygen>().remaining_seconds = 31.
                }),
                wait(7.),
                shot("oxygen-low"),
                Step::Do("critical-threshold-real-seconds", |w| {
                    w.resource_mut::<Oxygen>().remaining_seconds = 10.3
                }),
                wait(3.),
                shot("oxygen-critical"),
                Step::Do("failure-with-pending", |w| {
                    w.resource_mut::<Oxygen>().remaining_seconds = 0.05
                }),
                wait(0.5),
                shot("oxygen-failed"),
                Step::Do("failed-latched", |w| assert!(w.resource::<Oxygen>().failed)),
                key(KeyCode::KeyR),
                wait(0.5),
                shot("restart"),
                Step::Do("restart-did-not-pay", |w| {
                    assert!(!w.resource::<Oxygen>().failed);
                    assert_eq!(w.resource::<Expedition>().completed, 0);
                }),
                Step::Do("outside-for-signals", site),
                wait(0.3),
            ]);
            for cue in SoundCue::ALL {
                tasks.extend([Step::Cue(cue), wait(1.)]);
            }
            tasks.extend([
                wait(2.3),
                Step::Do("audio-transients-expire", |w| {
                    assert_eq!(w.query::<&AudioPlayer>().iter(w).count(), 0);
                }),
                Step::Do("open-options-for-volume", |w| {
                    w.resource_mut::<MenuState>().open()
                }),
                wait(0.3),
                click("Options"),
                wait(0.3),
            ]);
            for _ in 0..5 {
                tasks.extend([click("Volume sonore :"), wait(0.25)]);
            }
            tasks.extend([
                click("Son :"),
                wait(0.3),
                shot("muted-options"),
                Step::Do("muted-gain-zero", |w| {
                    assert_eq!(w.resource::<PlayerSettings>().audio_gain(), 0.)
                }),
                click("Rétablir les valeurs par défaut"),
                wait(0.3),
                shot("reset-options"),
                Step::Do("reset-volume-and-bindings", |w| {
                    assert_eq!(w.resource::<PlayerSettings>().audio_gain(), 0.5);
                    assert_eq!(w.resource::<PlayerSettings>().bindings, default());
                }),
                key(KeyCode::Escape),
                wait(0.2),
                key(KeyCode::Enter),
                wait(0.3),
                Step::Do("restore-for-process-save", restore),
                key(KeyCode::Enter),
                wait(0.2),
                Step::Do("prepare-pending-save", |w| {
                    site(w);
                    let mut s = w.resource_mut::<PlayerSettings>();
                    s.bindings.assign(0, KeyCode::KeyC).unwrap();
                    s.bindings.assign(8, KeyCode::KeyV).unwrap();
                }),
                wait(0.3),
                key(KeyCode::KeyF),
                wait(0.2),
                Step::Do("save-pending-to-disk", |w| {
                    assert!(w.resource::<Expedition>().carrying);
                    w.resource_mut::<SaveControl>().request = Some(SaveAction::Save);
                }),
                shot("saved-pending"),
                wait(1.),
            ]);
        }
    }
    tasks.push(wait(1.));
    let config = GameConfig {
        mars_global: true,
        ..default()
    };
    assert!(!config.automated(), "Ne pas mélanger les scénarios");
    let mut app = App::new();
    app.init_resource::<CueLog>()
        .insert_resource(config)
        .insert_resource(Campaign {
            out,
            steps: tasks.into(),
            previous_button: None,
            log: String::new(),
            frames: "phase,frame_ms,voices,sinks\n".into(),
            max_voices: 0,
            max_sinks: 0,
        })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Solar Survival · 0.2 · Mars".into(),
                        resolution: (width, height).into(),
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
        .add_observer(
            |event: On<SoundCue>, time: Res<Time<Real>>, mut c: ResMut<CueLog>| {
                c.0.push_str(&format!(
                    "{:0.3} CUE {:?}\n",
                    time.elapsed_secs_f64(),
                    event.event()
                ));
            },
        )
        .add_systems(Update, drive.before(GameSet::Input));
    app.run();
}
