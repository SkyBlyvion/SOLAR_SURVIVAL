//! Objectif et cap P10 visibles en surface sans ouvrir un menu ni suspendre l'oxygène.
use super::{HudState, theme};
use crate::{
    GameSet,
    buildings::Buildings,
    construction::Construction,
    expedition::{Expedition, REWARDS, SITE_COUNT, SITE_NAMES},
    planets::terrain::MarsTerrain,
    player::{Explorer, MovementMode, surface::SurfaceState},
    survival::Oxygen,
    ui::{inventory::InventoryScreen, menu::MenuState},
};
use bevy::prelude::*;

#[derive(Component)]
struct Root;
#[derive(Component)]
struct Label;
pub(super) struct ExpeditionHudPlugin;
impl Plugin for ExpeditionHudPlugin {
    fn build(&self, app: &mut App) {
        if !app.world().resource::<Oxygen>().enabled {
            return;
        }
        app.add_systems(Startup, setup).add_systems(
            Update,
            refresh.after(super::toggle).in_set(GameSet::Interface),
        );
    }
}
fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands
        .spawn((
            Root,
            Node {
                position_type: PositionType::Absolute,
                right: px(24),
                top: px(24),
                width: percent(40),
                max_width: px(420),
                padding: UiRect::all(px(14)),
                display: Display::None,
                ..default()
            },
            BackgroundColor(theme::PANEL),
        ))
        .with_child((
            Label,
            Text::new("EXPÉDITION"),
            super::font(14.0, &assets.load("fonts/DejaVuSans.ttf")),
            TextColor(theme::TEXT),
        ));
}
/// Cap relatif horizontal à la visée, sans convertir les coordonnées du corps en f32.
pub fn bearing(target: bevy::math::DVec3, terrain: &MarsTerrain, state: &SurfaceState) -> String {
    let delta = terrain.orientation.inverse() * (target - terrain.body_relative(state.feet));
    let forward = state.local_rotation(terrain) * -bevy::math::DVec3::Z;
    let forward =
        bevy::math::DVec3::new(forward.x, 0.0, forward.z).normalize_or(-bevy::math::DVec3::Z);
    let angle = delta
        .dot(forward.cross(bevy::math::DVec3::Y))
        .atan2(delta.dot(forward))
        .to_degrees();
    if angle.abs() <= 10.0 {
        "devant".into()
    } else if angle.abs() >= 170.0 {
        "derrière".into()
    } else {
        format!(
            "{} {:.0}°",
            if angle < 0.0 { "gauche" } else { "droite" },
            angle.abs()
        )
    }
}
#[expect(
    clippy::too_many_arguments,
    reason = "Objectif/navigation et visibilité des écrans du HUD."
)]
fn refresh(
    mut commands: Commands,
    e: Res<Expedition>,
    c: Res<Construction>,
    screen: Res<InventoryScreen>,
    menu: Res<MenuState>,
    hud: Res<HudState>,
    pilot: Res<Explorer>,
    oxygen: Res<Oxygen>,
    state: Res<SurfaceState>,
    terrain: Res<MarsTerrain>,
    buildings: Res<Buildings>,
    mut root: Single<(Entity, &mut Node), With<Root>>,
    mut label: Single<&mut Text, With<Label>>,
    camera: Single<Entity, With<crate::camera::GameCamera>>,
) {
    let active = pilot.mode == MovementMode::Surface
        && !menu.is_open()
        && !screen.open
        && !c.catalog_open
        && !c.active
        && !hud.visible
        && !hud.navigation
        && !e.sites.is_empty();
    root.1.display = if active { Display::Flex } else { Display::None };
    if !active {
        return;
    }
    commands.entity(root.0).insert(UiTargetCamera(*camera));
    let feet = terrain.body_relative(state.feet);
    let home = &buildings.shelters[0];
    let distance_home = home.position_m.distance(feet);
    let home_line = format!(
        "Base · {:.0} m · {}",
        distance_home,
        bearing(home.position_m, &terrain, &state)
    );
    let goal = if e.completed == SITE_COUNT {
        "3 / 3 RAPPORTS VALIDÉS\nExploration libre · récompenses reçues".into()
    } else {
        let (alloy, circuits) = REWARDS[e.completed];
        let target = e.target(&buildings).unwrap();
        format!(
            "EXPÉDITION {} / 3 · {}\n{} · {:.0} m · {}\nF : {}\nRécompense : {alloy} alliage + {circuits} circuits",
            e.completed + 1,
            SITE_NAMES[e.completed],
            if e.carrying {
                "Retour au premier abri"
            } else if e.discovered[e.completed] {
                "Balise connue · relevé à refaire"
            } else {
                "Rejoindre la balise"
            },
            target.distance(feet),
            bearing(target, &terrain, &state),
            if e.carrying {
                "transmettre dans le premier abri"
            } else {
                "relever à 3 m"
            }
        )
    };
    let estimated = distance_home / 4.5 + 20.0;
    let warning = if oxygen.failed {
        "Données non rapportées perdues".into()
    } else {
        format!(
            "Retour marche ≳ {:.0} s + marge 20 s{}",
            distance_home / 4.5,
            if oxygen.remaining_seconds < estimated {
                " · O₂ : rentrez / courez"
            } else {
                ""
            }
        )
    };
    ***label = format!(
        "{goal}\n{home_line}\n{warning}\n{}",
        if e.message.is_empty() {
            "Données perdues si échec ou sortie spatiale."
        } else {
            e.message
        }
    );
}
