//! Rendu local en mètres, couche séparée de la profondeur astronomique compressée.
use crate::planets::{PatchKey, globe_terrain::SurfacePatch};
use crate::{
    GameSet,
    physics::surface::obstacle_pose,
    planets::terrain::{CELLS, MarsTerrain, TILE_M},
    player::{Explorer, MovementMode, surface::SurfaceState},
    universe::{
        SimulationClock, catalog::SolarSystem, celestial_body::BodyId, coordinates::local_to_render,
    },
};
use bevy::{
    asset::RenderAssetUsages,
    camera::{Hdr, visibility::RenderLayers},
    math::{DQuat, DVec3},
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};
use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(Component)]
pub struct SurfaceCamera;
#[derive(Component)]
struct LocalVisual {
    origin: DVec3,
}
#[derive(Component)]
struct ObstacleVisual(usize);
#[derive(Component)]
struct GlobeVisual {
    origin: DVec3,
    orientation: DQuat,
}
#[derive(Component)]
struct SurfaceSun;
#[derive(Resource)]
struct TerrainAssets {
    material: Handle<StandardMaterial>,
}
#[derive(Resource, Default)]
struct TileCache {
    revision: u64,
    pair: Vec<(Entity, Handle<Mesh>)>,
    tiles: BTreeMap<(i32, i32), (Entity, Handle<Mesh>)>,
    globe: HashMap<PatchKey, (Entity, Handle<Mesh>)>,
}

