use crate::{
    GameSet,
    camera::{NavigationRequest, controls},
    player::{Explorer, MovementMode, surface::SurfaceState},
    universe::{
        SimulationClock,
        catalog::SolarSystem,
        celestial_body::BodyId,
        units::{DAY, format_distance},
    },
};
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::PrimaryWindow,
};
mod construction;
pub mod inventory;
pub mod menu;
mod survival;
mod theme;

#[derive(Component)]
struct HudRoot;
#[derive(Component)]
struct NavigationPanel;
#[derive(Component)]
struct DebugPanel;
#[derive(Component)]
struct StatusText;
#[derive(Component)]
struct TargetText;
#[derive(Component)]
struct HelpText;
#[derive(Component)]
struct Marker(BodyId);
#[derive(Component, Clone, Copy)]
enum Action {
    Select(BodyId),
    Visit,
    Center,
    Overview,
    Surface,
    Menu,
}
#[derive(Resource)]
struct HudState {
    visible: bool,
    navigation: bool,
}
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct HudInputSet;
pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HudState {
            visible: false,
            navigation: false,
        })
        .add_plugins((
            menu::MenuPlugin,
            survival::SurvivalHudPlugin,
            construction::ConstructionHudPlugin,
            inventory::InventoryHudPlugin,
        ))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            buttons
                .after(controls)
                .in_set(HudInputSet)
                .in_set(GameSet::Input),
        )
        .add_systems(
            Update,
            (target_camera, toggle, help, refresh, markers)
                .chain()
                .in_set(GameSet::Interface),
        );
    }
}
fn font(size: f32, handle: &Handle<Font>) -> TextFont {
    TextFont {
        font: handle.clone().into(),
        font_size: FontSize::Px(size),
        ..default()
    }
}
fn setup(mut commands: Commands, solar: Res<SolarSystem>, assets: Res<AssetServer>) {
    let ui_font = assets.load("fonts/DejaVuSans.ttf");
    commands.spawn((HudRoot, Node { width: percent(100), height: percent(100), position_type: PositionType::Absolute, ..default() }))
    .with_children(|root| {
        root.spawn((NavigationPanel,Node { position_type: PositionType::Absolute, top: px(24), left: px(24), width: px(212), padding: UiRect::all(px(16)), flex_direction: FlexDirection::Column, row_gap: px(4), border_radius: BorderRadius::ZERO, ..default() }, BackgroundColor(theme::PANEL)))
        .with_children(|panel| {
            panel.spawn((Text::new("SOLAR SURVIVAL"), font(19.0, &ui_font), TextColor(theme::TEXT)));
            panel.spawn((Text::new("0.2  /  MARS\nSystème solaire · échelle 1:1"), font(12.0, &ui_font), TextColor(theme::MUTED)));
            for body in &solar.bodies {
                panel.spawn((Button, Action::Select(body.id), Node { padding: UiRect::axes(px(12),px(4)), border_radius: BorderRadius::ZERO, ..default() }, BackgroundColor(theme::BUTTON)))
                    .with_child((Text::new(format!("{}   {}",body.id.0,body.name)),font(13.0, &ui_font),TextColor(theme::TEXT)));
            }
            for (action,label) in [(Action::Center,"F   Centrer"),(Action::Visit,"G   Visiter"),(Action::Overview,"O   Vue du système"),(Action::Surface,"M   Surface Mars / retour"),(Action::Menu,"Échap   Menu principal")] {
                panel.spawn((Button,action,Node { padding: UiRect::axes(px(12),px(6)), border_radius: BorderRadius::ZERO, ..default() }, BackgroundColor(theme::BUTTON)))
                    .with_child((Text::new(label),font(13.0, &ui_font),TextColor(theme::TEXT)));
            }
        });
        root.spawn((DebugPanel,Node { position_type: PositionType::Absolute, top: px(24), right: px(24), width: px(314), padding: UiRect::all(px(18)), flex_direction: FlexDirection::Column, row_gap: px(14), border_radius: BorderRadius::ZERO, ..default() }, BackgroundColor(theme::PANEL)))
            .with_children(|panel| {
                panel.spawn((StatusText,Text::new("Initialisation..."),font(13.0, &ui_font),TextColor(theme::MUTED)));
                panel.spawn((TargetText,Text::new("Terre"),font(15.0, &ui_font),TextColor(theme::ACCENT)));
            });
        root.spawn((Node { position_type: PositionType::Absolute, bottom: px(24), left: px(24), right: px(24), padding: UiRect::all(px(14)), border_radius: BorderRadius::ZERO, ..default() }, BackgroundColor(theme::PANEL)))
            .with_child((HelpText,Text::new("Clic droit : piloter  ·  Échap : curseur  ·  WASD / ZQSD : vol  ·  Espace : saut / monter  ·  Ctrl : descendre en vol  ·  Shift : course / ×10\nMolette / Pg↑ Pg↓ : vitesse  ·  Tab / 0–9 : cible  ·  F : centrer  ·  G : visite spatiale  ·  M : surface Mars / retour  ·  U : repère solaire  ·  T : temps ×1/10/100/1000  ·  P : pause  ·  H : télémétrie · N : navigation"),font(13.0, &ui_font),TextColor(theme::MUTED)));
        for body in &solar.bodies {
            root.spawn((Marker(body.id), Node { position_type: PositionType::Absolute, display: Display::None, ..default() }, Text::new(format!("◇ {}",body.name)),font(12.0, &ui_font),TextColor(Color::srgb(0.48,0.70,0.79))));
        }
    });
}
fn buttons(
    mut menu: ResMut<menu::MenuState>,
    mut query: Query<(Ref<Interaction>, &Action, &mut BackgroundColor)>,
    mut pilot: ResMut<Explorer>,
    mut request: ResMut<NavigationRequest>,
) {
    if menu.is_open() {
        return;
    }
    for (interaction, action, mut color) in &mut query {
        let selected = matches!(action, Action::Select(id) if *id == pilot.selected);
        color.0 = match *interaction {
            Interaction::Hovered => theme::HOVER,
            Interaction::Pressed => theme::ACTIVE,
            Interaction::None if selected => theme::ACTIVE,
            _ => theme::BUTTON,
        };
        if interaction.is_changed() && *interaction == Interaction::Pressed {
            match *action {
                Action::Select(id) => pilot.selected = id,
                Action::Visit => request.visit = true,
                Action::Center => request.center = true,
                Action::Overview => request.overview = true,
                Action::Surface => request.surface = true,
                Action::Menu => menu.open(),
            }
        }
    }
}
#[expect(
    clippy::type_complexity,
    reason = "Filtres ECS disjoints des panneaux du HUD."
)]
fn toggle(
    keys: Res<ButtonInput<KeyCode>>,
    menu: Res<menu::MenuState>,
    mut state: ResMut<HudState>,
    mut root: Single<&mut Node, (With<HudRoot>, Without<NavigationPanel>, Without<DebugPanel>)>,
    mut navigation: Single<
        &mut Node,
        (With<NavigationPanel>, Without<HudRoot>, Without<DebugPanel>),
    >,
    mut debug: Single<&mut Node, (With<DebugPanel>, Without<HudRoot>, Without<NavigationPanel>)>,
    pilot: Res<Explorer>,
) {
    if !menu.is_open() && keys.just_pressed(KeyCode::KeyH) {
        state.visible = !state.visible;
    }
    if !menu.is_open() && keys.just_pressed(KeyCode::KeyN) {
        state.navigation = !state.navigation;
    }
    navigation.display = if pilot.mode == MovementMode::Space || state.navigation {
        Display::Flex
    } else {
        Display::None
    };
    navigation.top = px(24);
    navigation.left = if pilot.mode == MovementMode::Space {
        px(24)
    } else {
        auto()
    };
    navigation.right = if pilot.mode == MovementMode::Surface {
        px(24)
    } else {
        auto()
    };
    debug.display = if state.visible && !state.navigation {
        Display::Flex
    } else {
        Display::None
    };
    root.display = if !menu.is_open() {
        Display::Flex
    } else {
        Display::None
    };
}
#[expect(
    clippy::type_complexity,
    reason = "ParamSet garantit les emprunts exclusifs des deux textes ECS."
)]
#[expect(
    clippy::too_many_arguments,
    reason = "Paramètres de système injectés par Bevy ECS."
)]
fn refresh(
    time: Res<Time<Real>>,
    mut elapsed: Local<f64>,
    pilot: Res<Explorer>,
    clock: Res<SimulationClock>,
    solar: Res<SolarSystem>,
    diagnostics: Res<DiagnosticsStore>,
    surface: Res<SurfaceState>,
    terrain: Res<crate::planets::terrain::MarsTerrain>,
    mut texts: ParamSet<(
        Single<&mut Text, With<StatusText>>,
        Single<&mut Text, With<TargetText>>,
    )>,
) {
    *elapsed += time.delta_secs_f64();
    if *elapsed < 0.1 {
        return;
    }
    *elapsed = 0.0;
    let global = pilot.global(&solar, clock.jd);
    let body = solar.body(pilot.selected);
    let distance = global.distance(solar.position(pilot.selected, clock.jd));
    let nearest = solar
        .bodies
        .iter()
        .min_by(|a, b| {
            let da = global.distance(solar.position(a.id, clock.jd)) - a.radius_m;
            let db = global.distance(solar.position(b.id, clock.jd)) - b.radius_m;
            da.total_cmp(&db)
        })
        .unwrap();
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|d| d.smoothed())
        .unwrap_or(0.0);
    let anchor = pilot
        .anchor
        .map_or("Héliocentrique", |id| solar.body(id).name);
    let anchor = if pilot.mode == MovementMode::Surface {
        "Mars · site tournant"
    } else {
        anchor
    };
    let mode = if pilot.mode == MovementMode::Surface {
        if terrain.globe.is_some() {
            "Mars · terrain mondial"
        } else if terrain.cube.is_some() {
            "Mars · paire cube-sphere"
        } else {
            "surface Mars"
        }
    } else {
        "espace"
    };
    let lod = terrain
        .globe
        .as_ref()
        .filter(|_| pilot.mode == MovementMode::Surface)
        .map_or(String::new(), |g| {
            let (details, _, triangles) = g.detail_counts();
            format!(
                "\nLOD 32/16/8 : {}/{}/{} · {} triangles · {} changements",
                details[0], details[1], details[2], triangles, g.lod_changes
            )
        });
    let movement = if pilot.mode == MovementMode::Surface {
        format!(
            "{} · {}\nVitesse / site : {:.2} m/s\nHauteur pieds/sol : {:.2} m",
            if pilot.boosted { "Course" } else { "Marche" },
            if surface.grounded {
                "au sol"
            } else {
                "en l’air"
            },
            surface.velocity.length(),
            surface.feet.y - crate::physics::surface::support_height(&terrain, surface.feet)
        )
    } else {
        Explorer::SPEED_NAMES[pilot.speed_index].to_owned()
    };
    let weather = if pilot.mode == MovementMode::Surface && terrain.globe.is_some() {
        let w = crate::rendering::atmosphere::current(&terrain, &surface, &solar, &clock);
        format!(
            "\n{} · visibilité {:.1} km\nPression moyenne ≈ 600 Pa (0,6 % Terre)",
            if w.storm > 0.35 {
                "Tempête de poussière"
            } else if w.clouds > 0.45 {
                "Voiles de glace"
            } else {
                "Ciel poussiéreux"
            },
            w.visibility_m().1 / 1000.0
        )
    } else {
        String::new()
    };
    let movement = movement + &lod + &weather;
    ***texts.p0() = format!(
        "TÉLÉMÉTRIE\n\n{fps:.0} FPS  ·  Mode {mode}\nJour julien : {:.5}\nJ2000 + {:.3} jours\nTemps : ×{:.0} {}\n\n{}\n{} / s{}\nRepère : {anchor}\nOffset : {}\nPlus proche : {}\n\n{}",
        clock.jd,
        clock.jd - 2451545.0,
        SimulationClock::SPEEDS[clock.speed_index],
        if clock.paused { "(pause)" } else { "" },
        movement,
        format_distance(pilot.speed()),
        if pilot.boosted { " (Shift)" } else { "" },
        format_distance(pilot.local_m.length()),
        nearest.name,
        if pilot.captured {
            "PILOTAGE ACTIF"
        } else {
            "CLIC DROIT POUR PILOTER"
        }
    );
    ***texts.p1() = format!(
        "CIBLE : {}\n\nDistance au centre : {}\nAltitude : {}\nRayon moyen : {}\n{}\n{}",
        body.name,
        format_distance(distance),
        format_distance((distance - body.radius_m).max(0.0)),
        format_distance(body.radius_m),
        body.orbit.map_or("".into(), |o| format!(
            "Révolution : {:.2} jours",
            o.period_seconds() / DAY
        )),
        match body.surface {
            crate::universe::celestial_body::SurfaceKind::Gas
            | crate::universe::celestial_body::SurfaceKind::IceGiant =>
                "Enveloppe géante · sans sol ordinaire",
            crate::universe::celestial_body::SurfaceKind::Plasma => "Étoile · plasma",
            _ if body.id == BodyId::MARS => "M : surface Mars / retour spatial",
            _ => "Surface solide · visite spatiale",
        }
    );
}
#[expect(
    clippy::too_many_arguments,
    reason = "Paramètres ECS des marqueurs et de leur occultation locale."
)]
fn markers(
    settings: Res<menu::PlayerSettings>,
    pilot: Res<Explorer>,
    solar: Res<SolarSystem>,
    clock: Res<SimulationClock>,
    window: Single<&Window, With<PrimaryWindow>>,
    terrain: Res<crate::planets::terrain::MarsTerrain>,
    state: Res<SurfaceState>,
    mut markers: Query<(&Marker, &mut Node, &mut TextColor)>,
) {
    let position = pilot.global(&solar, clock.jd);
    let width = window.width();
    let height = window.height();
    let focal = height * 0.5 / (settings.fov_degrees().to_radians() * 0.5).tan();
    for (marker, mut node, mut color) in &mut markers {
        let relative = solar.position(marker.0, clock.jd) - position;
        if pilot.mode == MovementMode::Surface {
            let frame = terrain.inertial_frame(
                solar
                    .body(BodyId::MARS)
                    .rotation(clock.seconds_since_epoch()),
            );
            let direction = frame.orientation.inverse() * relative.normalize_or_zero();
            let eye = state.eye(&terrain);
            let start = terrain.body_relative(eye);
            let end = start + terrain.orientation * direction * 120.0;
            let hidden = direction.dot(terrain.site_relative(eye).normalize()) <= 0.0
                || terrain
                    .building_solids
                    .iter()
                    .any(|solid| solid.blocks_segment(start, end))
                || (0..3).any(|i| {
                    let (center, size, orientation) =
                        crate::physics::surface::obstacle_pose(&terrain, i);
                    crate::physics::surface::BodyBox {
                        center: terrain.body_relative(center),
                        size,
                        orientation: terrain.orientation * orientation,
                    }
                    .blocks_segment(start, end)
                });
            if hidden {
                node.display = Display::None;
                continue;
            }
        }
        let local = (pilot.rotation.inverse() * relative.normalize_or_zero()).as_vec3();
        if local.z >= -0.001 || solar.body(marker.0).radius_m / relative.length().max(1.0) > 0.025 {
            node.display = Display::None;
            continue;
        }
        let x = width * 0.5 + local.x / (-local.z) * focal;
        let y = height * 0.5 - local.y / (-local.z) * focal;
        node.display = if (250.0..width - 340.0).contains(&x) && (20.0..height - 105.0).contains(&y)
        {
            Display::Flex
        } else {
            Display::None
        };
        node.left = px(x);
        node.top = px(y);
        color.0 = if marker.0 == pilot.selected {
            Color::srgb(1.0, 0.82, 0.38)
        } else {
            Color::srgb(0.48, 0.70, 0.79)
        };
    }
}

