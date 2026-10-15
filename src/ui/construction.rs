//! Inventaire et commandes P5 visibles même lorsque H masque la télémétrie.
use super::theme;
use crate::{
    GameSet,
    base::BuildKind,
    construction::Construction,
    planets::terrain::MarsTerrain,
    player::{Explorer, MovementMode, surface::SurfaceState},
    survival::Oxygen,
    ui::menu::MenuState,
};
use bevy::prelude::*;
#[derive(Component)]
struct CatalogRoot;
#[derive(Component)]
struct RecipeText;
#[derive(Component)]
struct BuildAction(bool);
#[derive(Component)]
struct SelectRecipe(BuildKind);
#[derive(Component)]
struct Root;
#[derive(Component)]
struct Label;
pub(super) struct ConstructionHudPlugin;
impl Plugin for ConstructionHudPlugin {
    fn build(&self, app: &mut App) {
        if !app.world().resource::<Oxygen>().enabled {
            return;
        }
        app.add_systems(Startup, setup)
            .add_systems(
                Update,
                interact
                    .after(crate::ui::menu::MenuInputSet)
                    .after(super::inventory::InventoryInputSet)
                    .before(crate::camera::controls)
                    .in_set(GameSet::Input),
            )
            .add_systems(Update, refresh.in_set(GameSet::Interface));
    }
}
fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts/DejaVuSans.ttf");
    commands.spawn((CatalogRoot,GlobalZIndex(30),Node {position_type:PositionType::Absolute,left:px(24),top:px(136),width:percent(44),max_width:px(440),padding:UiRect::all(px(24)),flex_direction:FlexDirection::Column,row_gap:px(9),display:Display::None,border:UiRect::left(px(2)),..default()},BackgroundColor(theme::PANEL),BorderColor::all(theme::ACCENT)))
        .with_children(|root| {
            root.spawn((Text::new("CONSTRUCTION / BASE"),super::font(12.0,&font),TextColor(theme::ACCENT)));
            for kind in BuildKind::ALL {
                root.spawn((Button,SelectRecipe(kind),Node {width:percent(100),padding:UiRect::all(px(7)),..default()},BackgroundColor(theme::BUTTON)))
                    .with_child((Text::new(format!("{} · {}",kind.name(),kind.recipe_label())),super::font(13.0,&font),TextColor(theme::TEXT)));
            }

            root.spawn((RecipeText,Text::new(""),super::font(16.0,&font),TextColor(theme::TEXT)));
            for (begin,title) in [(true,"Prévisualiser · Entrée"),(false,"Fermer · <key:KeyB> / Échap")] {
                root.spawn((Button,BuildAction(begin),Node {width:percent(100),padding:UiRect::all(px(14)),..default()},BackgroundColor(theme::BUTTON)))
                    .with_child((Text::new(title),super::font(16.0,&font),TextColor(theme::TEXT)));
            }
            root.spawn((Text::new("La prévisualisation est gratuite.\nLes ressources sont dépensées à la confirmation.\nL’oxygène continue de se consommer."),super::font(12.0,&font),TextColor(theme::MUTED)));
        });

    commands
        .spawn((
            Root,
            Node {
                position_type: PositionType::Absolute,
                bottom: px(92),
                left: px(24),
                width: percent(45),
                max_width: px(520),
                padding: UiRect::all(px(14)),
                border_radius: BorderRadius::ZERO,
                display: Display::None,
                ..default()
            },
            BackgroundColor(theme::PANEL),
        ))
        .with_child((
            Label,
            Text::new("RÉCUPÉRATION"),
            TextFont {
                font: assets.load("fonts/DejaVuSans.ttf").into(),
                font_size: FontSize::Px(14.0),
                ..default()
            },
            TextColor(theme::TEXT),
        ));
}
#[expect(
    clippy::too_many_arguments,
    clippy::type_complexity,
    reason = "HUD d'inventaire et d'admission ECS."
)]
fn refresh(
    c: Res<Construction>,
    pilot: Res<Explorer>,
    oxygen: Res<Oxygen>,
    menu: Res<MenuState>,
    terrain: Res<MarsTerrain>,
    state: Res<SurfaceState>,
    base: Res<crate::base::Base>,
    buildings: Res<crate::buildings::Buildings>,
    screen: Res<crate::ui::inventory::InventoryScreen>,
    mut root: Single<(Entity, &mut Node), (With<Root>, Without<CatalogRoot>)>,
    mut catalog: Single<(Entity, &mut Node), (With<CatalogRoot>, Without<Root>)>,
    mut recipe: Single<(&mut Text, &mut TextColor), (With<RecipeText>, Without<Label>)>,
    mut label: Single<(&mut Text, &mut TextColor), With<Label>>,
    camera: Single<Entity, With<crate::camera::GameCamera>>,
    mut commands: Commands,
) {
    let active =
        pilot.mode == MovementMode::Surface && !oxygen.failed && !menu.is_open() && !screen.open;
    catalog.1.display = if active && c.catalog_open {
        Display::Flex
    } else {
        Display::None
    };
    if active && c.catalog_open {
        commands.entity(catalog.0).insert(UiTargetCamera(*camera));
        let available = c
            .kind
            .recipe()
            .iter()
            .all(|(i, n)| c.inventory.count(*i) >= *n);
        **recipe.0 = format!(
            "{} — {}\n{}\nPorté : {} alliage / {} circuits · {}/8 piles",
            c.kind.name(),
            c.kind.recipe_label(),
            c.kind.description(),
            c.inventory.count(crate::inventory::Item::Alloy),
            c.inventory.count(crate::inventory::Item::Circuit),
            c.inventory.used()
        );
        recipe.1.0 = if available {
            theme::SAFE
        } else {
            theme::DANGER
        };
    }
    root.1.display = if active && !c.catalog_open {
        Display::Flex
    } else {
        Display::None
    };
    if !active {
        return;
    }
    commands.entity(root.0).insert(UiTargetCamera(*camera));
    let header = format!(
        "PORTÉ {}/8 piles · {} alliage / {} circuits · <key:KeyI> : inventaire",
        c.inventory.used(),
        c.inventory.count(crate::inventory::Item::Alloy),
        c.inventory.count(crate::inventory::Item::Circuit)
    );
    if let Some(p) = c.preview.as_ref().filter(|_| c.active) {
        let status = match p.verdict {
            Ok(()) => "AUTORISÉ · Entrée : construire",
            Err(reason) => reason,
        };
        label.1.0 = if p.verdict.is_ok() {
            theme::SAFE
        } else {
            theme::DANGER
        };
        **label.0 = format!(
            "{header}\n{} : {status}\nFlèches : déplacer · <key:KeyJ>/<key:KeyL> : orienter · souris : viser\n<key:KeyX> / <key:KeyB> / Échap : annuler · recettes payées uniquement depuis le porté",
            if p.verdict.is_ok() {
                "VERT"
            } else {
                "ROUGE — INTERDIT"
            }
        );
    } else {
        label.1.0 = theme::TEXT;
        let feet = terrain.body_relative(state.feet);
        let instruction = if let Some(f) = buildings.facilities.iter().find(|f| {
            crate::base::chest_access(
                &buildings,
                f.id,
                feet,
                terrain.body_relative(state.eye(&terrain)),
                &terrain,
            )
        }) {
            format!(
                "<key:KeyE> / <key:KeyI> : ouvrir le coffre {} · {}/16 piles",
                f.id.0,
                f.inventory.used()
            )
        } else if let Some(i) = base.nearby(feet) {
            let source = &base.sources[i];
            format!(
                "<key:KeyE> : {} +6 · renouvellement {:.0} s",
                source.item.name(),
                source.cooldown_s
            )
        } else if c.nearby(feet).is_some() {
            "<key:KeyE> : récupérer 6 alliage".into()
        } else {
            c.deposits
                .iter()
                .filter(|d| d.remaining > 0)
                .min_by(|a, b| {
                    a.position_m
                        .distance_squared(feet)
                        .total_cmp(&b.position_m.distance_squared(feet))
                })
                .map_or("Toutes les caisses ont été récupérées".into(), |d| {
                    let delta = state.local_rotation(&terrain).inverse()
                        * (terrain.orientation.inverse() * (d.position_m - feet));
                    let direction = if delta.z > delta.x.abs() {
                        "derrière"
                    } else if -delta.z > delta.x.abs() {
                        "devant"
                    } else if delta.x < 0.0 {
                        "à gauche"
                    } else {
                        "à droite"
                    };
                    format!(
                        "Caisse dorée · {:.0} m · {direction} · E à moins de 2,5 m",
                        d.position_m.distance(feet)
                    )
                })
        };
        **label.0 = format!(
            "{header}\n{instruction}\nSources durables cyan/ocre : derrière l’arrivée à 12–16 m\nB : construction · {}",
            c.message
        );
    }
}

