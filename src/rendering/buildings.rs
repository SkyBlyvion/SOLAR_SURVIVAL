//! Proxies locaux des abris : même géométrie que les boîtes de contact, assets partagés.
use crate::{
    GameSet,
    buildings::{BuildingId, Buildings, OxygenShelter},
    planets::terrain::MarsTerrain,
    player::{Explorer, MovementMode, surface::SurfaceState},
    universe::coordinates::local_to_render,
};
use bevy::{camera::visibility::RenderLayers, prelude::*};
use std::collections::HashMap;

#[derive(Component)]
struct ShelterPart {
    id: BuildingId,
    index: usize,
}
#[derive(Resource)]
pub(super) struct BuildingAssets {
    pub(super) cube: Handle<Mesh>,
    wall: Handle<StandardMaterial>,
    roof: Handle<StandardMaterial>,
    door: Handle<StandardMaterial>,
    sign: Handle<StandardMaterial>,
    parts: [Handle<Mesh>; 6],
}
#[derive(Resource, Default)]
struct BuildingCache(HashMap<BuildingId, Vec<Entity>>);
pub(super) struct BuildingRenderingPlugin;
impl Plugin for BuildingRenderingPlugin {
    fn build(&self, app: &mut App) {
        if !app.world().resource::<crate::survival::Oxygen>().enabled {
            return;
        }
        app.init_resource::<BuildingCache>()
            .add_systems(Startup, setup)
            .add_systems(Update, (stream, sync).chain().in_set(GameSet::Rendering));
    }
}
pub(super) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    textures: Option<Res<super::textures::P7Textures>>,
) {
    let parts = OxygenShelter::parts();
    let side = meshes.add(panel_mesh(parts[0].0, parts[0].1));
    let back = meshes.add(panel_mesh(parts[2].0, parts[2].1));
    let front = meshes.add(panel_mesh(parts[3].0, parts[3].1));
    let roof = meshes.add(panel_mesh(parts[5].0, parts[5].1));
    commands.insert_resource(BuildingAssets {
        cube: meshes.add(readable_cube()),
        wall: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: textures.as_ref().map(|t| t.wall.clone()),
            perceptual_roughness: 0.88,
            reflectance: 0.15,
            ..default()
        }),
        roof: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: textures.as_ref().map(|t| t.roof.clone()),
            perceptual_roughness: 0.72,
            reflectance: 0.2,
            ..default()
        }),
        door: materials.add(StandardMaterial {
            base_color: Color::srgb(0.25, 0.65, 0.65),
            base_color_texture: textures.as_ref().map(|t| t.wall.clone()),
            perceptual_roughness: 0.88,
            reflectance: 0.15,
            ..default()
        }),
        sign: materials.add(StandardMaterial {
            base_color_texture: textures.as_ref().map(|t| t.sign.clone()),
            unlit: true,
            ..default()
        }),
        parts: [side.clone(), side, back, front.clone(), front, roof],
    });
}
/// Cuboid Bevy : UV latérales historiques tournées. Marquages lisibles sur chaque face.
fn readable_cube() -> Mesh {
    use bevy::mesh::VertexAttributeValues;
    let mut mesh = Mesh::from(Cuboid::new(1.0, 1.0, 1.0));
    let VertexAttributeValues::Float32x3(positions) =
        mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap()
    else {
        unreachable!()
    };
    let VertexAttributeValues::Float32x3(normals) = mesh.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap()
    else {
        unreachable!()
    };
    let uv: Vec<_> = positions
        .iter()
        .zip(normals)
        .map(|(p, n)| {
            if n[0].abs() > 0.5 {
                [0.5 - p[2] * n[0], 0.5 - p[1]]
            } else if n[2].abs() > 0.5 {
                [0.5 + p[0] * n[2], 0.5 - p[1]]
            } else {
                [0.5 + p[0], 0.5 + p[2] * n[1]]
            }
        })
        .collect();
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv);
    mesh
}
/// UV d'un panneau d'un mètre, avant l'échelle du proxy ; géométrie solide inchangée.
fn panel_mesh(center: bevy::math::DVec3, size: bevy::math::DVec3) -> Mesh {
    use bevy::mesh::VertexAttributeValues;
    let mut mesh = Mesh::from(Cuboid::new(1.0, 1.0, 1.0));
    let VertexAttributeValues::Float32x3(positions) =
        mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap()
    else {
        unreachable!()
    };
    let VertexAttributeValues::Float32x3(normals) = mesh.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap()
    else {
        unreachable!()
    };
    let uvs: Vec<_> = positions
        .iter()
        .zip(normals)
        .map(|(p, n)| {
            let p = Vec3::from_array(*p).as_dvec3() * size + center;
            if n[0].abs() > 0.5 {
                [p.z as f32, -p.y as f32]
            } else if n[1].abs() > 0.5 {
                [p.x as f32, p.z as f32]
            } else {
                [p.x as f32, -p.y as f32]
            }
        })
        .collect();
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}
/// Plaque de présentation au-dessus de la porte ; aucun collider/service supplémentaire.
fn visual_part(index: usize) -> (bevy::math::DVec3, bevy::math::DVec3) {
    use bevy::math::DVec3;
    if index == 6 {
        (DVec3::new(0.0, 2.95, 3.16), DVec3::new(2.4, 0.6, 0.04))
    } else {
        OxygenShelter::parts()[index]
    }
}
fn stream(
    mut commands: Commands,
    pilot: Res<Explorer>,
    state: Res<SurfaceState>,
    terrain: Res<MarsTerrain>,
    buildings: Res<Buildings>,
    assets: Res<BuildingAssets>,
    mut cache: ResMut<BuildingCache>,
) {
    let feet = terrain.body_relative(state.feet);
    let local = |id| {
        pilot.mode == MovementMode::Surface
            && buildings
                .shelters
                .iter()
                .any(|b| b.id == id && b.position_m.distance(feet) < 120.0)
    };
    cache.0.retain(|id, entities| {
        if local(*id) {
            return true;
        }
        for &entity in entities.iter() {
            commands.entity(entity).despawn();
        }
        false
    });
    for b in &buildings.shelters {
        if !local(b.id) {
            continue;
        }
        cache.0.entry(b.id).or_insert_with(|| {
            (0..7)
                .map(|index| {
                    commands
                        .spawn((
                            ShelterPart { id: b.id, index },
                            Mesh3d(if index == 6 {
                                assets.cube.clone()
                            } else {
                                assets.parts[index].clone()
                            }),
                            MeshMaterial3d(if index == 5 {
                                assets.roof.clone()
                            } else if index == 6 {
                                assets.sign.clone()
                            } else if index == 3 || index == 4 {
                                assets.door.clone()
                            } else {
                                assets.wall.clone()
                            }),
                            RenderLayers::layer(1),
                            Transform::default(),
                        ))
                        .id()
                })
                .collect()
        });
    }
}
fn sync(
    buildings: Res<Buildings>,
    terrain: Res<MarsTerrain>,
    state: Res<SurfaceState>,
    mut query: Query<(&ShelterPart, &mut Transform)>,
) {
    let eye = terrain.body_relative(state.eye(&terrain));
    let inverse = terrain.orientation.inverse();
    for (part, mut proxy) in &mut query {
        let b = buildings.shelters.iter().find(|b| b.id == part.id).unwrap();
        let (center, size) = visual_part(part.index);
        proxy.translation =
            local_to_render(inverse * ((b.position_m - eye) + b.orientation * center))
                .expect("Bâtiment chargé localement");
        proxy.rotation = (inverse * b.orientation).as_quat();
        proxy.scale = size.as_vec3();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::math::{DQuat, DVec3};
    #[test]
    fn shelter_proxies_share_assets_survive_rebase_and_unload_in_space() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<Explorer>()
            .init_resource::<SurfaceState>()
            .init_resource::<crate::universe::catalog::SolarSystem>()
            .init_resource::<MarsTerrain>()
            .init_resource::<Buildings>()
            .init_resource::<BuildingCache>()
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .add_systems(Startup, setup)
            .add_systems(Update, (stream, sync).chain());
        app.world_mut().resource_mut::<MarsTerrain>().globe_enabled = true;
        app.world_mut()
            .resource_mut::<MarsTerrain>()
            .set_site(DVec3::Y);
        app.world_mut()
            .resource_mut::<Buildings>()
            .install_shelter(DVec3::Y * 3389500.0 + DVec3::Z * -6.0, DQuat::IDENTITY);
        app.world_mut().resource_mut::<Explorer>().mode = MovementMode::Surface;
        app.update();
        let first = app.world().resource::<BuildingCache>().0.clone();
        assert_eq!(first.values().next().unwrap().len(), 7);
        assert_eq!(app.world().resource::<Assets<Mesh>>().len(), 5);
        app.world_mut().resource_mut::<SurfaceState>().feet.x = 64.1;
        app.world_mut()
            .resource_scope(|world, mut t: Mut<MarsTerrain>| {
                world.resource_mut::<SurfaceState>().migrate(&mut t);
            });
        app.update();
        assert_eq!(app.world().resource::<BuildingCache>().0, first);
        let terrain = app.world().resource::<MarsTerrain>();
        let eye = terrain.body_relative(app.world().resource::<SurfaceState>().eye(terrain));
        let q = terrain.orientation;
        let b = app.world().resource::<Buildings>().shelters[0].clone();
        for (part, proxy) in app
            .world_mut()
            .query::<(&ShelterPart, &Transform)>()
            .iter(app.world())
        {
            let (center, _) = visual_part(part.index);
            let restored = eye + q * proxy.translation.as_dvec3();
            assert!(restored.distance(b.position_m + b.orientation * center) < 5e-6);
        }
        app.world_mut().resource_mut::<Explorer>().mode = MovementMode::Space;
        app.update();
        assert!(app.world().resource::<BuildingCache>().0.is_empty());
        assert_eq!(app.world().resource::<Assets<Mesh>>().len(), 5);
        assert_eq!(
            app.world().resource::<Buildings>().shelters[0].position_m,
            b.position_m
        );
    }
}
