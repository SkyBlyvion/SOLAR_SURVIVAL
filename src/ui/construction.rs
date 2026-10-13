//! Inventaire et commandes P5 visibles même lorsque H masque la télémétrie.
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
            .add_systems(Update, refresh.in_set(GameSet::Interface));
    }
}
fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands
        .spawn((
            Root,
            Node {
                position_type: PositionType::Absolute,
                bottom: px(100),
                left: px(270),
                right: px(360),
                padding: UiRect::all(px(14)),
                border_radius: BorderRadius::all(px(8)),
                display: Display::None,
                ..default()
            },
            BackgroundColor(Color::srgba(0.025, 0.04, 0.065, 0.93)),
        ))
        .with_child((
            Label,
            Text::new("RÉCUPÉRATION"),
            TextFont {
                font: assets.load("fonts/DejaVuSans.ttf").into(),
                font_size: FontSize::Px(16.0),
                ..default()
            },
            TextColor(Color::WHITE),
        ));
}
#[expect(
    clippy::too_many_arguments,
    reason = "HUD d'inventaire et d'admission ECS."
)]
fn refresh(
    c: Res<Construction>,
    pilot: Res<Explorer>,
    oxygen: Res<Oxygen>,
    menu: Res<MenuState>,
    terrain: Res<MarsTerrain>,
    state: Res<SurfaceState>,
    mut root: Single<(Entity, &mut Node), With<Root>>,
    mut label: Single<(&mut Text, &mut TextColor), With<Label>>,
    camera: Single<Entity, With<crate::camera::GameCamera>>,
    mut commands: Commands,
) {
    let active = pilot.mode == MovementMode::Surface && !oxygen.failed && !menu.is_open();
    root.1.display = if active { Display::Flex } else { Display::None };
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
            Color::srgb(0.45, 1.0, 0.65)
        } else {
            Color::srgb(1.0, 0.48, 0.32)
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
        label.1.0 = Color::WHITE;
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
        **label.0 = format!(
            "{header}\n{instruction}\nB : placer un abri · {}",
            c.message
        );
    }
}
