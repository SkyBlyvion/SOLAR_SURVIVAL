//! Inventaire et commandes P5 visibles même lorsque H masque la télémétrie.
use super::theme;
use crate::{
    GameSet,
    construction::{Construction, SHELTER_COST},
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
                    .before(crate::camera::controls)
                    .in_set(GameSet::Input),
            )
            .add_systems(Update, refresh.in_set(GameSet::Interface));
    }
}
fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts/DejaVuSans.ttf");
    commands.spawn((CatalogRoot,GlobalZIndex(30),Node {position_type:PositionType::Absolute,left:px(24),top:px(136),width:percent(44),max_width:px(440),padding:UiRect::all(px(24)),flex_direction:FlexDirection::Column,row_gap:px(16),display:Display::None,border:UiRect::left(px(2)),..default()},BackgroundColor(theme::PANEL),BorderColor::all(theme::ACCENT)))
        .with_children(|root| {
            root.spawn((Text::new("CONSTRUCTION / ABRIS"),super::font(12.0,&font),TextColor(theme::ACCENT)));
            root.spawn((Text::new("Abri à oxygène"),super::font(24.0,&font),TextColor(theme::TEXT)));
            root.spawn((Text::new("Recharge la combinaison lorsque vous entrez.\nEmprise au sol : 6,3 × 6,3 m."),super::font(14.0,&font),TextColor(theme::MUTED)));
            root.spawn((RecipeText,Text::new(""),super::font(16.0,&font),TextColor(theme::TEXT)));
            for (begin,title) in [(true,"Prévisualiser · Entrée"),(false,"Fermer · B / Échap")] {
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
    mut root: Single<(Entity, &mut Node), (With<Root>, Without<CatalogRoot>)>,
    mut catalog: Single<(Entity, &mut Node), (With<CatalogRoot>, Without<Root>)>,
    mut recipe: Single<(&mut Text, &mut TextColor), (With<RecipeText>, Without<Label>)>,
    mut label: Single<(&mut Text, &mut TextColor), With<Label>>,
    camera: Single<Entity, With<crate::camera::GameCamera>>,
    mut commands: Commands,
) {
    let active = pilot.mode == MovementMode::Surface && !oxygen.failed && !menu.is_open();
    catalog.1.display = if active && c.catalog_open {
        Display::Flex
    } else {
        Display::None
    };
    if active && c.catalog_open {
        commands.entity(catalog.0).insert(UiTargetCamera(*camera));
        let available = c.alloy >= SHELTER_COST;
        **recipe.0 = format!(
            "Coût : {SHELTER_COST} alliage · En stock : {}\n{}",
            c.alloy,
            if available {
                "Ressources disponibles"
            } else {
                "Ressources insuffisantes pour construire"
            }
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
    let header = format!("ALLIAGE  {}  ·  Abri : {SHELTER_COST} unités", c.alloy);
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
            "{header}\n{} : {status}\nFlèches : déplacer · J/L : orienter · souris : viser\nX / B / Échap : annuler · entrez ensuite par l’ouverture pour recharger",
            if p.verdict.is_ok() {
                "VERT"
            } else {
                "ROUGE — INTERDIT"
            }
        );
    } else {
        label.1.0 = theme::TEXT;
        let feet = terrain.body_relative(state.feet);
        let instruction = if c.nearby(feet).is_some() {
            "E : récupérer 6 alliage".into()
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
        **label.0 = format!("{header}\n{instruction}\nB : construction · {}", c.message);
    }
}

fn interact(
    menu: Res<MenuState>,
    mut c: ResMut<Construction>,
    mut buttons: Query<(&Interaction, &BuildAction, &mut BackgroundColor), Changed<Interaction>>,
) {
    if menu.is_open() || !c.catalog_open {
        return;
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
