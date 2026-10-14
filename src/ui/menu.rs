//! Menus de partie, profils nommés et options. Une seule caméra de jeu.
use super::theme;
use crate::{
    GameSet,
    camera::{GameCamera, controls},
    config::GameConfig,
    persistence::{SaveAction, SaveControl},
    player::Explorer,
};
use bevy::{
    app::AppExit,
    input::{
        ButtonState,
        keyboard::{Key, KeyboardInput},
    },
    prelude::*,
    window::{MonitorSelection, PrimaryWindow, WindowMode},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
enum Screen {
    #[default]
    Main,
    NewAdventure,
    Profiles,
    Options,
    Credits,
    Playing,
}
#[derive(Resource, Default)]
pub struct MenuState {
    screen: Screen,
    started: bool,
}
impl MenuState {
    pub fn is_open(&self) -> bool {
        self.screen != Screen::Playing
    }
    pub(crate) fn loaded(&mut self) {
        self.started = true;
        self.screen = Screen::Main;
    }
    pub fn started(&self) -> bool {
        self.started
    }
    pub fn open(&mut self) {
        self.screen = Screen::Main;
    }
    pub(crate) fn play(&mut self) {
        self.started = true;
        self.screen = Screen::Playing;
    }
    fn back(&mut self) {
        self.screen = if self.screen == Screen::Main && self.started {
            Screen::Playing
        } else {
            Screen::Main
        };
    }
}
#[derive(Resource, Clone)]
pub struct PlayerSettings {
    pub(crate) sensitivity_index: usize,
    pub(crate) fov_index: usize,
    pub(crate) msaa_index: usize,
    pub(crate) fullscreen: bool,
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
struct Brand;
#[derive(Component)]
struct MenuRoot;
#[derive(Component)]
struct Panel(Screen);
#[derive(Component)]
struct SaveStatus;
#[derive(Component)]
struct DraftText;
#[derive(Component)]
struct ProfileList;
#[derive(Component, Clone, Copy, PartialEq)]
enum MenuAction {
    Play,
    New,
    Create,
    Choose(usize),
    OpenSelected,
    Previous,
    Next,
    Options,
    Credits,
    Back,
    Sensitivity,
    Fov,
    Msaa,
    Fullscreen,
    Reset,
    Save,
    Load,
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
        .add_message::<KeyboardInput>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (text_input, input)
                .chain()
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
                padding: UiRect::axes(px(18), px(13)),
                border: UiRect::left(px(2)),
                ..default()
            },
            BorderColor::all(theme::MUTED),
            BackgroundColor(theme::BUTTON),
        ))
        .with_child((
            Label(action),
            Text::new(""),
            super::font(17.0, handle),
            TextColor(theme::TEXT),
        ));
}
fn heading(parent: &mut ChildSpawnerCommands, text: &str, font: &Handle<Font>) {
    parent.spawn((
        Text::new(text),
        super::font(24.0, font),
        TextColor(theme::TEXT),
    ));
}
fn description(parent: &mut ChildSpawnerCommands, text: &str, font: &Handle<Font>) {
    parent.spawn((
        Text::new(text),
        super::font(14.0, font),
        TextColor(theme::MUTED),
    ));
}
fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts/DejaVuSans.ttf");
    commands.spawn((MenuRoot,GlobalZIndex(100),Node {width:percent(100),height:percent(100),position_type:PositionType::Absolute,
        align_items:AlignItems::Center,padding:UiRect::axes(vw(6.0),px(28)),column_gap:vw(8.0),..default()},
        BackgroundColor(Color::srgba(0.025,0.03,0.03,0.93)))).with_children(|root| {
        root.spawn(Node {width:percent(48),max_width:px(560),flex_direction:FlexDirection::Column,row_gap:px(24),..default()})
            .with_children(|left| {
                left.spawn((Brand,Text::new("SOLAR\nSURVIVAL"),super::font(48.0,&font),TextColor(theme::TEXT)));
                for screen in [Screen::Main,Screen::NewAdventure,Screen::Profiles,Screen::Options,Screen::Credits] {
                    left.spawn((Panel(screen),Node {flex_direction:FlexDirection::Column,row_gap:px(8),..default()})).with_children(|panel| {
                        match screen {
                            Screen::Main=>{
                                for action in [MenuAction::Play,MenuAction::New,MenuAction::Load,MenuAction::Save,MenuAction::Options,MenuAction::Credits,MenuAction::Quit] {button(panel,action,&font);}
                            }
                            Screen::NewAdventure=>{
                                heading(panel,"Nouvelle aventure",&font);
                                description(panel,"Nom de l’aventure\nUn répertoire indépendant sera créé pour cette partie.",&font);
                                panel.spawn((Node {padding:UiRect::all(px(16)),border:UiRect::bottom(px(2)),..default()},BackgroundColor(theme::BUTTON),BorderColor::all(theme::ACCENT)))
                                    .with_child((DraftText,Text::new(""),super::font(23.0,&font),TextColor(theme::TEXT)));
                                description(panel,"40 caractères maximum · lettres, chiffres, espaces, tirets\nVotre aventure active sera sauvegardée avant le changement.",&font);
                                button(panel,MenuAction::Create,&font);button(panel,MenuAction::Back,&font);
                            }
                            Screen::Profiles=>{
                                heading(panel,"Charger une aventure",&font);
                                panel.spawn((ProfileList,Node {flex_direction:FlexDirection::Column,row_gap:px(6),..default()}));
                                panel.spawn(Node {flex_direction:FlexDirection::Row,column_gap:px(8),..default()}).with_children(|row| {
                                    button(row,MenuAction::Previous,&font);button(row,MenuAction::Next,&font);
                                });
                                button(panel,MenuAction::OpenSelected,&font);button(panel,MenuAction::Back,&font);
                                description(panel,"↑ / ↓ : sélectionner · Entrée : charger\nLes aventures ont des inventaires et des constructions séparés.",&font);
                            }
                            Screen::Options=>{
                                heading(panel,"Options",&font);
                                for action in [MenuAction::Sensitivity,MenuAction::Fov,MenuAction::Msaa,MenuAction::Fullscreen,MenuAction::Reset,MenuAction::Back] {button(panel,action,&font);}
                                description(panel,"Cliquez pour changer une valeur.\nCes réglages sont inclus dans les sauvegardes.",&font);
                            }
                            Screen::Credits=>{
                                heading(panel,"Crédits",&font);
                                description(panel,"Solar Survival · Rust / Bevy · licence MIT\n\nMoteur : Bevy (MIT / Apache-2.0)\nDonnées astronomiques : NASA / JPL\nPolice : DejaVu Sans\n\nDéveloppement assisté par OpenAI Codex.\nTextures, ciel et terrain de présentation artistiques.",&font);
                                button(panel,MenuAction::Back,&font);
                            }
                            Screen::Playing=>unreachable!(),
                        }
                    });
                }
                left.spawn((SaveStatus,Text::new(""),super::font(13.0,&font),TextColor(theme::MUTED)));
            });
        root.spawn((Node {width:percent(36),flex_direction:FlexDirection::Column,row_gap:px(20),padding:UiRect::left(px(32)),border:UiRect::left(px(1)),..default()},BorderColor::all(Color::srgb(0.25,0.24,0.21))))
            .with_children(|right| {
                right.spawn((Text::new("EXPLORATION / SURVIE"),super::font(13.0,&font),TextColor(theme::ACCENT)));
                right.spawn((Text::new("MARS\nSURVIE EN SURFACE"),super::font(36.0,&font),TextColor(theme::TEXT)));
                description(right,"Explorez le système solaire.\nRécoltez des ressources sur Mars.\nConstruisez vos abris et surveillez votre oxygène.",&font);
                right.spawn((Node {height:px(1),width:px(64),..default()},BackgroundColor(theme::ACCENT)));
                description(right,"Chaque aventure conserve son monde,\nson inventaire et ses constructions.",&font);
            });
    });
}
fn text_input(
    mut events: MessageReader<KeyboardInput>,
    state: Res<MenuState>,
    mut control: Option<ResMut<SaveControl>>,
) {
    for event in events.read() {
        if state.screen != Screen::NewAdventure || event.state != ButtonState::Pressed {
            continue;
        }
        let Some(c) = control.as_mut() else {
            continue;
        };
        if event.logical_key == Key::Backspace {
            c.draft.pop();
            continue;
        }
        if let Some(text) = &event.text {
            for ch in text.chars().filter(|ch| !ch.is_control()) {
                if c.draft.chars().count() < 40 {
                    c.draft.push(ch);
                }
            }
        }
    }
}
fn action(
    action: MenuAction,
    state: &mut MenuState,
    settings: &mut PlayerSettings,
    c: Option<&mut SaveControl>,
) {
    match action {
        MenuAction::Play => {
            if state.started {
                state.play();
            } else {
                state.screen = Screen::NewAdventure;
            }
        }
        MenuAction::New => {
            state.screen = Screen::NewAdventure;
            if let Some(c) = c {
                c.draft.clear();
                c.status = "Nommez cette nouvelle aventure".into();
            }
        }
        MenuAction::Create => {
            if let Some(c) = c {
                c.request = Some(SaveAction::Create(c.draft.clone()));
            }
        }
        MenuAction::Save => {
            if let Some(c) = c {
                c.request = Some(SaveAction::Save);
            }
        }
        MenuAction::Load => {
            state.screen = Screen::Profiles;
            if let Some(c) = c {
                c.request = Some(SaveAction::Refresh);
            }
        }
        MenuAction::Choose(i) => {
            if let Some(c) = c {
                c.selected = i;
            }
        }
        MenuAction::Previous => {
            if let Some(c) = c {
                c.selected = c.selected.saturating_sub(1);
            }
        }
        MenuAction::Next => {
            if let Some(c) = c {
                c.selected = (c.selected + 1).min(c.profiles.len().saturating_sub(1));
            }
        }
        MenuAction::OpenSelected => {
            if let Some(c) = c
                && let Some(name) = c.profiles.get(c.selected)
            {
                c.request = Some(SaveAction::Open(name.clone()));
            }
        }
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
        MenuAction::Quit => {}
    }
}
#[expect(clippy::too_many_arguments, reason = "Menu, profils et options ECS.")]
fn input(
    keys: Res<ButtonInput<KeyCode>>,
    pilot: Res<Explorer>,
    construction: Option<Res<crate::construction::Construction>>,
    mut persistence: Option<ResMut<SaveControl>>,
    mut state: ResMut<MenuState>,
    mut settings: ResMut<PlayerSettings>,
    mut interactions: Query<
        (&Interaction, &MenuAction, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut exit: MessageWriter<AppExit>,
) {
    if keys.just_pressed(KeyCode::Escape)
        && !construction.is_some_and(|c| c.active || c.catalog_open)
    {
        if state.is_open() {
            state.back();
        } else if !pilot.captured {
            state.open();
        }
        return;
    }
    let shortcut = if keys.just_pressed(KeyCode::Enter) {
        match state.screen {
            Screen::Main => Some(MenuAction::Play),
            Screen::NewAdventure => Some(MenuAction::Create),
            Screen::Profiles => Some(MenuAction::OpenSelected),
            _ => None,
        }
    } else if state.screen == Screen::Main && keys.just_pressed(KeyCode::F5) {
        Some(MenuAction::Save)
    } else if state.screen == Screen::Main && keys.just_pressed(KeyCode::F9) {
        Some(MenuAction::Load)
    } else if state.screen == Screen::Profiles && keys.just_pressed(KeyCode::ArrowUp) {
        Some(MenuAction::Previous)
    } else if state.screen == Screen::Profiles && keys.just_pressed(KeyCode::ArrowDown) {
        Some(MenuAction::Next)
    } else {
        None
    };
    if let Some(a) = shortcut {
        action(a, &mut state, &mut settings, persistence.as_deref_mut());
    }
    for (interaction, a, mut color) in &mut interactions {
        color.0 = match interaction {
            Interaction::Hovered => theme::HOVER,
            Interaction::Pressed => theme::ACTIVE,
            Interaction::None => theme::BUTTON,
        };
        if !state.is_open() || *interaction != Interaction::Pressed {
            continue;
        }
        let expected = match a {
            MenuAction::Sensitivity
            | MenuAction::Fov
            | MenuAction::Msaa
            | MenuAction::Fullscreen
            | MenuAction::Reset => Screen::Options,
            MenuAction::Create => Screen::NewAdventure,
            MenuAction::Choose(_)
            | MenuAction::OpenSelected
            | MenuAction::Previous
            | MenuAction::Next => Screen::Profiles,
            MenuAction::Back => state.screen,
            _ => Screen::Main,
        };
        if expected != state.screen {
            continue;
        }
        if *a == MenuAction::Quit {
            exit.write(AppExit::Success);
        } else {
            action(*a, &mut state, &mut settings, persistence.as_deref_mut());
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
        if let Projection::Perspective(p) = &mut *projection {
            p.fov = settings.fov_degrees().to_radians();
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
    clippy::type_complexity,
    reason = "Affichage des panneaux, profils et saisie, emprunts de textes disjoints."
)]
fn display(
    mut commands: Commands,
    state: Res<MenuState>,
    settings: Res<PlayerSettings>,
    root: Single<(Entity, &mut Node), With<MenuRoot>>,
    mut panels: Query<(&Panel, &mut Node), Without<MenuRoot>>,
    mut labels: Query<(&Label, &mut Text), (Without<SaveStatus>, Without<DraftText>)>,
    camera: Single<Entity, With<GameCamera>>,
    persistence: Option<Res<SaveControl>>,
    mut statuses: Query<&mut Text, (With<SaveStatus>, Without<Label>, Without<DraftText>)>,
    mut drafts: Query<&mut Text, (With<DraftText>, Without<Label>, Without<SaveStatus>)>,
    lists: Query<Entity, With<ProfileList>>,
    mut buttons: Query<
        (&MenuAction, &mut Node, &mut BorderColor),
        (Without<MenuRoot>, Without<Panel>),
    >,
    assets: Res<AssetServer>,
    mut brand: Single<
        (&mut Text, &mut TextFont),
        (
            With<Brand>,
            Without<Label>,
            Without<SaveStatus>,
            Without<DraftText>,
        ),
    >,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let (entity, mut node) = root.into_inner();
    commands.entity(entity).insert(UiTargetCamera(*camera));
    node.display = if state.is_open() {
        Display::Flex
    } else {
        Display::None
    };
    if !state.is_changed()
        && !settings.is_changed()
        && !persistence.as_ref().is_some_and(|p| p.is_changed())
    {
        return;
    }
    **brand.0 = if state.screen == Screen::Main {
        "SOLAR\nSURVIVAL".into()
    } else {
        "SOLAR SURVIVAL".into()
    };
    brand.1.font_size = FontSize::Px(if state.screen == Screen::Main {
        48.0
    } else {
        20.0
    });
    for (panel, mut node) in &mut panels {
        node.display = if panel.0 == state.screen {
            Display::Flex
        } else {
            Display::None
        };
    }
    let c = persistence.as_deref();
    for mut text in &mut statuses {
        **text = c.map_or_else(
            || "".into(),
            |c| {
                format!(
                    "{}{}",
                    c.active_profile
                        .as_ref()
                        .map_or(String::new(), |n| format!("Aventure active : {n}\n")),
                    c.status
                )
            },
        );
    }
    for mut text in &mut drafts {
        **text = c
            .filter(|c| !c.draft.is_empty())
            .map_or("Nom de votre aventure…".into(), |c| {
                format!("{}│", c.draft)
            });
    }
    for (a, mut node, mut border) in &mut buttons {
        node.display = if (*a == MenuAction::Play && !state.started)
            || (*a == MenuAction::Save && c.is_none_or(|c| c.active_profile.is_none()))
        {
            Display::None
        } else {
            Display::Flex
        };
        *border = if matches!(
            a,
            MenuAction::New | MenuAction::Create | MenuAction::OpenSelected
        ) || matches!(a,MenuAction::Choose(i) if c.is_some_and(|c|c.selected==*i))
        {
            BorderColor::all(theme::ACCENT)
        } else {
            BorderColor::all(theme::BUTTON)
        };
    }
    for (label, mut text) in &mut labels {
        **text = match label.0 {
            MenuAction::Play => "Reprendre".into(),
            MenuAction::New => "Nouvelle aventure".into(),
            MenuAction::Create => "Créer l’aventure".into(),
            MenuAction::Choose(i) => c
                .and_then(|c| {
                    c.profiles
                        .get(i)
                        .map(|n| format!("{}  {n}", if c.selected == i { "›" } else { " " }))
                })
                .unwrap_or_default(),
            MenuAction::OpenSelected => "Charger cette aventure".into(),
            MenuAction::Previous => "↑ Précédente".into(),
            MenuAction::Next => "↓ Suivante".into(),
            MenuAction::Options => "Options".into(),
            MenuAction::Credits => "Crédits".into(),
            MenuAction::Save => "Sauvegarder l’aventure · F5".into(),
            MenuAction::Load => "Charger une aventure · F9".into(),
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
    if state.screen == Screen::Profiles {
        let font = assets.load("fonts/DejaVuSans.ttf");
        for entity in &lists {
            commands.entity(entity).despawn_related::<Children>();
            commands.entity(entity).with_children(|parent| {
                if let Some(c)=c && !c.profiles.is_empty() {
                    let per_page=if window.height()<800.0 {3}else{5};
                    let start=c.selected/per_page*per_page;
                    for (i,name) in c.profiles.iter().enumerate().skip(start).take(per_page) {
                        parent.spawn((Button,MenuAction::Choose(i),Node {width:percent(100),padding:UiRect::all(px(13)),..default()},BackgroundColor(if i==c.selected{theme::ACTIVE}else{theme::BUTTON})))
                            .with_child((Label(MenuAction::Choose(i)),Text::new(format!("{}  {name}",if i==c.selected{"›"}else{" "})),super::font(17.0,&font),TextColor(theme::TEXT)));
                    }
                    description(parent,&format!("{} aventure(s) · sélection {}/{}",c.profiles.len(),c.selected+1,c.profiles.len()),&font);
                }else{description(parent,"Aucune aventure sauvegardée.\nCréez votre première aventure depuis le menu.",&font);}
            });
        }
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
                started: true,
            })
            .init_resource::<PlayerSettings>()
            .add_message::<KeyboardInput>()
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
                (text_input, input)
                    .chain()
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
    fn named_adventures_from_menu_text_input_and_profile_browser() {
        let mut app = app();
        app.add_plugins((
            crate::survival::SurvivalPlugin,
            crate::construction::ConstructionPlugin,
            crate::persistence::PersistencePlugin,
        ));
        app.world_mut().resource_mut::<MenuState>().started = false;
        let root = std::env::temp_dir().join(format!("solar-menu-profiles-{}", std::process::id()));
        assert!(!root.exists());
        app.world_mut().resource_mut::<SaveControl>().root = root.clone();
        click(&mut app, MenuAction::New);
        assert_eq!(
            app.world().resource::<MenuState>().screen,
            Screen::NewAdventure
        );
        press(&mut app, KeyCode::Enter);
        assert!(!root.exists());
        let window = app
            .world_mut()
            .query_filtered::<Entity, With<PrimaryWindow>>()
            .single(app.world())
            .unwrap();
        app.world_mut().write_message(KeyboardInput {
            key_code: KeyCode::KeyA,
            logical_key: Key::Character("Alpha".into()),
            state: ButtonState::Pressed,
            text: Some("Alpha".into()),
            repeat: false,
            window,
        });
        app.update();
        assert_eq!(app.world().resource::<SaveControl>().draft, "Alpha");
        press(&mut app, KeyCode::Enter);
        assert!(!app.world().resource::<MenuState>().is_open());
        assert!(root.join("Alpha/session.solar").is_file());
        press(&mut app, KeyCode::KeyM);
        let orientation = app
            .world()
            .resource::<crate::planets::terrain::MarsTerrain>()
            .orientation;
        press(&mut app, KeyCode::Escape);
        click(&mut app, MenuAction::Options);
        click(&mut app, MenuAction::Fov);
        click(&mut app, MenuAction::Back);
        press(&mut app, KeyCode::F5);
        click(&mut app, MenuAction::New);
        app.world_mut().resource_mut::<SaveControl>().draft = "Bravo".into();
        click(&mut app, MenuAction::Create);
        assert_eq!(app.world().resource::<Explorer>().mode, MovementMode::Space);
        assert!(
            app.world()
                .resource::<crate::buildings::Buildings>()
                .shelters
                .is_empty()
        );
        press(&mut app, KeyCode::Escape);
        press(&mut app, KeyCode::F9);
        assert_eq!(app.world().resource::<MenuState>().screen, Screen::Profiles);
        assert_eq!(
            app.world().resource::<SaveControl>().profiles,
            vec!["Alpha", "Bravo"]
        );
        click(&mut app, MenuAction::Choose(0));
        press(&mut app, KeyCode::Enter);
        assert_eq!(
            app.world()
                .resource::<SaveControl>()
                .active_profile
                .as_deref(),
            Some("Alpha")
        );
        assert_eq!(
            app.world().resource::<Explorer>().mode,
            MovementMode::Surface
        );
        assert_eq!(
            app.world()
                .resource::<crate::planets::terrain::MarsTerrain>()
                .orientation,
            orientation
        );
        assert_eq!(app.world().resource::<PlayerSettings>().fov_degrees(), 75.0);
        std::fs::remove_dir_all(root).unwrap();
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
        press(&mut app, KeyCode::Enter);
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
        press(&mut app, KeyCode::Enter);
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
