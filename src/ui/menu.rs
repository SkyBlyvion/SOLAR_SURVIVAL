//! Menu et réglages de session. Le calendrier orbital continue indépendamment de l'UI.
use crate::{
    GameSet,
    camera::{GameCamera, controls},
    config::GameConfig,
    player::Explorer,
};
use bevy::{
    app::AppExit,
    prelude::*,
    window::{MonitorSelection, PrimaryWindow, WindowMode},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Screen {
    Main,
    Options,
    Credits,
    Playing,
}

#[derive(Resource)]
pub struct MenuState {
    screen: Screen,
    started: bool,
}
impl MenuState {
    pub fn is_open(&self) -> bool {
        self.screen != Screen::Playing
    }
    pub fn open(&mut self) {
        self.screen = Screen::Main;
    }
    pub(crate) fn play(&mut self) {
        self.started = true;
        self.screen = Screen::Playing;
    }
    fn back(&mut self) {
        if self.screen == Screen::Main && self.started {
            self.screen = Screen::Playing;
        } else {
            self.screen = Screen::Main;
        }
    }
}

#[derive(Resource)]
pub struct PlayerSettings {
    sensitivity_index: usize,
    fov_index: usize,
    msaa_index: usize,
    fullscreen: bool,
}
impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            sensitivity_index: 1,
            fov_index: 1,
            msaa_index: 2,
            fullscreen: false,
        }
    }
}
impl PlayerSettings {
    pub fn sensitivity(&self) -> f64 {
        [0.5, 1.0, 1.5, 2.0][self.sensitivity_index]
    }
    pub fn fov_degrees(&self) -> f32 {
        [55.0, 65.0, 75.0, 90.0][self.fov_index]
    }
    fn msaa(&self) -> Msaa {
        [Msaa::Off, Msaa::Sample2, Msaa::Sample4][self.msaa_index]
    }
}

#[derive(Component)]
struct MenuRoot;
#[derive(Component)]
struct Panel(Screen);
#[derive(Component, Clone, Copy)]
enum MenuAction {
    Play,
    Options,
    Credits,
    Back,
    Sensitivity,
    Fov,
    Msaa,
    Fullscreen,
    Reset,
    Quit,
}
#[derive(Component)]
struct Label(MenuAction);

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) struct MenuInputSet;
pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        let automated = app.world().resource::<GameConfig>().automated();
        app.insert_resource(MenuState {
            screen: if automated {
                Screen::Playing
            } else {
                Screen::Main
            },
            started: automated,
        })
        .init_resource::<PlayerSettings>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            input
                .before(controls)
                .in_set(MenuInputSet)
                .in_set(GameSet::Input),
        )
        .add_systems(Update, apply_settings.in_set(GameSet::Rendering))
        .add_systems(Update, display.in_set(GameSet::Interface));
    }
}

