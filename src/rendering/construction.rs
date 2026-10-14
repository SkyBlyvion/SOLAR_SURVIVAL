//! Preview et récupération : proxies locaux seulement, cube de l'abri partagé.
use super::buildings::BuildingAssets;
use crate::{
    GameSet,
    buildings::OxygenShelter,
    construction::Construction,
    planets::terrain::MarsTerrain,
    player::{Explorer, MovementMode, surface::SurfaceState},
    universe::coordinates::local_to_render,
};
use bevy::{
    camera::visibility::RenderLayers,
    math::{DQuat, DVec3},
    prelude::*,
};
use std::collections::HashMap;
#[derive(Resource)]
struct Materials {
    allowed: Handle<StandardMaterial>,
    denied: Handle<StandardMaterial>,
    salvage: Handle<StandardMaterial>,
}
#[derive(Resource, Default)]
struct Cache {
    preview: Vec<Entity>,
    deposits: HashMap<usize, Entity>,
}
pub(super) struct ConstructionRenderingPlugin;
impl Plugin for ConstructionRenderingPlugin {
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
    let ghost = |color: Color| StandardMaterial {
        base_color: color,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        cull_mode: None,
        ..default()
    };
    commands.insert_resource(Materials {
        allowed: materials.add(ghost(Color::srgba(0.2, 1.0, 0.5, 0.48))),
        denied: materials.add(ghost(Color::srgba(1.0, 0.15, 0.08, 0.48))),
        salvage: materials.add(StandardMaterial {
            base_color: Color::srgb(0.85, 0.57, 0.12),
            emissive: LinearRgba::new(0.1, 0.06, 0.005, 1.0),
            perceptual_roughness: 0.7,
            ..default()
        }),
    });
}
fn proxy(
    terrain: &MarsTerrain,
    eye: DVec3,
    position: DVec3,
    orientation: DQuat,
    size: DVec3,
) -> Transform {
    Transform {
        translation: local_to_render(terrain.orientation.inverse() * (position - eye))
            .expect("Proxy local P5"),
        rotation: (terrain.orientation.inverse() * orientation).as_quat(),
        scale: size.as_vec3(),
    }
}
#[expect(
    clippy::too_many_arguments,
    reason = "Synchronisation des seuls proxies P5, données physiques en lecture."
)]
fn sync(
    mut commands: Commands,
    c: Res<Construction>,
    terrain: Res<MarsTerrain>,
    state: Res<SurfaceState>,
    pilot: Res<Explorer>,
    assets: Res<BuildingAssets>,
    materials: Res<Materials>,
    mut cache: ResMut<Cache>,
) {
    let surface = pilot.mode == MovementMode::Surface;
    let eye = terrain.body_relative(state.eye(&terrain));
    let feet = terrain.body_relative(state.feet);
    cache.deposits.retain(|&index, entity| {
        let Some(d) = c.deposits.get(index) else {
            commands.entity(*entity).despawn();
            return false;
        };
        if surface && d.remaining > 0 && d.position_m.distance(feet) < 120.0 {
            return true;
        }
        commands.entity(*entity).despawn();
        false
    });
    if surface {
        for (index, d) in c
            .deposits
            .iter()
            .enumerate()
            .filter(|(_, d)| d.remaining > 0 && d.position_m.distance(feet) < 120.0)
        {
            let entity = *cache.deposits.entry(index).or_insert_with(|| {
                commands
                    .spawn((
                        Mesh3d(assets.cube.clone()),
                        MeshMaterial3d(materials.salvage.clone()),
                        RenderLayers::layer(1),
                        Transform::default(),
                    ))
                    .id()
            });
            commands.entity(entity).insert(proxy(
                &terrain,
                eye,
                d.position_m + d.orientation * DVec3::Y * 0.4,
                d.orientation,
                DVec3::new(1.0, 0.8, 1.0),
            ));
        }
    }
    let preview = c.preview.as_ref().filter(|_| surface && c.active);
    let Some(p) = preview else {
        for entity in cache.preview.drain(..) {
            commands.entity(entity).despawn();
        }
        return;
    };
    if cache.preview.is_empty() {
        cache.preview = (0..6)
            .map(|_| {
                commands
                    .spawn((
                        Mesh3d(assets.cube.clone()),
                        RenderLayers::layer(1),
                        Transform::default(),
                    ))
                    .id()
            })
            .collect();
    }
    for (index, &entity) in cache.preview.iter().enumerate() {
        let (center, size) = OxygenShelter::parts()[index];
        commands.entity(entity).insert((
            proxy(
                &terrain,
                eye,
                p.position_m + p.orientation * center,
                p.orientation,
                size,
            ),
            MeshMaterial3d(if p.verdict.is_ok() {
                materials.allowed.clone()
            } else {
                materials.denied.clone()
            }),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{buildings::Buildings, construction::Placement, universe::catalog::SolarSystem};
    #[test]
    fn preview_and_salvage_share_cube_keep_entities_on_rebase_and_unload() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<SolarSystem>()
            .init_resource::<MarsTerrain>()
            .init_resource::<Construction>()
            .init_resource::<Explorer>()
            .init_resource::<SurfaceState>()
            .init_resource::<Cache>()
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .add_systems(Startup, (super::super::buildings::setup, setup))
            .add_systems(Update, sync);
        {
            let mut t = app.world_mut().resource_mut::<MarsTerrain>();
            t.globe_enabled = true;
            t.set_site(DVec3::Y);
        }
        app.world_mut()
            .resource_scope(|world, t: Mut<MarsTerrain>| {
                let mut c = world.resource_mut::<Construction>();
                c.seed_site(&t);
                c.alloy = 12;
                c.active = true;
                c.lateral_m = 16.0;
                c.preview = Some(c.candidate(&t, &SurfaceState::default(), &Buildings::default()));
            });
        app.world_mut().resource_mut::<Explorer>().mode = MovementMode::Surface;
        app.update();
        assert_eq!(app.world().resource::<Assets<Mesh>>().len(), 1);
        assert_eq!(app.world().resource::<Cache>().deposits.len(), 8);
        let preview = app.world().resource::<Cache>().preview.clone();
        let deposits = app.world().resource::<Cache>().deposits.clone();
        assert_eq!(preview.len(), 6);
        let pose = app
            .world()
            .resource::<Construction>()
            .preview
            .as_ref()
            .unwrap()
            .clone();
        app.world_mut().resource_mut::<SurfaceState>().feet.x = 64.1;
        app.world_mut()
            .resource_scope(|world, mut t: Mut<MarsTerrain>| {
                world.resource_mut::<SurfaceState>().migrate(&mut t);
            });
        app.update();
        assert_eq!(app.world().resource::<Cache>().preview, preview);
        assert_eq!(app.world().resource::<Cache>().deposits, deposits);
        let t = app.world().resource::<MarsTerrain>();
        let eye = t.body_relative(app.world().resource::<SurfaceState>().eye(t));
        for (i, entity) in preview.iter().enumerate() {
            let transform = app.world().get::<Transform>(*entity).unwrap();
            let restored = eye + t.orientation * transform.translation.as_dvec3();
            assert!(
                restored.distance(pose.position_m + pose.orientation * OxygenShelter::parts()[i].0)
                    < 5e-6
            );
        }
        app.world_mut().resource_mut::<Construction>().preview = Some(Placement {
            verdict: Err("obstacle"),
            ..pose.clone()
        });
        app.update();
        for e in &preview {
            assert_eq!(
                app.world()
                    .get::<MeshMaterial3d<StandardMaterial>>(*e)
                    .unwrap()
                    .0,
                app.world().resource::<Materials>().denied
            );
        }
        app.world_mut().resource_mut::<Construction>().deposits[0].remaining = 0;
        app.update();
        assert_eq!(app.world().resource::<Cache>().deposits.len(), 7);
        app.world_mut().resource_mut::<Explorer>().mode = MovementMode::Space;
        app.update();
        assert!(app.world().resource::<Cache>().deposits.is_empty());
        assert!(app.world().resource::<Cache>().preview.is_empty());
        assert_eq!(app.world().resource::<Assets<Mesh>>().len(), 1);
        assert_eq!(
            app.world()
                .resource::<Construction>()
                .preview
                .as_ref()
                .unwrap()
                .position_m,
            pose.position_m
        );
        app.world_mut().resource_mut::<Explorer>().mode = MovementMode::Surface;
        app.update();
        assert_eq!(app.world().resource::<Cache>().deposits.len(), 7);
        // Une sauvegarde antérieure à l'arrivée remplace les huit dépôts par zéro.
        app.world_mut().insert_resource(Construction::default());
        app.update();
        assert!(app.world().resource::<Cache>().deposits.is_empty());
        assert!(app.world().resource::<Cache>().preview.is_empty());
    }
}
