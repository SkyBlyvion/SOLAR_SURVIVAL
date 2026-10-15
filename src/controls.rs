//! Commandes physiques configurables, traduites une fois vers les commandes de jeu.
//! Les menus gardent leurs touches de secours et lisent toujours le clavier brut.
use crate::{
    GameSet,
    ui::menu::{MenuInputSet, MenuState, PlayerSettings},
};
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

pub const ACTIONS: &[(KeyCode, &str)] = &[
    (KeyCode::KeyW, "Avancer"),
    (KeyCode::KeyS, "Reculer"),
    (KeyCode::KeyA, "Aller à gauche"),
    (KeyCode::KeyD, "Aller à droite"),
    (KeyCode::Space, "Sauter / monter"),
    (KeyCode::ControlLeft, "Descendre en vol"),
    (KeyCode::ShiftLeft, "Courir / accélérer"),
    (KeyCode::KeyE, "Récolter / coffre"),
    (KeyCode::KeyF, "Relevé / rapport / centrer"),
    (KeyCode::KeyI, "Inventaire"),
    (KeyCode::KeyB, "Construction"),
    (KeyCode::KeyX, "Annuler le placement"),
    (KeyCode::KeyJ, "Orienter à gauche"),
    (KeyCode::KeyL, "Orienter à droite"),
    (KeyCode::KeyM, "Surface Mars / retour"),
    (KeyCode::KeyG, "Visiter la cible"),
    (KeyCode::KeyO, "Vue du système"),
    (KeyCode::KeyU, "Repère solaire"),
    (KeyCode::KeyR, "Repartir après échec"),
    (KeyCode::KeyT, "Accélérer les orbites"),
    (KeyCode::KeyP, "Pause des orbites"),
    (KeyCode::KeyH, "Télémétrie"),
    (KeyCode::KeyN, "Navigation"),
    (KeyCode::Tab, "Cible suivante"),
    (KeyCode::PageUp, "Vitesse supérieure"),
    (KeyCode::PageDown, "Vitesse inférieure"),
];
pub const PAGE_SIZE: usize = 6;
/// Identifiants explicites et stables du format disque ; jamais de cast de l'enum Bevy.
pub const KEYS: &[KeyCode] = &[
    KeyCode::KeyA,
    KeyCode::KeyB,
    KeyCode::KeyC,
    KeyCode::KeyD,
    KeyCode::KeyE,
    KeyCode::KeyF,
    KeyCode::KeyG,
    KeyCode::KeyH,
    KeyCode::KeyI,
    KeyCode::KeyJ,
    KeyCode::KeyK,
    KeyCode::KeyL,
    KeyCode::KeyM,
    KeyCode::KeyN,
    KeyCode::KeyO,
    KeyCode::KeyP,
    KeyCode::KeyQ,
    KeyCode::KeyR,
    KeyCode::KeyS,
    KeyCode::KeyT,
    KeyCode::KeyU,
    KeyCode::KeyV,
    KeyCode::KeyW,
    KeyCode::KeyX,
    KeyCode::KeyY,
    KeyCode::KeyZ,
    KeyCode::Space,
    KeyCode::ControlLeft,
    KeyCode::ShiftLeft,
    KeyCode::Tab,
    KeyCode::PageUp,
    KeyCode::PageDown,
    KeyCode::AltLeft,
    KeyCode::F1,
    KeyCode::F2,
    KeyCode::F3,
    KeyCode::F4,
    KeyCode::F6,
    KeyCode::F7,
    KeyCode::F8,
    KeyCode::F10,
    KeyCode::F11,
];
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bindings(pub(crate) Vec<KeyCode>);
impl Default for Bindings {
    fn default() -> Self {
        Self(ACTIONS.iter().map(|a| a.0).collect())
    }
}
impl Bindings {
    pub fn valid(&self) -> bool {
        self.0.len() == ACTIONS.len()
            && self.0.iter().all(|k| KEYS.contains(k))
            && self.0.iter().collect::<HashSet<_>>().len() == ACTIONS.len()
    }
    pub fn key(&self, action: usize) -> KeyCode {
        self.0[action]
    }
    pub fn assign(&mut self, action: usize, key: KeyCode) -> Result<(), String> {
        if action >= ACTIONS.len() {
            return Err("Commande inconnue".into());
        }
        // Les deux modificateurs physiques correspondent à la même commande.
        let key = normalize(key);
        if !KEYS.contains(&key) {
            return Err("Touche réservée ou non prise en charge".into());
        }
        if let Some(i) = self.0.iter().position(|k| *k == key)
            && i != action
        {
            return Err(format!("Déjà utilisée : {}", ACTIONS[i].1));
        }
        self.0[action] = key;
        Ok(())
    }
    pub fn label(&self, canonical: KeyCode) -> String {
        let key = ACTIONS
            .iter()
            .position(|a| a.0 == canonical)
            .map_or(canonical, |i| self.0[i]);
        key_label(key)
    }
    pub fn hints(&self, text: &str) -> String {
        let mut out = text.to_owned();
        for (canonical, _) in ACTIONS {
            out = out.replace(&format!("<key:{canonical:?}>"), &self.label(*canonical));
        }
        out
    }
    pub(crate) fn ids(&self) -> impl Iterator<Item = usize> + '_ {
        self.0
            .iter()
            .map(|k| KEYS.iter().position(|v| v == k).expect("validated binding"))
    }
    pub(crate) fn from_ids(ids: &[usize]) -> Result<Self, String> {
        let keys = ids
            .iter()
            .map(|i| KEYS.get(*i).copied().ok_or("Touche inconnue".to_owned()))
            .collect::<Result<Vec<_>, _>>()?;
        let result = Self(keys);
        if !result.valid() {
            return Err("Commandes dupliquées ou incomplètes".into());
        }
        Ok(result)
    }
}
fn normalize(key: KeyCode) -> KeyCode {
    match key {
        KeyCode::ShiftRight => KeyCode::ShiftLeft,
        KeyCode::ControlRight => KeyCode::ControlLeft,
        KeyCode::AltRight => KeyCode::AltLeft,
        other => other,
    }
}
pub fn key_label(key: KeyCode) -> String {
    match key {
        KeyCode::Space => "Espace".into(),
        KeyCode::ControlLeft => "Ctrl".into(),
        KeyCode::ShiftLeft => "Maj".into(),
        KeyCode::AltLeft => "Alt".into(),
        KeyCode::PageUp => "Pg↑".into(),
        KeyCode::PageDown => "Pg↓".into(),
        // Deux noms pour les positions qui diffèrent entre QWERTY et AZERTY.
        KeyCode::KeyW => "W / Z".into(),
        KeyCode::KeyA => "A / Q".into(),
        KeyCode::KeyQ => "Q / A".into(),
        KeyCode::KeyZ => "Z / W".into(),
        KeyCode::KeyM => "M / ,".into(),
        other => format!("{other:?}").trim_start_matches("Key").to_owned(),
    }
}
#[derive(Resource, Default, Deref, DerefMut)]
pub struct GameplayKeys(ButtonInput<KeyCode>);
#[derive(Resource, Default)]
pub(crate) struct HeldInMenu(HashSet<KeyCode>);
pub(crate) fn install(app: &mut App) {
    app.init_resource::<GameplayKeys>()
        .init_resource::<HeldInMenu>()
        .add_systems(
            Update,
            translate
                .after(MenuInputSet)
                .before(crate::ui::inventory::InventoryInputSet)
                .before(crate::camera::controls)
                .in_set(GameSet::Input),
        )
        .add_systems(Update, resolve_hints.after(GameSet::Interface));
}
pub(crate) fn translate(
    raw: Res<ButtonInput<KeyCode>>,
    settings: Option<Res<PlayerSettings>>,
    menu: Option<Res<MenuState>>,
    mut held: ResMut<HeldInMenu>,
    mut game: ResMut<GameplayKeys>,
) {
    held.0.retain(|k| raw.pressed(*k) && !raw.just_pressed(*k));
    if menu.is_some_and(|m| m.is_open()) {
        held.0.extend(raw.get_pressed().copied());
        game.reset_all();
        return;
    }
    let defaults = Bindings::default();
    let bindings = settings.as_ref().map_or(&defaults, |s| &s.bindings);
    let mut active = HashSet::new();
    let mut fresh = HashSet::new();
    for raw_key in raw.get_pressed().filter(|k| !held.0.contains(k)) {
        let key = normalize(*raw_key);
        let canonical = if let Some(i) = bindings.0.iter().position(|k| *k == key) {
            ACTIONS[i].0
        } else if !KEYS.contains(&key) {
            key
        } else {
            continue;
        };
        active.insert(canonical);
        if raw.just_pressed(*raw_key) {
            fresh.insert(canonical);
        }
    }
    game.clear();
    let previous: Vec<_> = game.get_pressed().copied().collect();
    for key in previous {
        if !active.contains(&key) {
            game.release(key);
        }
    }
    for key in active {
        if fresh.contains(&key) {
            game.reset(key);
        }
        game.press(key);
    }
}
/// Retient uniquement les modèles contenant une aide de touche ; les textes de jeu
/// réécrits sont résolus après les HUD, les textes statiques suivent aussi les options.
fn resolve_hints(
    settings: Option<Res<PlayerSettings>>,
    mut texts: Query<(Entity, &mut Text)>,
    mut templates: Local<HashMap<Entity, (String, String)>>,
) {
    let Some(settings) = settings else {
        return;
    };
    templates.retain(|e, _| texts.contains(*e));
    for (entity, mut text) in &mut texts {
        if text.contains("<key:") {
            let source = text.0.clone();
            let resolved = settings.bindings.hints(&source);
            templates.insert(entity, (source, resolved.clone()));
            if text.0 != resolved {
                text.0 = resolved;
            }
        } else if let Some((source, last)) = templates.get_mut(&entity) {
            if text.0 != *last {
                templates.remove(&entity);
                continue;
            }
            if settings.is_changed() {
                let resolved = settings.bindings.hints(source);
                if text.0 != resolved {
                    text.0 = resolved.clone();
                }
                *last = resolved;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn conflicts_reserved_keys_modifier_aliases_and_disk_ids_are_transactional() {
        let mut b = Bindings::default();
        let before = b.clone();
        assert!(b.assign(0, KeyCode::KeyS).unwrap_err().contains("Reculer"));
        for key in [
            KeyCode::Escape,
            KeyCode::Enter,
            KeyCode::F5,
            KeyCode::F9,
            KeyCode::F12,
            KeyCode::ArrowUp,
            KeyCode::Digit1,
            KeyCode::Equal,
        ] {
            assert!(b.assign(0, key).is_err());
        }
        assert_eq!(b, before);
        b.assign(0, KeyCode::KeyC).unwrap();
        assert_eq!(Bindings::from_ids(&b.ids().collect::<Vec<_>>()).unwrap(), b);
        let mut ids: Vec<_> = b.ids().collect();
        ids[0] = ids[1];
        assert!(Bindings::from_ids(&ids).is_err());
        ids[0] = usize::MAX;
        assert!(Bindings::from_ids(&ids).is_err());
        assert!(Bindings::from_ids(&[]).is_err());
        b.assign(6, KeyCode::ShiftRight).unwrap();
        assert_eq!(b.key(6), KeyCode::ShiftLeft);
        assert_eq!(b.hints("<key:KeyW> : avancer"), "C : avancer");
    }
    #[test]
    fn held_repeat_release_and_menu_barrier_translate_without_old_shortcut() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<GameplayKeys>()
            .init_resource::<HeldInMenu>()
            .init_resource::<PlayerSettings>()
            .add_systems(Update, translate);
        app.world_mut()
            .resource_mut::<PlayerSettings>()
            .bindings
            .assign(0, KeyCode::KeyC)
            .unwrap();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyW);
        app.update();
        assert!(
            !app.world()
                .resource::<GameplayKeys>()
                .pressed(KeyCode::KeyW)
        );
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyC);
        app.update();
        assert!(
            app.world()
                .resource::<GameplayKeys>()
                .just_pressed(KeyCode::KeyW)
        );
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .clear();
        app.update();
        assert!(
            app.world()
                .resource::<GameplayKeys>()
                .pressed(KeyCode::KeyW)
        );
        assert!(
            !app.world()
                .resource::<GameplayKeys>()
                .just_pressed(KeyCode::KeyW)
        );
        app.init_resource::<MenuState>();
        app.update();
        assert!(
            !app.world()
                .resource::<GameplayKeys>()
                .pressed(KeyCode::KeyW)
        );
        app.world_mut().resource_mut::<MenuState>().play();
        app.update();
        assert!(
            !app.world()
                .resource::<GameplayKeys>()
                .pressed(KeyCode::KeyW)
        );
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .release(KeyCode::KeyC);
        app.update();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyC);
        app.update();
        assert!(
            app.world()
                .resource::<GameplayKeys>()
                .just_pressed(KeyCode::KeyW)
        );
    }
    #[test]
    fn static_and_rewritten_hints_follow_bindings_and_remove_dead_entities() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<PlayerSettings>()
            .add_systems(Update, resolve_hints);
        let entity = app
            .world_mut()
            .spawn(Text::new("<key:KeyF> : relever"))
            .id();
        app.update();
        assert_eq!(app.world().get::<Text>(entity).unwrap().0, "F : relever");
        app.world_mut()
            .resource_mut::<PlayerSettings>()
            .bindings
            .assign(8, KeyCode::KeyV)
            .unwrap();
        app.update();
        assert_eq!(app.world().get::<Text>(entity).unwrap().0, "V : relever");
        app.world_mut().get_mut::<Text>(entity).unwrap().0 = "<key:KeyF> : transmettre".into();
        app.update();
        assert_eq!(
            app.world().get::<Text>(entity).unwrap().0,
            "V : transmettre"
        );
        app.world_mut().get_mut::<Text>(entity).unwrap().0 = "Mission terminée".into();
        app.update();
        app.world_mut().resource_mut::<PlayerSettings>().bindings = default();
        app.update();
        assert_eq!(
            app.world().get::<Text>(entity).unwrap().0,
            "Mission terminée"
        );
        app.world_mut().despawn(entity);
        app.update();
    }
}