fn button(parent: &mut ChildSpawnerCommands, action: MenuAction, handle: &Handle<Font>) {
    parent
        .spawn((
            Button,
            action,
            Node {
                width: percent(100),
                padding: UiRect::axes(px(20), px(12)),
                border_radius: BorderRadius::all(px(6)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.055, 0.12, 0.17)),
        ))
        .with_child((
            Label(action),
            Text::new(""),
            super::font(17.0, handle),
            TextColor(Color::srgb(0.83, 0.93, 0.98)),
        ));
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let handle = assets.load("fonts/DejaVuSans.ttf");
    commands.spawn((MenuRoot, GlobalZIndex(100),
        Node { width: percent(100), height: percent(100), position_type: PositionType::Absolute,
            align_items: AlignItems::Center, justify_content: JustifyContent::Center, ..default() },
        BackgroundColor(Color::srgba(0.008, 0.015, 0.025, 0.93)),
    )).with_children(|root| {
        root.spawn((Node { width: percent(90), max_width: px(540), padding: UiRect::all(px(28)),
            flex_direction: FlexDirection::Column, row_gap: px(12),
            border_radius: BorderRadius::all(px(12)), ..default() },
            BackgroundColor(Color::srgb(0.018, 0.035, 0.052)),
        )).with_children(|card| {
            card.spawn((Text::new("SOLAR SURVIVAL"), super::font(30.0, &handle),
                TextColor(Color::srgb(0.78, 0.9, 1.0))));
            card.spawn((Text::new("0.2  ·  EXPLORATION & PROTOTYPE MARS"), super::font(12.0, &handle),
                TextColor(Color::srgb(0.46, 0.66, 0.77))));
            for screen in [Screen::Main, Screen::Options, Screen::Credits] {
                card.spawn((Panel(screen), Node { flex_direction: FlexDirection::Column,
                    row_gap: px(9), ..default() })).with_children(|panel| {
                    match screen {
                        Screen::Main => {
                            panel.spawn((Text::new("Un système solaire à explorer.\nUn premier pas sur Mars."),
                                super::font(16.0, &handle), TextColor(Color::srgb(0.66, 0.77, 0.83))));
                            for action in [MenuAction::Play, MenuAction::Options, MenuAction::Credits, MenuAction::Quit] {
                                button(panel, action, &handle);
                            }
                        }
                        Screen::Options => {
                            panel.spawn((Text::new("OPTIONS"), super::font(20.0, &handle)));
                            for action in [MenuAction::Sensitivity, MenuAction::Fov, MenuAction::Msaa,
                                MenuAction::Fullscreen, MenuAction::Reset, MenuAction::Back] {
                                button(panel, action, &handle);
                            }
                            panel.spawn((Text::new("Cliquez pour changer de valeur.\nRéglages conservés pendant cette session.\nLe calendrier continue ; P le met en pause en jeu."),
                                super::font(12.0, &handle), TextColor(Color::srgb(0.52, 0.68, 0.76))));
                        }
                        Screen::Credits => {
                            panel.spawn((Text::new("CRÉDITS"), super::font(20.0, &handle)));
                            panel.spawn((Text::new("Solar Survival · prototype 0.2\nProjet Rust / Bevy · licence MIT\n\nMoteur : Bevy et ses contributeurs (MIT / Apache-2.0).\nDonnées astronomiques : NASA et JPL.\nPolice : DejaVu Sans, licence embarquée dans assets/fonts.\n\nDéveloppement assisté par OpenAI Codex.\nRelief et textures artistiques ; aucune cartographie\nréelle ni orientation IAU exacte revendiquée."),
                                super::font(15.0, &handle), TextColor(Color::srgb(0.72, 0.82, 0.88))));
                            button(panel, MenuAction::Back, &handle);
                        }
                        Screen::Playing => unreachable!(),
                    }
                });
            }
        });
    });
}

