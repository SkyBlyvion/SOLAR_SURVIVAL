//! Parcours ciblé caméra/ciel. Préparations de pose/JD explicites, contrôles ECS réels.
use crate::{
    GameSet, camera,
    config::GameConfig,
    planets::terrain::MarsTerrain,
    player::{Explorer, MovementMode, surface::SurfaceState},
    universe::{
        SimulationClock,
        catalog::SolarSystem,
        celestial_body::BodyId,
        units::{DAY, J2000},
    },
};
use bevy::{
    app::AppExit,
    math::DVec3,
    prelude::*,
    render::{
        Render, RenderApp, RenderSystems,
        camera::ExtractedCamera,
        view::screenshot::{Screenshot, save_to_disk},
    },
};

pub(super) fn install(app: &mut App) {
    let Some(path) = &app.world().resource::<GameConfig>().sky_validation_dir else {
        return;
    };
    std::fs::create_dir(path).expect("Dossier ciel final nouveau requis");
    app.add_systems(
        Update,
        input
            .before(crate::ui::menu::MenuInputSet)
            .before(camera::controls)
            .in_set(GameSet::Input),
    )
    .add_systems(Update, capture.after(GameSet::Interface));
    if let Some(render) = app.get_sub_app_mut(RenderApp) {
        render.add_systems(Render, check_views.after(RenderSystems::PrepareViews));
    }
}
fn check_views(cameras: Query<&ExtractedCamera>, mut seen: Local<bool>) {
    let count = cameras.iter().count();
    assert!(count <= 1, "Plus d'une caméra extraite : {count}");
    if count == 1 && !*seen {
        info!("SKY renderer : une caméra extraite, aucune vue de ciel séparée");
        *seen = true;
    }
}
#[expect(clippy::too_many_arguments, reason = "Diagnostic ECS ciblé")]
fn input(
    time: Res<Time<Real>>,
    solar: Res<SolarSystem>,
    mut state: ResMut<SurfaceState>,
    mut pilot: ResMut<Explorer>,
    mut terrain: ResMut<MarsTerrain>,
    mut clock: ResMut<SimulationClock>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut oxygen: ResMut<crate::survival::Oxygen>,
    buildings: Res<crate::buildings::Buildings>,
    mut menu: ResMut<crate::ui::menu::MenuState>,
    mut previous: Local<Option<usize>>,
    mut exit: MessageWriter<AppExit>,
    mut buttons: Query<(&Children, &mut Interaction), With<Button>>,
    texts: Query<&Text>,
) {
    let stage = (time.elapsed_secs_f64() / 3.0) as usize;
    keys.reset_all();
    let planets = std::env::var_os("SOLAR_P7_PLANETS_REVIEW").is_some();
    let count = if planets {
        25
    } else if std::env::var_os("SOLAR_P7_REVIEW").is_some() {
        34
    } else {
        24
    };
    if stage >= count {
        info!("SKY terminé : {count} poses, transitions et caméra unique vérifiées");
        exit.write(AppExit::Success);
        return;
    }
    if *previous != Some(stage) {
        clock.paused = true;
        pilot.captured = false;
        if planets {
            clock.jd = J2000;
            if stage < 24 {
                let id = BodyId([1, 2, 5, 6, 7, 8][stage / 4]);
                inspect_body(&mut pilot, &solar, &clock, id, stage % 4);
            } else {
                menu.open();
            }
        } else {
            match stage {
                0 => {
                    clock.jd = J2000;
                    keys.press(KeyCode::KeyM);
                }
                1 => aim_sun(&mut state, &pilot, &terrain, &solar, &clock),
                2 => {
                    state.pitch = 0.0;
                    state.yaw += std::f64::consts::PI;
                }
                3 => {
                    clock.jd = J2000 + solar.body(BodyId::MARS).rotation_period_s * 0.5 / DAY;
                    state.pitch = 0.7;
                }
                4 => {
                    state.yaw += 1.5;
                    state.pitch = 0.4;
                }
                5 | 6 => {
                    state.feet = terrain.chart(buildings.shelters[0].position_m);
                    state.feet.y = terrain.support_height(state.feet, 0.3) + 0.002;
                    state.velocity = DVec3::ZERO;
                    clock.jd = J2000;
                    if stage == 5 {
                        state.pitch = 1.2;
                    } else {
                        aim_sun(&mut state, &pilot, &terrain, &solar, &clock);
                    }
                }
                7 => {
                    state.feet = DVec3::ZERO;
                    state.feet.y = terrain.support_height(state.feet, 0.3) + 0.002;
                    // Chercher un crépuscule dans un cycle, sans changer le modèle.
                    let period = solar.body(BodyId::MARS).rotation_period_s / DAY;
                    let mut best = (f64::MAX, J2000);
                    for i in 0..1000 {
                        let jd = J2000 + period * i as f64 / 1000.0;
                        let frame = terrain
                            .inertial_frame(solar.body(BodyId::MARS).rotation((jd - J2000) * DAY));
                        let sun = frame.orientation.inverse()
                            * (-solar.position(BodyId::MARS, jd)).normalize();
                        let error = (sun.dot(DVec3::Y) - 0.025).abs();
                        if error < best.0 {
                            best = (error, jd);
                        }
                    }
                    clock.jd = best.1;
                    aim_sun(&mut state, &pilot, &terrain, &solar, &clock);
                }
                8 => {
                    clock.jd = J2000;
                    state.feet.x = 63.5;
                    state.yaw = -std::f64::consts::FRAC_PI_2;
                    state.pitch = 0.1;
                }
                9 => {
                    aim_sun(&mut state, &pilot, &terrain, &solar, &clock);
                }
                10 => menu.open(),
                11 => keys.press(KeyCode::Enter),
                12 | 13 | 15 | 17 | 19 => keys.press(KeyCode::KeyM),
                14 => keys.press(KeyCode::KeyG),
                16 => keys.press(KeyCode::KeyO),
                18 => keys.press(KeyCode::KeyU),
                20 => {
                    oxygen.remaining_seconds = 0.05;
                }
                21 => keys.press(KeyCode::KeyR),
                22 => {
                    clock.jd = 2461325.5;
                    state.pitch = 0.5;
                }
                23 => {
                    // Nouveau site de diagnostic : même sphère, bâtiments restés au premier site.
                    terrain.set_site(DVec3::NEG_Y);
                    *state = SurfaceState::default();
                    state.feet.y = terrain.support_height(state.feet, 0.3) + 0.002;
                    state.pitch = 0.3;
                }
                24 | 28 => {
                    clock.jd = J2000;
                    pilot.selected = if stage == 24 {
                        BodyId::EARTH
                    } else {
                        BodyId::MARS
                    };
                    pilot.visit(&solar, clock.jd);
                }
                25..=27 | 29..=31 => {
                    // Poses préparées d'inspection couture et pôles, dans la convention du catalogue.
                    let body = solar.body(pilot.selected);
                    let q = body.rotation(clock.seconds_since_epoch());
                    let direction = match stage {
                        25 | 29 => DVec3::new(-1.0, 0.05, 0.0),
                        26 | 30 => DVec3::new(0.05, 1.0, 0.05),
                        _ => DVec3::new(0.05, -1.0, 0.05),
                    };
                    pilot.local_m = q * direction.normalize() * body.radius_m * 3.5;
                    let direction = -pilot.local_m;
                    pilot.look_toward(direction);
                }
                32 => keys.press(KeyCode::KeyM),
                33 => {
                    state.feet = DVec3::new(10.0, 0.0, 0.0);
                    state.feet.y = terrain.support_height(state.feet, 0.3) + 0.002;
                    state.velocity = DVec3::ZERO;
                    state.yaw = 0.0;
                    state.pitch = -1.0;
                }
                _ => {}
            }
        }
        *previous = Some(stage);
    }
    if planets && stage == 24 {
        for (children, mut interaction) in &mut buttons {
            if children
                .iter()
                .any(|e| texts.get(e).is_ok_and(|t| t.0.trim() == "Crédits"))
            {
                *interaction = Interaction::Pressed;
            }
        }
    }
    if !planets && stage == 8 {
        pilot.captured = true;
        keys.press(KeyCode::KeyW);
        keys.press(KeyCode::ShiftLeft);
    }
    // Vraies commandes P et T : pause→reprise et accélération ; pose suivante les fige.
    if !planets && stage == 9 && time.elapsed_secs_f64() % 3.0 < 0.15 && clock.paused {
        keys.press(KeyCode::KeyP);
        keys.press(KeyCode::KeyT);
    }
}
fn inspect_body(
    pilot: &mut Explorer,
    solar: &SolarSystem,
    clock: &SimulationClock,
    id: BodyId,
    pose: usize,
) {
    pilot.selected = id;
    pilot.visit(solar, clock.jd);
    if pose == 0 {
        return;
    }
    let body = solar.body(id);
    let direction = match pose {
        1 => DVec3::new(-1.0, 0.05, 0.0),
        2 => DVec3::new(0.05, 1.0, 0.05),
        _ => DVec3::new(0.05, -1.0, 0.05),
    };
    pilot.local_m =
        body.rotation(clock.seconds_since_epoch()) * direction.normalize() * body.radius_m * 3.5;
    let aim = -pilot.local_m;
    pilot.look_toward(aim);
}
fn aim_sun(
    state: &mut SurfaceState,
    pilot: &Explorer,
    terrain: &MarsTerrain,
    solar: &SolarSystem,
    clock: &SimulationClock,
) {
    let frame = terrain.inertial_frame(
        solar
            .body(BodyId::MARS)
            .rotation(clock.seconds_since_epoch()),
    );
    let local = frame.orientation.inverse() * (-pilot.global(solar, clock.jd)).normalize();
    let q = bevy::math::DQuat::from_rotation_arc(
        DVec3::Y,
        terrain.site_relative(state.feet).normalize(),
    ) * state.heading;
    let direction = q.inverse() * local;
    state.yaw = (-direction.x).atan2(-direction.z);
    state.pitch = direction.y.asin().clamp(-1.55, 1.55);
}
#[expect(clippy::too_many_arguments, reason = "Diagnostic ECS ciblé")]
fn capture(
    time: Res<Time<Real>>,
    config: Res<GameConfig>,
    pilot: Res<Explorer>,
    state: Res<SurfaceState>,
    clock: Res<SimulationClock>,
    cameras: Query<(Entity, &Camera, &DistanceFog, &Projection), With<camera::GameCamera>>,
    all: Query<Entity, With<Camera3d>>,
    mut previous: Local<Option<usize>>,
    mut identity: Local<Option<Entity>>,
    mut commands: Commands,
) {
    let stage = (time.elapsed_secs_f64() / 3.0) as usize;
    assert_eq!(all.iter().count(), 1);
    let (entity, camera, fog, projection) = cameras.single().unwrap();
    assert!(camera.is_active);
    if let Some(id) = *identity {
        assert_eq!(entity, id);
    } else {
        *identity = Some(entity);
    }
    let count = if std::env::var_os("SOLAR_P7_PLANETS_REVIEW").is_some() {
        25
    } else if std::env::var_os("SOLAR_P7_REVIEW").is_some() {
        34
    } else {
        24
    };
    if time.elapsed_secs_f64() % 3.0 >= 1.5 && stage < count && *previous != Some(stage) {
        info!(
            "SKY pose {stage} : {:?}, JD {:.8}, pieds {:?}, caméra {:?}",
            pilot.mode, clock.jd, state.feet, entity
        );
        info!(
            "SKY présentation : brouillard {:?}, {:?}, projection {:?}",
            fog.color, fog.falloff, projection
        );
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(
                config
                    .sky_validation_dir
                    .as_ref()
                    .unwrap()
                    .join(format!("sky-{stage:02}.png")),
            ));
        *previous = Some(stage);
    }
    if stage == 12 || stage == 14 || stage == 16 || stage == 18 {
        assert_eq!(pilot.mode, MovementMode::Space);
    }
}
