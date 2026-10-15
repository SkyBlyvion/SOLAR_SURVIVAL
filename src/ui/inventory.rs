//! Inventaire porté et coffre de proximité : sélection de piles, quantité et transactions.
use super::theme;
use crate::{
    GameSet,
    base::{Base, BuildKind, chest_access},
    buildings::Buildings,
    construction::Construction,
    inventory::{Amount, transfer},
    planets::terrain::MarsTerrain,
    player::{Explorer, MovementMode, surface::SurfaceState},
    survival::Oxygen,
    ui::menu::{MenuInputSet, MenuState},
};
use bevy::prelude::*;
#[derive(Resource, Default)]
pub struct InventoryScreen {
    pub open: bool,
    pub chest: Option<crate::buildings::BuildingId>,
    pub selected: usize,
    pub storage_side: bool,
    pub quantity: u32,
    pub swap_from: Option<(bool, usize)>,
    pub message: String,
}

#[derive(Component)]
struct Root;
#[derive(Component)]
struct Status;
#[derive(Component)]
struct StockText(bool);
#[derive(Component, Clone, Copy)]
struct Slot {
    storage: bool,
    index: usize,
}
#[derive(Component)]
struct SlotText(Slot);
#[derive(Component, Clone, Copy)]
enum Action {
    Close,
    Sort,
    Swap,
    Quantity,
    Exact,
    Stack,
    Maximum,
}
#[derive(Component)]
struct ActionText(Action);
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub(crate) struct InventoryInputSet;
pub(super) struct InventoryHudPlugin;
impl Plugin for InventoryHudPlugin {
    fn build(&self, app: &mut App) {
        if !app.world().resource::<Oxygen>().enabled {
            return;
        }
        app.add_systems(Startup, setup)
            .add_systems(
                Update,
                input
                    .after(MenuInputSet)
                    .before(crate::camera::controls)
                    .in_set(InventoryInputSet)
                    .in_set(GameSet::Input),
            )
            .add_systems(Update, refresh.in_set(GameSet::Interface));
    }
}
fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts/DejaVuSans.ttf");
    commands
        .spawn((
            Root,
            GlobalZIndex(35),
            Node {
                position_type: PositionType::Absolute,
                left: px(24),
                right: px(24),
                top: px(136),
                max_width: px(1120),
                padding: UiRect::all(px(18)),
                flex_direction: FlexDirection::Column,
                row_gap: px(10),
                display: Display::None,
                ..default()
            },
            BackgroundColor(theme::PANEL),
        ))
        .with_children(|root| {
            root.spawn((
                Text::new("INVENTAIRE / BASE   ·   O₂ actif · ↑↓ : quantité 1–20"),
                super::font(14.0, &font),
                TextColor(theme::ACCENT),
            ));
            root.spawn((
                Status,
                Text::new(""),
                super::font(13.0, &font),
                TextColor(theme::TEXT),
            ));
            root.spawn(Node {
                width: percent(100),
                column_gap: px(16),
                ..default()
            })
            .with_children(|columns| {
                for storage in [false, true] {
                    columns
                        .spawn(Node {
                            width: percent(49),
                            flex_direction: FlexDirection::Column,
                            row_gap: px(6),
                            ..default()
                        })
                        .with_children(|column| {
                            column.spawn((
                                StockText(storage),
                                Text::new(""),
                                super::font(15.0, &font),
                                TextColor(theme::TEXT),
                            ));
                            column
                                .spawn(Node {
                                    width: percent(100),
                                    flex_wrap: FlexWrap::Wrap,
                                    column_gap: px(4),
                                    row_gap: px(4),
                                    ..default()
                                })
                                .with_children(|slots| {
                                    for index in 0..if storage { 16 } else { 8 } {
                                        let slot = Slot { storage, index };
                                        slots
                                            .spawn((
                                                Button,
                                                slot,
                                                Node {
                                                    width: percent(48),
                                                    height: px(34),
                                                    padding: UiRect::all(px(6)),
                                                    ..default()
                                                },
                                                BackgroundColor(theme::BUTTON),
                                            ))
                                            .with_child((
                                                SlotText(slot),
                                                Text::new(""),
                                                super::font(12.0, &font),
                                                TextColor(theme::TEXT),
                                            ));
                                    }
                                });
                        });
                }
            });
            root.spawn(Node {
                width: percent(100),
                flex_wrap: FlexWrap::Wrap,
                column_gap: px(7),
                row_gap: px(7),
                ..default()
            })
            .with_children(|row| {
                for action in [
                    Action::Quantity,
                    Action::Exact,
                    Action::Stack,
                    Action::Maximum,
                    Action::Sort,
                    Action::Swap,
                    Action::Close,
                ] {
                    row.spawn((
                        Button,
                        action,
                        Node {
                            padding: UiRect::all(px(10)),
                            ..default()
                        },
                        BackgroundColor(theme::BUTTON),
                    ))
                    .with_child((
                        ActionText(action),
                        Text::new(""),
                        super::font(12.0, &font),
                        TextColor(theme::TEXT),
                    ));
                }
            });
        });
}
#[expect(
    clippy::too_many_arguments,
    clippy::type_complexity,
    reason = "Accès de proximité et transactions d'inventaire ECS."
)]
fn input(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    menu: Res<MenuState>,
    pilot: Res<Explorer>,
    state: Res<SurfaceState>,
    terrain: Res<MarsTerrain>,
    oxygen: Res<Oxygen>,
    mut screen: ResMut<InventoryScreen>,
    mut c: ResMut<Construction>,
    mut buildings: ResMut<Buildings>,
    mut slots: Query<
        (&Interaction, &Slot, &mut BackgroundColor),
        (Changed<Interaction>, Without<Action>),
    >,
    mut actions: Query<
        (&Interaction, &Action, &mut BackgroundColor),
        (Changed<Interaction>, Without<Slot>),
    >,
) {
    if menu.is_open() || pilot.mode != MovementMode::Surface || oxygen.failed {
        screen.open = false;
        screen.chest = None;
        screen.swap_from = None;
        return;
    }
    let feet = terrain.body_relative(state.feet);
    let eye = terrain.body_relative(state.eye(&terrain));
    if screen
        .chest
        .is_some_and(|id| !chest_access(&buildings, id, feet, eye, &terrain))
    {
        screen.open = false;
        screen.chest = None;
        screen.swap_from = None;
        screen.message = "Coffre inaccessible : interface fermée".into();
    }
    let nearest = buildings
        .facilities
        .iter()
        .filter(|f| {
            f.kind == BuildKind::Chest && chest_access(&buildings, f.id, feet, eye, &terrain)
        })
        .min_by(|a, b| {
            a.position_m
                .distance_squared(feet)
                .total_cmp(&b.position_m.distance_squared(feet))
        })
        .map(|f| f.id);
    let open_key = keys.just_pressed(KeyCode::KeyI)
        || keys.just_pressed(KeyCode::KeyE)
            && !screen.open
            && nearest.is_some()
            && !c.active
            && !c.catalog_open;
    if open_key {
        screen.open = !screen.open;
        screen.chest = nearest;
        screen.selected = 0;
        screen.storage_side = false;
        screen.quantity = 1;
        screen.swap_from = None;
        if screen.open {
            c.cancel();
            screen.message =
                "Sélectionnez une pile ; quantité exacte, pile ou maximum admissible.".into();
        }
    } else if screen.open && keys.just_pressed(KeyCode::Escape) {
        screen.open = false;
        screen.swap_from = None;
    }
    if !screen.open {
        return;
    }
    if keys.just_pressed(KeyCode::ArrowUp) {
        screen.quantity = (screen.quantity + 1).min(20);
    }
    if keys.just_pressed(KeyCode::ArrowDown) {
        screen.quantity = screen.quantity.saturating_sub(1).max(1);
    }
    for (interaction, slot, mut color) in &mut slots {
        color.0 = if *interaction == Interaction::None {
            theme::BUTTON
        } else {
            theme::HOVER
        };
        if *interaction == Interaction::Pressed && (!slot.storage || screen.chest.is_some()) {
            if let Some((storage, index)) = screen.swap_from.take() {
                if storage != slot.storage {
                    screen.message =
                        "Ranger sur le même côté ; utilisez Transférer entre inventaires".into();
                } else {
                    let inventory = if storage {
                        &mut buildings
                            .facilities
                            .iter_mut()
                            .find(|f| Some(f.id) == screen.chest)
                            .unwrap()
                            .inventory
                    } else {
                        &mut c.inventory
                    };
                    screen.message = match inventory.swap(index, slot.index) {
                        Ok(()) => "Emplacements échangés".into(),
                        Err(e) => e.into(),
                    };
                }
            }
            screen.selected = slot.index;
            screen.storage_side = slot.storage;
        }
    }
    for (interaction, action, mut color) in &mut actions {
        color.0 = if *interaction == Interaction::None {
            theme::BUTTON
        } else {
            theme::HOVER
        };
        if *interaction != Interaction::Pressed {
            continue;
        }
        match action {
            Action::Close => {
                screen.open = false;
                screen.swap_from = None;
            }
            Action::Quantity => {
                screen.quantity = match screen.quantity {
                    1 => 5,
                    5 => 20,
                    _ => 1,
                }
            }
            Action::Swap => {
                screen.swap_from = Some((screen.storage_side, screen.selected));
                screen.message = "Cliquez l’emplacement de destination sur le même côté".into();
            }
            Action::Sort => {
                if screen.storage_side {
                    if let Some(f) = buildings
                        .facilities
                        .iter_mut()
                        .find(|f| Some(f.id) == screen.chest)
                    {
                        f.inventory.organize();
                    }
                } else {
                    c.inventory.organize();
                }
                screen.swap_from = None;
                screen.message = "Piles regroupées par type".into();
            }
            Action::Exact | Action::Stack | Action::Maximum => {
                screen.swap_from = None;
                let Some(id) = screen
                    .chest
                    .filter(|id| chest_access(&buildings, *id, feet, eye, &terrain))
                else {
                    commands.trigger(crate::audio::SoundCue::Refused);
                    screen.message = "Aucun coffre accessible (2,5 m)".into();
                    continue;
                };
                let chest = &mut buildings
                    .facilities
                    .iter_mut()
                    .find(|f| f.id == id)
                    .unwrap()
                    .inventory;
                let amount = match action {
                    Action::Exact => Amount::Exact(screen.quantity),
                    Action::Stack => Amount::Stack,
                    _ => Amount::Maximum,
                };
                let result = if screen.storage_side {
                    transfer(chest, &mut c.inventory, screen.selected, amount)
                } else {
                    transfer(&mut c.inventory, chest, screen.selected, amount)
                };
                commands.trigger(if result.is_ok() {
                    crate::audio::SoundCue::Harvest
                } else {
                    crate::audio::SoundCue::Refused
                });
                screen.message = match result {
                    Ok(n) => format!("{n} objet(s) transféré(s)"),
                    Err(e) => e.into(),
                };
            }
        }
    }
}
#[expect(
    clippy::too_many_arguments,
    clippy::type_complexity,
    reason = "Rafraîchissement des deux inventaires et réserves."
)]
fn refresh(
    screen: Res<InventoryScreen>,
    c: Res<Construction>,
    buildings: Res<Buildings>,
    base: Res<Base>,
    mut root: Single<(Entity, &mut Node), With<Root>>,
    mut status: Single<
        &mut Text,
        (
            With<Status>,
            Without<StockText>,
            Without<SlotText>,
            Without<ActionText>,
        ),
    >,
    mut stocks: Query<
        (&StockText, &mut Text),
        (Without<Status>, Without<SlotText>, Without<ActionText>),
    >,
    mut slots: Query<
        (&SlotText, &mut Text),
        (Without<Status>, Without<StockText>, Without<ActionText>),
    >,
    mut actions: Query<
        (&ActionText, &mut Text),
        (Without<Status>, Without<StockText>, Without<SlotText>),
    >,
    camera: Single<Entity, With<crate::camera::GameCamera>>,
    mut commands: Commands,
) {
    root.1.display = if screen.open {
        Display::Flex
    } else {
        Display::None
    };
    if !screen.open {
        return;
    }
    commands.entity(root.0).insert(UiTargetCamera(*camera));
    let chest = buildings
        .facilities
        .iter()
        .find(|f| Some(f.id) == screen.chest);
    let generator = if !buildings
        .facilities
        .iter()
        .any(|f| f.kind == BuildKind::Generator)
    {
        "non installé"
    } else if base.oxygen_s >= Base::oxygen_capacity(&buildings) {
        "arrêt · réserve pleine"
    } else if base.load_w > 0.0 {
        "production O₂"
    } else {
        "arrêt · énergie insuffisante"
    };
    ***status = format!(
        "Solaire {:.1} W · Consommation {:.1} W · Batterie {:.1}/{:.0} Wh · O₂ {:.0}/{:.0} s\nGénérateur : {generator}\n{}",
        base.solar_w,
        base.load_w,
        base.energy_j / 3600.0,
        Base::energy_capacity(&buildings) / 3600.0,
        base.oxygen_s,
        Base::oxygen_capacity(&buildings),
        screen.message
    )
    .replace("\\n", "\n");
    for (label, mut text) in &mut stocks {
        **text = if label.0 {
            chest.map_or("COFFRE · approchez à 2,5 m pour y accéder".into(), |f| {
                format!("COFFRE {} · {}/16 piles", f.id.0, f.inventory.used())
            })
        } else {
            format!("PORTÉ · {}/8 piles · 20 objets/pile", c.inventory.used())
        };
    }
    for (label, mut text) in &mut slots {
        let slot = label.0;
        let inventory = if slot.storage {
            chest.map(|f| &f.inventory)
        } else {
            Some(&c.inventory)
        };
        let content = inventory
            .and_then(|i| i.slots.get(slot.index))
            .and_then(Option::as_ref);
        **text = format!(
            "{}{:02}  {}",
            if screen.storage_side == slot.storage && screen.selected == slot.index {
                "▶ "
            } else {
                ""
            },
            slot.index + 1,
            content.map_or("—".into(), |s| format!(
                "{} ×{}",
                s.item.name(),
                s.quantity
            ))
        );
    }
    for (label, mut text) in &mut actions {
        **text = match label.0 {
            Action::Close => "Fermer · I / Échap".into(),
            Action::Sort => "Regrouper".into(),
            Action::Swap => "Déplacer la pile".into(),
            Action::Quantity => format!("Quantité : {} (1/5/20)", screen.quantity),
            Action::Exact => "Transférer quantité".into(),
            Action::Stack => "Transférer pile".into(),
            Action::Maximum => "Maximum admissible".into(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{inventory::Item, universe::catalog::SolarSystem};
    use bevy::math::{DQuat, DVec3};
    #[test]
    fn actual_ui_transactions_revalidate_capacity_and_close_when_chest_becomes_distant() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<ButtonInput<KeyCode>>()
            .init_resource::<SolarSystem>()
            .init_resource::<MarsTerrain>()
            .init_resource::<Explorer>()
            .init_resource::<SurfaceState>()
            .init_resource::<Oxygen>()
            .init_resource::<Construction>()
            .init_resource::<Buildings>()
            .init_resource::<InventoryScreen>()
            .init_resource::<MenuState>()
            .add_systems(Update, input);
        app.world_mut().resource_mut::<MenuState>().play();
        app.world_mut().resource_mut::<Explorer>().mode = MovementMode::Surface;
        app.world_mut()
            .resource_mut::<MarsTerrain>()
            .set_site(DVec3::Y);
        let t = app.world().resource::<MarsTerrain>();
        let p = t.body_relative(DVec3::new(0.0, t.height(0.0, 0.0), 0.0));
        let id = app
            .world_mut()
            .resource_mut::<Buildings>()
            .install_facility(BuildKind::Chest, p, DQuat::IDENTITY);
        let t = app.world().resource::<MarsTerrain>();
        let local = t.chart(p + DVec3::Z * 1.7);
        app.world_mut().resource_mut::<SurfaceState>().feet = local;
        app.world_mut().resource_scope(|w, b: Mut<Buildings>| {
            b.publish_colliders(&mut w.resource_mut::<MarsTerrain>())
        });
        app.world_mut()
            .resource_mut::<Construction>()
            .inventory
            .add(Item::Alloy, 7)
            .unwrap();
        app.world_mut().resource_mut::<Buildings>().facilities[0].inventory =
            crate::inventory::Inventory::new(crate::inventory::CHEST_SLOTS);
        {
            let mut b = app.world_mut().resource_mut::<Buildings>();
            let inventory = &mut b.facilities[0].inventory;
            inventory.add(Item::Circuit, 300).unwrap();
            inventory.add(Item::Alloy, 18).unwrap();
        }
        {
            let mut s = app.world_mut().resource_mut::<InventoryScreen>();
            s.open = true;
            s.chest = Some(id);
            s.quantity = 5;
        }
        let exact = app
            .world_mut()
            .spawn((
                Interaction::Pressed,
                Action::Exact,
                BackgroundColor::default(),
            ))
            .id();
        app.update();
        assert_eq!(
            app.world()
                .resource::<Construction>()
                .inventory
                .count(Item::Alloy),
            7
        );
        app.world_mut()
            .entity_mut(exact)
            .insert((Interaction::None, Action::Maximum));
        app.update();
        app.world_mut()
            .entity_mut(exact)
            .insert(Interaction::Pressed);
        app.update();
        assert_eq!(
            app.world()
                .resource::<Construction>()
                .inventory
                .count(Item::Alloy),
            5
        );
        for _ in 0..3 {
            app.world_mut()
                .entity_mut(exact)
                .insert(Interaction::Pressed);
            app.update();
        }
        assert_eq!(
            app.world()
                .resource::<Construction>()
                .inventory
                .count(Item::Alloy),
            5
        );
        assert_eq!(
            app.world().resource::<Buildings>().facilities[0]
                .inventory
                .count(Item::Alloy),
            20
        );
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyE);
        app.update();
        assert!(app.world().resource::<InventoryScreen>().open);
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .reset_all();
        app.world_mut().resource_mut::<SurfaceState>().feet.x += 10.0;
        app.update();
        assert!(!app.world().resource::<InventoryScreen>().open);
        assert!(app.world().resource::<InventoryScreen>().chest.is_none());
        assert_eq!(
            app.world()
                .resource::<Construction>()
                .inventory
                .count(Item::Alloy),
            5
        );
    }
}
