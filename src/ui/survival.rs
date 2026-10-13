//! HUD vital indépendant de H : le manque d'oxygène conserve toujours une issue visible.
use crate::{
    GameSet,
    buildings::Buildings,
    planets::terrain::MarsTerrain,
    player::{Explorer, MovementMode, surface::SurfaceState},
    survival::{CAPACITY_SECONDS, LOW_SECONDS, Oxygen},
    ui::menu::MenuState,
};
use bevy::prelude::*;
#[derive(Component)]
struct OxygenRoot;
#[derive(Component)]
struct OxygenText;
#[derive(Component)]
struct OxygenBar;
pub(super) struct SurvivalHudPlugin;
impl Plugin for SurvivalHudPlugin {
    fn build(&self, app: &mut App) {
        if !app.world().resource::<Oxygen>().enabled {
            return;
        }
        app.add_systems(Startup, setup)
            .add_systems(Update, refresh.in_set(GameSet::Interface));
    }
}
fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let font = assets.load("fonts/DejaVuSans.ttf");
    commands
        .spawn((
            OxygenRoot,
            Node {
                position_type: PositionType::Absolute,
                top: px(24),
                left: px(270),
                right: px(360),
                padding: UiRect::all(px(14)),
                flex_direction: FlexDirection::Column,
                row_gap: px(8),
                display: Display::None,
                border_radius: BorderRadius::all(px(8)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.025, 0.04, 0.065, 0.93)),
        ))
        .with_children(|root| {
            root.spawn((
                OxygenText,
                Text::new("OXYGÈNE"),
                TextFont {
                    font: font.into(),
                    font_size: FontSize::Px(17.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            root.spawn((
                Node {
                    height: px(6),
                    width: percent(100),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.12, 0.18, 0.22)),
            ))
            .with_child((
                OxygenBar,
                Node {
                    height: percent(100),
                    width: percent(100),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.8, 0.75)),
            ));
        });
}
#[expect(
    clippy::too_many_arguments,
    clippy::type_complexity,
    reason = "Paramètres et filtres exclusifs du HUD vital ECS."
)]
fn refresh(
    oxygen: Res<Oxygen>,
    pilot: Res<Explorer>,
    state: Res<SurfaceState>,
    terrain: Res<MarsTerrain>,
    buildings: Res<Buildings>,
    menu: Res<MenuState>,
    mut root: Single<(Entity, &mut Node), With<OxygenRoot>>,
    mut text: Single<(&mut Text, &mut TextColor), With<OxygenText>>,
    mut bar: Single<(&mut Node, &mut BackgroundColor), (With<OxygenBar>, Without<OxygenRoot>)>,
    camera: Single<Entity, With<crate::camera::GameCamera>>,
    mut commands: Commands,
) {
    let active =
        oxygen.enabled && !menu.is_open() && (pilot.mode == MovementMode::Surface || oxygen.failed);
    root.1.display = if active { Display::Flex } else { Display::None };
    if !active {
        return;
    }
    commands.entity(root.0).insert(UiTargetCamera(*camera));
    let color = if oxygen.failed || oxygen.remaining_seconds <= LOW_SECONDS {
        Color::srgb(1.0, 0.42, 0.27)
    } else {
        Color::srgb(0.55, 0.95, 0.89)
    };
    text.1.0 = color;
    bar.0.width = percent((oxygen.remaining_seconds / CAPACITY_SECONDS * 100.0) as f32);
    bar.1.0 = color;
    if oxygen.failed {
        **text.0 =
            "OXYGÈNE ÉPUISÉ — Expédition terminée\nR : repartir à l'abri avec une réserve pleine"
                .into();
        return;
    }
    let seconds = oxygen.remaining_seconds.ceil() as u64;
    let status = if let Some(id) = oxygen.shelter {
        format!("Abri {} · recharge active (+30 s/s)", id.0)
    } else {
        let feet = terrain.body_relative(state.feet);
        buildings
            .shelters
            .iter()
            .min_by(|a, b| {
                a.position_m
                    .distance_squared(feet)
                    .total_cmp(&b.position_m.distance_squared(feet))
            })
            .map_or("Rejoignez un abri".into(), |b| {
                let delta = state.local_rotation(&terrain).inverse()
                    * (terrain.orientation.inverse() * (b.position_m - feet));
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
                    "Abri {} · {:.0} m · {direction} · entrez par la porte",
                    b.id.0,
                    b.position_m.distance(feet)
                )
            })
    };
    **text.0 = format!(
        "O₂  {:02}:{:02} / 03:00{}\n{status}",
        seconds / 60,
        seconds % 60,
        if oxygen.remaining_seconds <= LOW_SECONDS {
            " — RÉSERVE FAIBLE"
        } else {
            ""
        }
    );
}