fn input(
    keys: Res<ButtonInput<KeyCode>>,
    pilot: Res<Explorer>,
    construction: Option<Res<crate::construction::Construction>>,
    mut state: ResMut<MenuState>,
    mut settings: ResMut<PlayerSettings>,
    mut interactions: Query<
        (&Interaction, &MenuAction, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut exit: MessageWriter<AppExit>,
) {
    if keys.just_pressed(KeyCode::Escape) && !construction.is_some_and(|c| c.active) {
        if state.is_open() {
            state.back();
        } else if !pilot.captured {
            state.open();
        }
    }
    if state.screen == Screen::Main && keys.just_pressed(KeyCode::Enter) {
        state.play();
    }
    for (interaction, action, mut color) in &mut interactions {
        color.0 = match interaction {
            Interaction::Hovered => Color::srgb(0.1, 0.24, 0.31),
            Interaction::Pressed => Color::srgb(0.16, 0.34, 0.42),
            Interaction::None => Color::srgb(0.055, 0.12, 0.17),
        };
        if !state.is_open() || *interaction != Interaction::Pressed {
            continue;
        }
        // Les panneaux cachés ne doivent pas déclencher d'action.
        let expected = match action {
            MenuAction::Sensitivity
            | MenuAction::Fov
            | MenuAction::Msaa
            | MenuAction::Fullscreen
            | MenuAction::Reset => Screen::Options,
            MenuAction::Back => state.screen,
            _ => Screen::Main,
        };
        if state.screen != expected {
            continue;
        }
        match action {
            MenuAction::Play => state.play(),
            MenuAction::Options => state.screen = Screen::Options,
            MenuAction::Credits => state.screen = Screen::Credits,
            MenuAction::Back => state.back(),
            MenuAction::Sensitivity => {
                settings.sensitivity_index = (settings.sensitivity_index + 1) % 4
            }
            MenuAction::Fov => settings.fov_index = (settings.fov_index + 1) % 4,
            MenuAction::Msaa => settings.msaa_index = (settings.msaa_index + 1) % 3,
            MenuAction::Fullscreen => settings.fullscreen = !settings.fullscreen,
            MenuAction::Reset => *settings = PlayerSettings::default(),
            MenuAction::Quit => {
                exit.write(AppExit::Success);
            }
        }
    }
}

fn apply_settings(
    settings: Res<PlayerSettings>,
    config: Res<GameConfig>,
    mut cameras: Query<(&mut Projection, &mut Msaa), With<GameCamera>>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
) {
    if !settings.is_changed() || config.profile.is_some() {
        return;
    }
    for (mut projection, mut msaa) in &mut cameras {
        if let Projection::Perspective(ref mut perspective) = *projection {
            perspective.fov = settings.fov_degrees().to_radians();
        }
        *msaa = settings.msaa();
    }
    window.mode = if settings.fullscreen {
        WindowMode::BorderlessFullscreen(MonitorSelection::Current)
    } else {
        WindowMode::Windowed
    };
}

#[expect(
    clippy::too_many_arguments,
    reason = "Paramètres ECS des panneaux et de leur caméra cible."
)]
fn display(
    mut commands: Commands,
    state: Res<MenuState>,
    settings: Res<PlayerSettings>,
    root: Single<(Entity, &mut Node), With<MenuRoot>>,
    mut panels: Query<(&Panel, &mut Node), Without<MenuRoot>>,
    mut labels: Query<(&Label, &mut Text)>,
    camera: Single<Entity, With<GameCamera>>,
    mut previous: Local<Option<Entity>>,
) {
    let camera = *camera;
    let (root, mut node) = root.into_inner();
    if *previous != Some(camera) {
        commands.entity(root).insert(UiTargetCamera(camera));
        *previous = Some(camera);
    }
    node.display = if state.is_open() {
        Display::Flex
    } else {
        Display::None
    };
    if !state.is_changed() && !settings.is_changed() {
        return;
    }
    for (panel, mut node) in &mut panels {
        node.display = if panel.0 == state.screen {
            Display::Flex
        } else {
            Display::None
        };
    }
    for (label, mut text) in &mut labels {
        **text = match label.0 {
            MenuAction::Play => if state.started { "Reprendre" } else { "Jouer" }.into(),
            MenuAction::Options => "Options".into(),
            MenuAction::Credits => "Crédits".into(),
            MenuAction::Quit => "Quitter".into(),
            MenuAction::Back => "Retour".into(),
            MenuAction::Reset => "Rétablir les valeurs par défaut".into(),
            MenuAction::Sensitivity => {
                format!("Sensibilité souris : ×{:.1}", settings.sensitivity())
            }
            MenuAction::Fov => format!("Champ de vision vertical : {:.0}°", settings.fov_degrees()),
            MenuAction::Msaa => format!(
                "Anticrénelage MSAA : {}",
                ["désactivé", "×2", "×4"][settings.msaa_index]
            ),
            MenuAction::Fullscreen => format!(
                "Affichage : {}",
                if settings.fullscreen {
                    "plein écran"
                } else {
                    "fenêtré"
                }
            ),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::MovementMode;
    use crate::player::surface::SurfaceState;
    use crate::{
        camera::{ExplorationPlugin, NavigationRequest},
        universe::{SimulationClock, UniversePlugin},
    };
    use bevy::{
        input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
        time::TimeUpdateStrategy,
        window::CursorOptions,
    };
    use std::time::Duration;

    fn app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(GameConfig::default())
            .insert_resource(MenuState {
                screen: Screen::Main,
                started: false,
            })
            .init_resource::<PlayerSettings>()
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<ButtonInput<MouseButton>>()
            .init_resource::<AccumulatedMouseMotion>()
            .init_resource::<AccumulatedMouseScroll>()
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(
                1.0 / 60.0,
            )))
            .configure_sets(
                Update,
                (
                    GameSet::Input,
                    GameSet::Simulation,
                    GameSet::Navigation,
                    GameSet::Rendering,
                )
                    .chain(),
            )
            .add_plugins((UniversePlugin, ExplorationPlugin))
            .add_systems(
                Update,
                input
                    .before(controls)
                    .in_set(MenuInputSet)
                    .in_set(GameSet::Input),
            )
            .add_systems(Update, apply_settings.in_set(GameSet::Rendering));
        app.world_mut().spawn((
            Window {
                focused: true,
                ..default()
            },
            CursorOptions::default(),
            PrimaryWindow,
        ));
        app.update();
        app
    }
    fn press(app: &mut App, key: KeyCode) {
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(key);
        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .reset_all();
    }
    fn click(app: &mut App, action: MenuAction) {
        let entity = app
            .world_mut()
            .spawn((action, Interaction::Pressed, BackgroundColor::default()))
            .id();
        app.update();
        app.world_mut().despawn(entity);
    }

    #[test]
    fn menu_suspends_oxygen_and_restart_but_keeps_orbit_and_gravity() {
        let mut app = app();
        app.add_plugins(crate::survival::SurvivalPlugin);
        press(&mut app, KeyCode::Enter);
        press(&mut app, KeyCode::KeyM);
        let jd = app.world().resource::<SimulationClock>().jd;
        let before = app
            .world()
            .resource::<crate::survival::Oxygen>()
            .remaining_seconds;
        app.world_mut().resource_mut::<MenuState>().open();
        app.world_mut().resource_mut::<SurfaceState>().feet.y += 2.0;
        let feet = app.world().resource::<SurfaceState>().feet;
        app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs(12)));
        app.update();
        assert_eq!(
            app.world()
                .resource::<crate::survival::Oxygen>()
                .remaining_seconds,
            before
        );
        assert!(app.world().resource::<SimulationClock>().jd > jd);
        assert!(app.world().resource::<SurfaceState>().feet.y < feet.y);
        {
            let mut o = app.world_mut().resource_mut::<crate::survival::Oxygen>();
            o.failed = true;
            o.remaining_seconds = 0.0;
        }
        press(&mut app, KeyCode::KeyR);
        assert!(app.world().resource::<crate::survival::Oxygen>().failed);
        press(&mut app, KeyCode::Enter);
        press(&mut app, KeyCode::KeyR);
        assert!(!app.world().resource::<crate::survival::Oxygen>().failed);
    }

    #[test]
    fn menu_blocks_navigation_and_capture_but_not_real_orbital_clock() {
        let mut app = app();
        let position = app.world().resource::<Explorer>().local_m;
        let jd = app.world().resource::<SimulationClock>().jd;
        app.world_mut().resource_mut::<Explorer>().captured = true;
        app.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .press(MouseButton::Right);
        for key in [KeyCode::KeyM, KeyCode::KeyT, KeyCode::KeyP, KeyCode::KeyW] {
            app.world_mut()
                .resource_mut::<ButtonInput<KeyCode>>()
                .press(key);
        }
        app.update();
        let pilot = app.world().resource::<Explorer>();
        assert!(!pilot.captured);
        assert_eq!(pilot.mode, MovementMode::Space);
        assert!(pilot.local_m.distance(position) < 0.001);
        let clock = app.world().resource::<SimulationClock>();
        assert!(clock.jd > jd);
        assert!(!clock.paused);
        assert_eq!(clock.speed_index, 0);
        assert!(!app.world().resource::<NavigationRequest>().surface);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .reset_all();
        app.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .reset_all();
        press(&mut app, KeyCode::Enter);
        assert!(!app.world().resource::<MenuState>().is_open());
        assert!(app.world().resource::<MenuState>().started);
        assert!(
            app.world()
                .resource::<Explorer>()
                .local_m
                .distance(position)
                < 0.001
        );
    }

    #[test]
    fn escape_releases_cursor_then_opens_menu_and_returns_through_credits() {
        let mut app = app();
        click(&mut app, MenuAction::Play);
        app.world_mut().resource_mut::<Explorer>().captured = true;
        press(&mut app, KeyCode::Escape);
        assert!(!app.world().resource::<Explorer>().captured);
        assert!(!app.world().resource::<MenuState>().is_open());
        press(&mut app, KeyCode::Escape);
        assert_eq!(app.world().resource::<MenuState>().screen, Screen::Main);
        click(&mut app, MenuAction::Credits);
        assert_eq!(app.world().resource::<MenuState>().screen, Screen::Credits);
        press(&mut app, KeyCode::Escape);
        assert_eq!(app.world().resource::<MenuState>().screen, Screen::Main);
        click(&mut app, MenuAction::Play);
        assert!(!app.world().resource::<MenuState>().is_open());
    }

    #[test]
    fn options_apply_to_both_cameras_and_window_then_reset() {
        let mut app = app();
        click(&mut app, MenuAction::Options);
        for action in [
            MenuAction::Sensitivity,
            MenuAction::Fov,
            MenuAction::Msaa,
            MenuAction::Fullscreen,
        ] {
            click(&mut app, action);
        }
        assert_eq!(app.world().resource::<PlayerSettings>().sensitivity(), 1.5);
        assert_eq!(app.world().resource::<PlayerSettings>().fov_degrees(), 75.0);
        assert!(matches!(
            app.world_mut()
                .query::<&Window>()
                .single(app.world())
                .unwrap()
                .mode,
            WindowMode::BorderlessFullscreen(_)
        ));
        for (projection, msaa) in app
            .world_mut()
            .query::<(&Projection, &Msaa)>()
            .iter(app.world())
        {
            let Projection::Perspective(p) = projection else {
                panic!("projection perspective attendue")
            };
            assert_eq!(p.fov, 75_f32.to_radians());
            assert_eq!(*msaa, Msaa::Off);
        }
        click(&mut app, MenuAction::Reset);
        assert_eq!(app.world().resource::<PlayerSettings>().sensitivity(), 1.0);
        for (projection, msaa) in app
            .world_mut()
            .query::<(&Projection, &Msaa)>()
            .iter(app.world())
        {
            let Projection::Perspective(p) = projection else {
                panic!("projection perspective attendue")
            };
            assert_eq!(p.fov, 65_f32.to_radians());
            assert_eq!(*msaa, Msaa::Sample4);
        }
        assert_eq!(
            app.world_mut()
                .query::<&Window>()
                .single(app.world())
                .unwrap()
                .mode,
            WindowMode::Windowed
        );
    }
    #[test]
    fn placement_escape_cancels_before_menu_and_menu_preserves_inventory() {
        let mut app = app();
        app.add_plugins((
            crate::survival::SurvivalPlugin,
            crate::construction::ConstructionPlugin,
        ));
        click(&mut app, MenuAction::Play);
        press(&mut app, KeyCode::KeyM);
        press(&mut app, KeyCode::KeyB);
        assert!(
            app.world()
                .resource::<crate::construction::Construction>()
                .active
        );
        press(&mut app, KeyCode::Escape);
        assert!(
            !app.world()
                .resource::<crate::construction::Construction>()
                .active
        );
        assert!(!app.world().resource::<MenuState>().is_open());
        app.world_mut()
            .resource_mut::<crate::construction::Construction>()
            .alloy = 12;
        press(&mut app, KeyCode::KeyB);
        click(&mut app, MenuAction::Play); // Bouton inactif en jeu.
        app.world_mut().resource_mut::<MenuState>().open();
        app.update();
        press(&mut app, KeyCode::Enter);
        assert!(
            !app.world()
                .resource::<crate::construction::Construction>()
                .active
        );
        assert_eq!(
            app.world()
                .resource::<crate::construction::Construction>()
                .alloy,
            12
        );
        assert_eq!(
            app.world()
                .resource::<crate::buildings::Buildings>()
                .shelters
                .len(),
            1
        );
    }
}