pub struct SurfaceRenderingPlugin;
impl Plugin for SurfaceRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileCache>()
            .add_systems(Startup, setup)
            .add_systems(Update, (stream, sync).chain().in_set(GameSet::Rendering));
    }
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        SurfaceCamera,
        Camera3d::default(),
        Camera {
            order: 1,
            is_active: false,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        Projection::Perspective(PerspectiveProjection {
            fov: 65_f32.to_radians(),
            near: 0.05,
            far: 1000.0,
            ..default()
        }),
        RenderLayers::layer(1),
        Transform::default(),
        Hdr,
        Msaa::Sample4,
        AmbientLight {
            color: Color::srgb(0.85, 0.64, 0.48),
            brightness: 100.0,
            ..default()
        },
        DistanceFog {
            color: Color::srgb(0.28, 0.13, 0.075),
            falloff: FogFalloff::Linear {
                start: 70.0,
                end: 125.0,
            },
            ..default()
        },
    ));
    commands.spawn((
        SurfaceSun,
        DirectionalLight {
            illuminance: 11000.0,
            shadow_maps_enabled: false,
            ..default()
        },
        RenderLayers::layer(1),
        Transform::default(),
    ));
    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let rock = materials.add(StandardMaterial {
        base_color: Color::srgb(0.19, 0.12, 0.08),
        perceptual_roughness: 1.0,
        ..default()
    });
    for i in 0..3 {
        commands.spawn((
            ObstacleVisual(i),
            LocalVisual {
                origin: DVec3::ZERO,
            },
            Mesh3d(cube.clone()),
            MeshMaterial3d(rock.clone()),
            RenderLayers::layer(1),
            Transform::default(),
            Visibility::Hidden,
        ));
    }
    commands.insert_resource(TerrainAssets {
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.55, 0.24, 0.115),
            perceptual_roughness: 1.0,
            ..default()
        }),
    });
}
fn tile_mesh(terrain: &MarsTerrain, key: (i32, i32)) -> Mesh {
    let origin = DVec3::new(key.0 as f64 * TILE_M, 0.0, key.1 as f64 * TILE_M);
    let mut positions = Vec::with_capacity((CELLS + 1).pow(2));
    let mut normals = Vec::with_capacity(positions.capacity());
    let mut colors = Vec::with_capacity(positions.capacity());
    let mut indices = Vec::with_capacity(CELLS * CELLS * 6);
    for z in 0..=CELLS {
        for x in 0..=CELLS {
            let ix = key.0 * CELLS as i32 + x as i32;
            let iz = key.1 * CELLS as i32 + z as i32;
            let point = terrain.vertex(ix, iz);
            positions.push(
                local_to_render(point - origin)
                    .expect("Tuile locale bornée")
                    .to_array(),
            );
            let dx = terrain.vertex(ix + 1, iz) - terrain.vertex(ix - 1, iz);
            let dz = terrain.vertex(ix, iz + 1) - terrain.vertex(ix, iz - 1);
            normals.push(dz.cross(dx).normalize().as_vec3().to_array());
            let shade = 0.87 + 0.13 * (point.x * 0.45 + point.z * 0.63).sin();
            colors.push([shade as f32, shade as f32, shade as f32, 1.0]);
            if x < CELLS && z < CELLS {
                let a = (z * (CELLS + 1) + x) as u32;
                let b = a + 1;
                let c = a + (CELLS + 1) as u32;
                let d = c + 1;
                indices.extend_from_slice(&[a, c, b, b, c, d]);
            }
        }
    }
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
    .with_inserted_indices(Indices::U32(indices))
}
/// Deux proxies utilisant exactement les indices physiques et le même tableau local.
fn cube_mesh(terrain: &MarsTerrain, patch: usize) -> Mesh {
    let cube = terrain.cube.as_ref().expect("Paire présente");
    let positions: Vec<_> = cube
        .local_positions
        .iter()
        .map(|&p| local_to_render(p).expect("Paire locale bornée").to_array())
        .collect();
    let normals: Vec<_> = cube
        .local_positions
        .iter()
        .map(|&p| terrain.site_relative(p).normalize().as_vec3().to_array())
        .collect();
    // Couleurs de diagnostic : rendent le raccord visible sans déplacer les sommets.
    // Identité de patch : le patch partagé conserve sa couleur au remplacement.
    let color = if cube.pair.keys[patch].x % 2 == 1 {
        [1.0, 0.8, 0.64, 1.0]
    } else {
        [0.68, 0.8, 1.0, 1.0]
    };
    let colors = vec![color; positions.len()];
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
    .with_inserted_indices(Indices::U32(cube.pair.indices[patch].clone()))
}
fn globe_mesh(patch: &SurfacePatch) -> (Mesh, GlobeVisual) {
    let up = patch.key.center_direction().unwrap();
    let radius = patch.positions_m[0].length();
    let origin = up * radius;
    let orientation = DQuat::from_rotation_arc(DVec3::Y, up);
    let inverse = orientation.inverse();
    let positions: Vec<_> = patch
        .positions_m
        .iter()
        .map(|p| local_to_render(inverse * (*p - origin)).unwrap().to_array())
        .collect();
    let normals: Vec<_> = patch
        .positions_m
        .iter()
        .map(|p| (inverse * p.normalize()).as_vec3().to_array())
        .collect();
    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_indices(Indices::U32(patch.indices.clone()));
    (
        mesh,
        GlobeVisual {
            origin,
            orientation,
        },
    )
}
fn stream(
    mut commands: Commands,
    pilot: Res<Explorer>,
    surface: Res<SurfaceState>,
    terrain: Res<MarsTerrain>,
    assets: Res<TerrainAssets>,
    mut cache: ResMut<TileCache>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let center = MarsTerrain::tile_at(surface.feet);
    let active = pilot.mode == MovementMode::Surface;
    let revision_changed = cache.revision != terrain.revision;
    cache.tiles.retain(|&(x, z), (entity, mesh)| {
        let keep = active
            && terrain.cube.is_none()
            && terrain.globe.is_none()
            && !revision_changed
            && (x - center.0).abs() <= 1
            && (z - center.1).abs() <= 1;
        if !keep {
            commands.entity(*entity).despawn();
            meshes.remove(mesh.id());
        }
        keep
    });
    let mut recycled = Vec::new();
    cache.globe.retain(|key, slot| {
        let keep = active
            && terrain
                .globe
                .as_ref()
                .is_some_and(|g| g.patches.contains_key(key));
        if !keep {
            recycled.push(slot.clone());
        }
        keep
    });
    if active && let Some(globe) = &terrain.globe {
        for (key, patch) in &globe.patches {
            cache.globe.entry(*key).or_insert_with(|| {
                let (mesh, visual) = globe_mesh(patch);
                if let Some((entity, handle)) = recycled.pop() {
                    *meshes.get_mut(&handle).unwrap() = mesh;
                    commands.entity(entity).insert(visual);
                    (entity, handle)
                } else {
                    let handle = meshes.add(mesh);
                    let entity = commands
                        .spawn((
                            LocalVisual {
                                origin: DVec3::ZERO,
                            },
                            visual,
                            Mesh3d(handle.clone()),
                            MeshMaterial3d(assets.material.clone()),
                            RenderLayers::layer(1),
                            Transform::default(),
                            Visibility::Hidden,
                        ))
                        .id();
                    (entity, handle)
                }
            });
        }
    }
    for (entity, handle) in recycled {
        commands.entity(entity).despawn();
        meshes.remove(handle.id());
    }
    if active && terrain.cube.is_some() {
        for patch in 0..2 {
            if cache.pair.len() <= patch {
                let mesh = meshes.add(cube_mesh(&terrain, patch));
                let entity = commands
                    .spawn((
                        LocalVisual {
                            origin: DVec3::ZERO,
                        },
                        Mesh3d(mesh.clone()),
                        MeshMaterial3d(assets.material.clone()),
                        RenderLayers::layer(1),
                        Transform::default(),
                        Visibility::Hidden,
                    ))
                    .id();
                cache.pair.push((entity, mesh));
            } else if revision_changed {
                *meshes
                    .get_mut(&cache.pair[patch].1)
                    .expect("Handle de paire résident") = cube_mesh(&terrain, patch);
            }
        }
    }
    cache.revision = terrain.revision;
    if !active || terrain.cube.is_some() || terrain.globe.is_some() {
        return;
    }
    for z in center.1 - 1..=center.1 + 1 {
        for x in center.0 - 1..=center.0 + 1 {
            cache.tiles.entry((x, z)).or_insert_with(|| {
                let mesh = meshes.add(tile_mesh(&terrain, (x, z)));
                let entity = commands
                    .spawn((
                        LocalVisual {
                            origin: DVec3::new(x as f64 * TILE_M, 0.0, z as f64 * TILE_M),
                        },
                        Mesh3d(mesh.clone()),
                        MeshMaterial3d(assets.material.clone()),
                        RenderLayers::layer(1),
                        Transform::default(),
                    ))
                    .id();
                (entity, mesh)
            });
        }
    }
}
#[expect(
    clippy::type_complexity,
    reason = "Filtres disjoints garantissant les emprunts des Transform ECS."
)]
#[expect(
    clippy::too_many_arguments,
    reason = "Paramètres de système injectés par Bevy ECS."
)]
fn sync(
    pilot: Res<Explorer>,
    surface: Res<SurfaceState>,
    terrain: Res<MarsTerrain>,
    solar: Res<SolarSystem>,
    clock: Res<SimulationClock>,
    mut camera: Single<
        (&mut Camera, &mut Transform),
        (
            With<SurfaceCamera>,
            Without<LocalVisual>,
            Without<SurfaceSun>,
        ),
    >,
    mut visuals: Query<
        (
            &LocalVisual,
            Option<&ObstacleVisual>,
            Option<&GlobeVisual>,
            &mut Transform,
            &mut Visibility,
        ),
        (Without<SurfaceCamera>, Without<SurfaceSun>),
    >,
    mut light: Single<
        (&mut Transform, &mut DirectionalLight),
        (
            With<SurfaceSun>,
            Without<SurfaceCamera>,
            Without<LocalVisual>,
        ),
    >,
) {
    let active = pilot.mode == MovementMode::Surface;
    let (camera, transform) = &mut *camera;
    camera.is_active = active;
    if !active {
        for (_, _, _, _, mut visibility) in &mut visuals {
            *visibility = Visibility::Hidden;
        }
        return;
    }
    transform.translation = Vec3::ZERO;
    transform.rotation = surface.local_rotation(&terrain).as_quat();
    // Soustraction dans le site en f64, sans aller-retour héliocentrique.
    let eye = surface.eye(&terrain);
    for (visual, obstacle, globe, mut transform, mut visibility) in &mut visuals {
        if let Some(globe) = globe {
            let inverse = terrain.orientation.inverse();
            let relative = inverse * (globe.origin - terrain.body_relative(eye));
            transform.rotation = (inverse * globe.orientation).as_quat();
            if let Some(relative) = local_to_render(relative) {
                transform.translation = relative;
                *visibility = Visibility::Visible;
            } else {
                *visibility = Visibility::Hidden;
            }
            continue;
        }
        if terrain.globe.is_some() && obstacle.is_none() {
            *visibility = Visibility::Hidden;
            continue;
        }
        let origin = if let Some(obstacle) = obstacle {
            let (center, size, rotation) = obstacle_pose(&terrain, obstacle.0);
            transform.scale = size.as_vec3();
            transform.rotation = rotation.as_quat();
            center
        } else {
            visual.origin
        };
        if let Some(relative) = local_to_render(origin - eye) {
            transform.translation = relative;
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
    let frame = terrain.inertial_frame(
        solar
            .body(BodyId::MARS)
            .rotation(clock.seconds_since_epoch()),
    );
    let to_sun = frame.orientation.inverse() * (-pilot.global(&solar, clock.jd)).normalize();
    let (transform, sunlight) = &mut *light;
    **transform = Transform::default().looking_to(-to_sun.as_vec3(), Vec3::Y);
    // La caméra locale ne rend pas la sphère de Mars et les ombres sont désactivées.
    // Sans ce masque, les parois reçoivent le Soleil même sous l'horizon radial.
    // Horizon idéal du site : pas d'occlusion du relief, réfraction ou pénombre.
    let radial_up = terrain.site_relative(eye).normalize();
    sunlight.illuminance = if to_sun.dot(radial_up) > 0.0 {
        11000.0
    } else {
        0.0
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn global_streaming_keeps_shared_handles_and_releases_all_assets_on_exit() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<Explorer>()
            .init_resource::<SurfaceState>()
            .init_resource::<SolarSystem>()
            .init_resource::<MarsTerrain>()
            .init_resource::<Assets<Mesh>>()
            .init_resource::<TileCache>()
            .insert_resource(TerrainAssets {
                material: Handle::default(),
            })
            .add_systems(Update, stream);
        {
            let mut terrain = app.world_mut().resource_mut::<MarsTerrain>();
            terrain.globe_enabled = true;
            terrain.set_site(DVec3::Y);
        }
        app.world_mut().resource_mut::<Explorer>().mode = MovementMode::Surface;
        app.update();
        let first = app.world().resource::<TileCache>().globe.clone();
        assert_eq!(first.len(), 13);
        for _ in 0..4 {
            app.world_mut().resource_mut::<SurfaceState>().feet = DVec3::new(64.1, 0.0, 0.0);
            app.world_mut()
                .resource_scope(|world, mut terrain: Mut<MarsTerrain>| {
                    assert!(world.resource_mut::<SurfaceState>().migrate(&mut terrain));
                });
            app.update();
        }
        let cache = app.world().resource::<TileCache>();
        assert_eq!(cache.globe.len(), 13);
        assert_eq!(app.world().resource::<Assets<Mesh>>().len(), 13);
        let mut shared = 0;
        for (key, slot) in &cache.globe {
            if let Some(old) = first.get(key) {
                assert_eq!(slot, old);
                shared += 1;
            }
            let visual = app.world().get::<GlobeVisual>(slot.0).unwrap();
            let terrain = app.world().resource::<MarsTerrain>();
            let patch = &terrain.globe.as_ref().unwrap().patches[key];
            let mesh = app.world().resource::<Assets<Mesh>>().get(&slot.1).unwrap();
            let bevy::mesh::VertexAttributeValues::Float32x3(positions) =
                mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap()
            else {
                panic!()
            };
            for (p, body) in positions.iter().zip(&patch.positions_m) {
                assert!(
                    (visual.origin + visual.orientation * Vec3::from_array(*p).as_dvec3())
                        .distance(*body)
                        < 0.00003
                );
            }
        }
        assert!(shared >= 8);
        let stable = cache.globe.clone();
        app.update();
        assert_eq!(app.world().resource::<TileCache>().globe, stable);
        app.world_mut().resource_mut::<Explorer>().mode = MovementMode::Space;
        app.update();
        assert!(app.world().resource::<TileCache>().globe.is_empty());
        assert!(app.world().resource::<Assets<Mesh>>().is_empty());
    }
    #[test]
    fn cube_mesh_matches_contacts_and_two_handles_survive_moves_rotation_and_reentry() {
        cube_handles(false);
    }
    #[test]
    fn offcenter_mesh_and_shared_colors_keep_two_handles_and_entities_after_migration_and_reentry()
    {
        cube_handles(true);
    }
    fn cube_handles(offcenter: bool) {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<Explorer>()
            .init_resource::<SurfaceState>()
            .init_resource::<SolarSystem>()
            .init_resource::<MarsTerrain>()
            .init_resource::<Assets<Mesh>>()
            .init_resource::<TileCache>()
            .insert_resource(TerrainAssets {
                material: Handle::default(),
            })
            .add_systems(Update, stream);
        {
            let mut terrain = app.world_mut().resource_mut::<MarsTerrain>();
            terrain.cube_enabled = true;
            terrain.migration_enabled = offcenter;
            terrain.offcenter_enabled = offcenter;
            terrain.set_site(DVec3::Y);
        }
        app.world_mut().resource_mut::<Explorer>().mode = MovementMode::Surface;
        app.update();
        let first = app.world().resource::<TileCache>().pair.clone();
        assert_eq!(first.len(), 2);
        assert!(app.world().resource::<TileCache>().tiles.is_empty());
        for (patch, (_, handle)) in first.iter().enumerate() {
            let world = app.world();
            let mesh = world.resource::<Assets<Mesh>>().get(handle).unwrap();
            let terrain = world.resource::<MarsTerrain>();
            let cube = terrain.cube.as_ref().unwrap();
            assert_eq!(
                mesh.indices().unwrap().iter().collect::<Vec<_>>(),
                cube.pair.indices[patch]
                    .iter()
                    .map(|&i| i as usize)
                    .collect::<Vec<_>>()
            );
            let bevy::mesh::VertexAttributeValues::Float32x3(positions) =
                mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap()
            else {
                panic!()
            };
            for (&actual, &expected) in positions.iter().zip(&cube.local_positions) {
                assert_eq!(actual, local_to_render(expected).unwrap().to_array());
                assert!(Vec3::from_array(actual).as_dvec3().distance(expected) < 0.00002);
            }
        }
        for x in [-90.0, -0.1, 0.0, 0.1, 90.0] {
            app.world_mut().resource_mut::<SurfaceState>().feet.x = x;
            app.update();
            assert_eq!(app.world().resource::<TileCache>().pair, first);
        }
        let old_colors = app
            .world()
            .resource::<Assets<Mesh>>()
            .get(&first[1].1)
            .unwrap()
            .attribute(Mesh::ATTRIBUTE_COLOR)
            .unwrap()
            .clone();
        app.world_mut()
            .resource_mut::<MarsTerrain>()
            .migration_enabled = true;
        let feet = if offcenter {
            let terrain = app.world().resource::<MarsTerrain>();
            let mut p = terrain
                .migration
                .as_ref()
                .unwrap()
                .preparation(terrain.cube.as_ref().unwrap().pair.keys[0]);
            p.y = 2.0;
            p
        } else {
            DVec3::new(101.0, 2.0, 30.0)
        };
        app.world_mut().resource_mut::<SurfaceState>().feet = feet;
        app.world_mut()
            .resource_scope(|world, mut terrain: Mut<MarsTerrain>| {
                assert!(world.resource_mut::<SurfaceState>().migrate(&mut terrain));
            });
        app.update();
        assert_eq!(app.world().resource::<TileCache>().pair, first);
        assert_eq!(app.world().resource::<Assets<Mesh>>().len(), 2);
        assert_eq!(
            app.world()
                .resource::<Assets<Mesh>>()
                .get(&first[0].1)
                .unwrap()
                .attribute(Mesh::ATTRIBUTE_COLOR)
                .unwrap(),
            &old_colors
        );
        for (patch, (_, handle)) in first.iter().enumerate() {
            let world = app.world();
            let mesh = world.resource::<Assets<Mesh>>().get(handle).unwrap();
            let cube = world.resource::<MarsTerrain>().cube.as_ref().unwrap();
            let bevy::mesh::VertexAttributeValues::Float32x3(positions) =
                mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap()
            else {
                panic!()
            };
            for (&actual, &p) in positions.iter().zip(&cube.local_positions) {
                assert_eq!(actual, local_to_render(p).unwrap().to_array());
            }
            assert_eq!(
                mesh.indices().unwrap().iter().collect::<Vec<_>>(),
                cube.pair.indices[patch]
                    .iter()
                    .map(|&i| i as usize)
                    .collect::<Vec<_>>()
            );
        }
        for direction in [DVec3::X, DVec3::NEG_Y, DVec3::Z] {
            app.world_mut().resource_mut::<Explorer>().mode = MovementMode::Space;
            app.update();
            assert_eq!(app.world().resource::<Assets<Mesh>>().len(), 2);
            app.world_mut()
                .resource_mut::<MarsTerrain>()
                .set_site(direction);
            app.world_mut().resource_mut::<Explorer>().mode = MovementMode::Surface;
            app.update();
            assert_eq!(app.world().resource::<TileCache>().pair, first);
            assert_eq!(app.world().resource::<Assets<Mesh>>().len(), 2);
            assert!(app.world().resource::<TileCache>().tiles.is_empty());
        }
    }

    #[test]
    fn tile_mesh_positions_match_physical_vertices_and_shared_edges() {
        let terrain = MarsTerrain {
            building_solids: Vec::new(),
            radius_m: 3389500.0,
            seed: 42,
            orientation: bevy::math::DQuat::IDENTITY,
            revision: 1,
            cube_enabled: false,
            migration_enabled: false,
            offcenter_enabled: false,
            migration: None,
            site_obstacles: None,
            cube: None,
            globe_enabled: false,
            globe: None,
        };
        let left = tile_mesh(&terrain, (-1, 0));
        let right = tile_mesh(&terrain, (0, 0));
        let bevy::mesh::VertexAttributeValues::Float32x3(left) =
            left.attribute(Mesh::ATTRIBUTE_POSITION).unwrap()
        else {
            panic!()
        };
        let bevy::mesh::VertexAttributeValues::Float32x3(right) =
            right.attribute(Mesh::ATTRIBUTE_POSITION).unwrap()
        else {
            panic!()
        };
        for z in 0..=CELLS {
            let a = Vec3::from_array(left[z * (CELLS + 1) + CELLS]) + Vec3::new(-128.0, 0.0, 0.0);
            let b = Vec3::from_array(right[z * (CELLS + 1)]);
            assert_eq!(a, b);
            assert!((b.y as f64 - terrain.vertex(0, z as i32).y).abs() < 1e-6);
        }
    }
    #[test]
    fn streaming_is_bounded_reuses_assets_and_releases_old_meshes() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<Explorer>()
            .init_resource::<SurfaceState>()
            .insert_resource(MarsTerrain {
                building_solids: Vec::new(),
                radius_m: 3389500.0,
                seed: 42,
                orientation: bevy::math::DQuat::IDENTITY,
                revision: 1,
                cube_enabled: false,
                migration_enabled: false,
                offcenter_enabled: false,
                migration: None,
                site_obstacles: None,
                cube: None,
                globe_enabled: false,
                globe: None,
            })
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<TileCache>()
            .insert_resource(TerrainAssets {
                material: Handle::default(),
            })
            .add_systems(Update, stream);
        app.world_mut().resource_mut::<Explorer>().mode = MovementMode::Surface;
        app.update();
        let first = app.world().resource::<TileCache>().tiles.clone();
        assert_eq!(first.len(), 9);
        for _ in 0..5 {
            app.update();
        }
        assert_eq!(app.world().resource::<TileCache>().tiles, first);
        app.world_mut().resource_mut::<SurfaceState>().feet.x = 129.0;
        app.update();
        let cache = app.world().resource::<TileCache>();
        assert_eq!(cache.tiles.len(), 9);
        for key in [(0, -1), (0, 0), (0, 1), (1, -1), (1, 0), (1, 1)] {
            assert_eq!(cache.tiles[&key], first[&key]);
        }
        assert_eq!(app.world().resource::<Assets<Mesh>>().len(), 9);
        app.world_mut().resource_mut::<Explorer>().mode = MovementMode::Space;
        app.update();
        assert!(app.world().resource::<TileCache>().tiles.is_empty());
        assert!(app.world().resource::<Assets<Mesh>>().is_empty());
    }

    #[test]
    fn rotating_site_keeps_local_proxies_and_meshes_but_changes_sun_direction() {
        use crate::universe::units::{DAY, J2000};
        let mut app = App::new();
        // Même composant requis que VisibilityPlugin dans DefaultPlugins,
        // sans initialiser le renderer GPU dans ce test ECS.
        app.world_mut()
            .register_required_components::<Mesh3d, Visibility>();
        app.add_plugins(MinimalPlugins)
            .init_resource::<SolarSystem>()
            .insert_resource(SimulationClock {
                jd: J2000,
                speed_index: 0,
                paused: false,
            })
            .init_resource::<Explorer>()
            .init_resource::<SurfaceState>()
            .init_resource::<MarsTerrain>()
            .init_resource::<Assets<Mesh>>()
            .init_resource::<TileCache>()
            .insert_resource(TerrainAssets {
                material: Handle::default(),
            })
            .add_systems(
                Update,
                (
                    |mut pilot: ResMut<Explorer>,
                     state: Res<SurfaceState>,
                     terrain: Res<MarsTerrain>,
                     solar: Res<SolarSystem>,
                     clock: Res<SimulationClock>| {
                        state.sync(
                            &mut pilot,
                            &terrain,
                            solar
                                .body(BodyId::MARS)
                                .rotation(clock.seconds_since_epoch()),
                        );
                    },
                    stream,
                    sync,
                )
                    .chain(),
            );
        app.world_mut().resource_mut::<Explorer>().mode = MovementMode::Surface;
        app.world_mut().resource_mut::<Explorer>().anchor = Some(BodyId::MARS);
        app.world_mut()
            .resource_mut::<MarsTerrain>()
            .set_site(DVec3::X);
        let camera = app
            .world_mut()
            .spawn((SurfaceCamera, Camera::default(), Transform::default()))
            .id();
        let light = app
            .world_mut()
            .spawn((
                SurfaceSun,
                Transform::default(),
                DirectionalLight::default(),
            ))
            .id();
        let obstacle = app
            .world_mut()
            .spawn((
                ObstacleVisual(0),
                LocalVisual {
                    origin: DVec3::ZERO,
                },
                Transform::default(),
                Visibility::Hidden,
            ))
            .id();
        app.update();
        let first = app.world().resource::<TileCache>().tiles.clone();
        let obstacle_pose = *app.world().get::<Transform>(obstacle).unwrap();
        let camera_pose = *app.world().get::<Transform>(camera).unwrap();
        let sun_start = *app.world().get::<Transform>(light).unwrap();
        assert_eq!(
            app.world()
                .get::<DirectionalLight>(light)
                .unwrap()
                .illuminance,
            0.0
        );
        let period = app
            .world()
            .resource::<SolarSystem>()
            .body(BodyId::MARS)
            .rotation_period_s;
        let revision = app.world().resource::<MarsTerrain>().revision;
        for fraction in [0.25, 0.5, 1.0] {
            app.world_mut().resource_mut::<SimulationClock>().jd = J2000 + period * fraction / DAY;
            app.update();
            assert_eq!(app.world().resource::<TileCache>().tiles, first);
            assert_eq!(app.world().resource::<Assets<Mesh>>().len(), 9);
            assert_eq!(app.world().resource::<MarsTerrain>().revision, revision);
            assert_eq!(
                *app.world().get::<Transform>(obstacle).unwrap(),
                obstacle_pose
            );
            assert_eq!(*app.world().get::<Transform>(camera).unwrap(), camera_pose);
            for (entity, _) in first.values() {
                let world = app.world();
                let relative = world.get::<LocalVisual>(*entity).unwrap().origin
                    - world
                        .resource::<SurfaceState>()
                        .eye(world.resource::<MarsTerrain>());
                assert_eq!(
                    world.get::<Transform>(*entity).unwrap().translation,
                    local_to_render(relative).unwrap()
                );
            }
            let world = app.world();
            let solar = world.resource::<SolarSystem>();
            let clock = world.resource::<SimulationClock>();
            let frame = world.resource::<MarsTerrain>().inertial_frame(
                solar
                    .body(BodyId::MARS)
                    .rotation(clock.seconds_since_epoch()),
            );
            let expected = frame.orientation.inverse()
                * (-world.resource::<Explorer>().global(solar, clock.jd)).normalize();
            let actual = world.get::<Transform>(light).unwrap().rotation * Vec3::Z;
            assert!(actual.distance(expected.as_vec3()) < 1e-6);
            if fraction == 0.5 {
                assert_eq!(
                    world.get::<DirectionalLight>(light).unwrap().illuminance,
                    11000.0
                );
            } else if fraction == 1.0 {
                assert_eq!(
                    world.get::<DirectionalLight>(light).unwrap().illuminance,
                    0.0
                );
            }
            if fraction == 0.25 {
                assert!(actual.distance(sun_start.rotation * Vec3::Z) > 0.5);
            }
        }
    }
}