#[expect(
    clippy::type_complexity,
    reason = "Boutons de recette et confirmation disjoints."
)]
fn interact(
    menu: Res<MenuState>,
    mut c: ResMut<Construction>,
    mut buttons: Query<
        (&Interaction, &BuildAction, &mut BackgroundColor),
        (Changed<Interaction>, Without<SelectRecipe>),
    >,
    mut recipes: Query<
        (&Interaction, &SelectRecipe, &mut BackgroundColor),
        (Changed<Interaction>, Without<BuildAction>),
    >,
) {
    if menu.is_open() || !c.catalog_open {
        return;
    }
    for (interaction, recipe, mut color) in &mut recipes {
        color.0 = if *interaction == Interaction::None {
            theme::BUTTON
        } else {
            theme::HOVER
        };
        if *interaction == Interaction::Pressed {
            c.kind = recipe.0;
        }
    }
    for (interaction, action, mut color) in &mut buttons {
        color.0 = match interaction {
            Interaction::Hovered => theme::HOVER,
            Interaction::Pressed => theme::ACTIVE,
            Interaction::None => theme::BUTTON,
        };
        if *interaction == Interaction::Pressed {
            if action.0 {
                c.place_requested = true;
            } else {
                c.close_requested = true;
            }
        }
    }
}