fn target_camera(
    mut commands: Commands,
    root: Single<Entity, With<HudRoot>>,
    camera: Single<Entity, With<crate::camera::GameCamera>>,
    mut previous: Local<Option<Entity>>,
) {
    let camera = *camera;
    if *previous != Some(camera) {
        commands.entity(*root).insert(UiTargetCamera(camera));
        *previous = Some(camera);
    }
}

fn help(
    pilot: Res<Explorer>,
    mut text: Single<&mut Text, With<HelpText>>,
    mut previous: Local<Option<MovementMode>>,
) {
    if *previous == Some(pilot.mode) {
        return;
    }
    *previous = Some(pilot.mode);
    ***text = if pilot.mode == MovementMode::Surface {
        "WASD / ZQSD : marcher · Shift : courir · Espace : sauter · E : récolte / coffre · I : inventaire · B : construction\nClic droit : piloter · Échap : curseur / menu · M : espace · N : navigation · H : télémétrie".into()
    } else {
        "Clic droit : piloter · Échap : curseur · WASD / ZQSD : vol · Espace / Ctrl : monter / descendre · Shift : ×10\nMolette / Pg↑ Pg↓ : vitesse · Tab / 0–9 : cible · F : centrer · G : visiter · M : surface Mars · O : système · U : repère solaire · T / P : temps · H : télémétrie · N : navigation".into()
    };
}
