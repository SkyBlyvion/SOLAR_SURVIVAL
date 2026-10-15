//! Balises artistiques P10 : deux cubes partagés par site, poses corporelles en lecture.
use super::buildings::BuildingAssets;
use crate::{
    GameSet,
    expedition::Expedition,
    planets::terrain::MarsTerrain,
    player::{Explorer, MovementMode, surface::SurfaceState},
    universe::coordinates::local_to_render,
};
use bevy::{camera::visibility::RenderLayers, math::DVec3, prelude::*};
use std::collections::HashMap;

#[derive(Resource)]
struct Materials([Handle<StandardMaterial>; 3]);
#[derive(Resource, Default)]
struct Cache(HashMap<usize, [Entity; 2]>);
pub(super) struct ExpeditionRenderingPlugin;
impl Plugin for ExpeditionRenderingPlugin {
    fn build(&self, app: &mut App) {
        if !app.world().resource::<crate::survival::Oxygen>().enabled {
            return;
        }
        app.init_resource::<Cache>()
            .add_systems(Startup, setup)
            .add_systems(Update, sync.in_set(GameSet::Rendering));
    }
}
fn setup(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(Materials(
        [
            Color::srgb(0.18, 0.22, 0.24),
            Color::srgb(1.0, 0.65, 0.12),
            Color::srgb(0.2, 0.8, 0.75),
        ]
        .map(|color| {
            materials.add(StandardMaterial {
                base_color: color,
                unlit: true,
                perceptual_roughness: 1.0,
                ..default()
            })
        }),
    ));
}
#[expect(
    clippy::too_many_arguments,
    reason = "Caches et données de rendu distincts, aucune mutation de simulation."
)]
fn sync(
    mut commands: Commands,
    e: Res<Expedition>,
    terrain: Res<MarsTerrain>,
    state: Res<SurfaceState>,
    pilot: Res<Explorer>,
    assets: Res<BuildingAssets>,
    materials: Res<Materials>,
    mut cache: ResMut<Cache>,
) {
    let feet = terrain.body_relative(state.feet);
    let eye = terrain.body_relative(state.eye(&terrain));
    let visible = |i: usize| {
        pilot.mode == MovementMode::Surface
            && i <= e.completed
            && e.sites
                .get(i)
                .is_some_and(|s| s.position_m.distance(feet) < 600.0)
    };
    cache.0.retain(|&i, entities| {
        if visible(i) {
            true
        } else {
            for &entity in entities.iter() {
                commands.entity(entity).despawn();
            }
            false
        }
    });
    for (i, site) in e.sites.iter().enumerate().filter(|(i, _)| visible(*i)) {
        let entities = cache.0.entry(i).or_insert_with(|| {
            std::array::from_fn(|_| {
                commands
                    .spawn((
                        Mesh3d(assets.cube.clone()),
                        RenderLayers::layer(1),
                        Transform::default(),
                    ))
                    .id()
            })
        });
        for (part, ((center, size), &entity)) in [
            (DVec3::Y * 1.0, DVec3::new(0.16, 2.6, 0.16)),
            (DVec3::Y * 2.3, DVec3::new(0.7, 0.4, 0.7)),
        ]
        .into_iter()
        .zip(entities.iter())
        .enumerate()
        {
            let relative = (site.position_m - eye) + site.orientation * center;
            commands.entity(entity).insert((
                Transform {
                    translation: local_to_render(terrain.orientation.inverse() * relative)
                        .expect("Balise locale P10"),
                    rotation: (terrain.orientation.inverse() * site.orientation).as_quat(),
                    scale: size.as_vec3(),
                },
                MeshMaterial3d(
                    materials.0[if part == 0 {
                        0
                    } else if i < e.completed {
                        2
                    } else {
                        1
                    }]
                    .clone(),
                ),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{buildings::Buildings, universe::catalog::SolarSystem};
    #[test]
    fn bounded_shared_proxies_keep_identity_precision_and_unload_across_profiles() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<SolarSystem>()
            .init_resource::<MarsTerrain>()
            .init_resource::<SurfaceState>()
            .init_resource::<Explorer>()
            .init_resource::<Expedition>()
            .init_resource::<Cache>()
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .add_systems(Startup, (super::super::buildings::setup, setup))
            .add_systems(Update, sync);
        app.world_mut().resource_mut::<Explorer>().mode = MovementMode::Surface;
        app.world_mut()
            .resource_scope(|world, mut terrain: Mut<MarsTerrain>| {
                terrain.globe_enabled = true;
                terrain.set_site(DVec3::new(0.3, 1.0, -0.4).normalize());
                let mut buildings = Buildings::default();
                buildings.install_first(&terrain);
                world
                    .resource_mut::<Expedition>()
                    .seed_site(&terrain, &buildings);
            });
        app.update();
        assert_eq!(app.world().resource::<Cache>().0.len(), 1);
        let handles = app.world().resource::<Assets<Mesh>>().len();
        let entities = app.world().resource::<Cache>().0[&0];
        app.world_mut()
            .resource_scope(|world, mut terrain: Mut<MarsTerrain>| {
                let mut state = world.resource_mut::<SurfaceState>();
                state.feet.x = 64.1;
                state.feet.y = terrain.support_height(state.feet, 0.3) + 0.002;
                assert!(state.migrate(&mut terrain));
            });
        app.update();
        assert_eq!(app.world().resource::<Cache>().0[&0], entities);
        let t = app.world().resource::<MarsTerrain>();
        let eye = t.body_relative(app.world().resource::<SurfaceState>().eye(t));
        let site = &app.world().resource::<Expedition>().sites[0];
        for (i, e) in entities.iter().enumerate() {
            let transform = app.world().get::<Transform>(*e).unwrap();
            let restored = eye + t.orientation * transform.translation.as_dvec3();
            assert!(
                restored.distance(
                    site.position_m + site.orientation * DVec3::Y * if i == 0 { 1.0 } else { 2.3 }
                ) < 20e-6
            );
        }
        app.world_mut().resource_mut::<Expedition>().completed = 3;
        app.update();
        assert_eq!(app.world().resource::<Cache>().0.len(), 3);
        assert_eq!(app.world().resource::<Assets<Mesh>>().len(), handles);
        app.world_mut().resource_mut::<Explorer>().mode = MovementMode::Space;
        app.update();
        assert!(app.world().resource::<Cache>().0.is_empty());
        app.world_mut().resource_mut::<Explorer>().mode = MovementMode::Surface;
        app.update();
        assert_eq!(app.world().resource::<Cache>().0.len(), 3);
        app.world_mut().insert_resource(Expedition::default());
        app.update();
        assert!(app.world().resource::<Cache>().0.is_empty());
        assert_eq!(app.world().resource::<Assets<Mesh>>().len(), handles);
    }
}
