//! Fond astronomique de la vue locale, à profondeur infinie (reverse-Z = 0).
//! Présentation artistique ; aucun état de simulation ni géométrie de support.
use crate::{
    GameSet,
    planets::terrain::MarsTerrain,
    player::{Explorer, MovementMode, surface::SurfaceState},
    universe::{SimulationClock, catalog::SolarSystem, celestial_body::BodyId},
};
use bevy::{
    asset::RenderAssetUsages,
    camera::visibility::{NoFrustumCulling, RenderLayers},
    image::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor},
    light::NotShadowCaster,
    math::{DQuat, DVec3},
    mesh::{MeshVertexBufferLayoutRef, PrimitiveTopology},
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypePath,
    render::render_resource::{
        AsBindGroup, RenderPipelineDescriptor, ShaderType, SpecializedMeshPipelineError,
        TextureDimension, TextureFormat,
    },
    shader::ShaderRef,
};

#[derive(Clone, Copy, Debug, ShaderType)]
pub struct SkyUniform {
    pub inertial_from_local: Mat4,
    pub up: Vec4,
    pub sun: Vec4,
    pub zenith: Vec4,
    pub horizon: Vec4,
    // Lointain → proche : les disques proches occultent les précédents.
    pub directions: [Vec4; 10],
    pub colors: [Vec4; 10],
    pub lighting: [Vec4; 10],
}
impl Default for SkyUniform {
    fn default() -> Self {
        Self {
            inertial_from_local: Mat4::IDENTITY,
            up: Vec3::Y.extend(0.0),
            sun: Vec3::Y.extend(0.0),
            zenith: Vec4::ZERO,
            horizon: Vec4::ZERO,
            directions: [Vec4::ZERO; 10],
            colors: [Vec4::ZERO; 10],
            lighting: [Vec4::ZERO; 10],
        }
    }
}
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SurfaceSkyMaterial {
    #[uniform(0)]
    pub data: SkyUniform,
    #[texture(1)]
    #[sampler(2)]
    pub stars: Handle<Image>,
}
impl Material for SurfaceSkyMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/surface_sky.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/surface_sky.wgsl".into()
    }
    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        if let Some(depth) = &mut descriptor.depth_stencil {
            depth.depth_write_enabled = Some(false);
        }
        Ok(())
    }
}
#[derive(Component)]
pub struct SurfaceSky;

pub struct SurfaceSkyPlugin;
impl Plugin for SurfaceSkyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<SurfaceSkyMaterial>::default())
            .add_systems(Startup, setup)
            .add_systems(Update, sync.in_set(GameSet::Rendering));
    }
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<SurfaceSkyMaterial>>,
) {
    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[-1.0, -1.0, 0.0], [3.0, -1.0, 0.0], [-1.0, 3.0, 0.0]],
    );
    commands.spawn((
        Name::new("Ciel martien / fond de la vue unique"),
        SurfaceSky,
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(SurfaceSkyMaterial {
            data: default(),
            stars: images.add(star_panorama()),
        })),
        Transform::default(),
        Visibility::Hidden,
        RenderLayers::layer(1),
        NoFrustumCulling,
        NotShadowCaster,
    ));
}
/// Palette de présentation partagée entre le fond et le brouillard local.
/// Intensités linéaires artistiques, sans modèle de diffusion atmosphérique.
pub fn sky_palette(elevation: f64) -> (Vec3, Vec3, f32) {
    let t = ((elevation + 0.10) / 0.30).clamp(0.0, 1.0) as f32;
    let day = t * t * (3.0 - 2.0 * t);
    let twilight = (1.0 - (elevation.abs() / 0.22).clamp(0.0, 1.0)) as f32;
    let zenith = Vec3::new(0.0005, 0.0008, 0.0015).lerp(Vec3::new(0.18, 0.105, 0.065), day);
    let horizon = Vec3::new(0.002, 0.0025, 0.004).lerp(Vec3::new(0.38, 0.245, 0.15), day)
        + Vec3::new(0.05, 0.014, 0.005) * twilight;
    (zenith, horizon, day)
}
/// Panorama statique : points fins, irréguliers et d'éclats variés. Graine artistique.
fn star_panorama() -> Image {
    const W: usize = 4096;
    const H: usize = 2048;
    let mut pixels = vec![0_u8; W * H * 4];
    for p in pixels.as_chunks_mut::<4>().0 {
        p[3] = 255;
    }
    let mut seed = 0x5a17_9c34_82ef_6b01_u64;
    let mut random = || {
        seed = seed.wrapping_add(0x9e3779b97f4a7c15);
        let mut x = seed;
        x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
        ((x ^ (x >> 31)) >> 11) as f64 / (1_u64 << 53) as f64
    };
    for _ in 0..2400 {
        let y = 2.0 * random() - 1.0;
        let x = (random() * W as f64) as usize;
        let row = ((0.5 - y.asin() / std::f64::consts::PI) * H as f64) as usize;
        let b = 0.06 + 0.7 * random().powi(5);
        let warmth = random();
        for (c, tint) in [0.8 + 0.2 * warmth, 0.88 + 0.1 * warmth, 1.0 - 0.2 * warmth]
            .into_iter()
            .enumerate()
        {
            pixels[(row.min(H - 1) * W + x) * 4 + c] = (b * tint * 255.0) as u8;
        }
    }
    let mut image = Image::new(
        bevy::render::render_resource::Extent3d {
            width: W as u32,
            height: H as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        pixels,
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD,
    );
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        ..ImageSamplerDescriptor::linear()
    });
    image
}
/// Soustraction astronomique f64 puis conversion de directions et angles seulement.
pub fn apparent_body(relative_m: DVec3, radius_m: f64, local_from_inertial: DQuat) -> (DVec3, f64) {
    let distance = relative_m.length();
    (
        local_from_inertial * (relative_m / distance),
        (radius_m / distance).clamp(0.0, 1.0).asin(),
    )
}
pub fn parameters(
    pilot: &Explorer,
    state: &SurfaceState,
    terrain: &MarsTerrain,
    solar: &SolarSystem,
    clock: &SimulationClock,
) -> SkyUniform {
    let frame = terrain.inertial_frame(
        solar
            .body(BodyId::MARS)
            .rotation(clock.seconds_since_epoch()),
    );
    let inverse = frame.orientation.inverse();
    let position = pilot.global(solar, clock.jd);
    let up = terrain.site_relative(state.eye(terrain)).normalize();
    let sun = inverse * (-position).normalize();
    let (zenith, horizon, day) = sky_palette(sun.dot(up));
    let mut data = SkyUniform {
        inertial_from_local: Mat4::from_quat(frame.orientation.as_quat()),
        up: up.as_vec3().extend(day),
        zenith: zenith.extend(0.0),
        horizon: horizon.extend(0.0),
        sun: sun.as_vec3().extend(sun.dot(up) as f32),
        ..default()
    };
    let mut bodies: Vec<_> = solar
        .bodies
        .iter()
        .filter(|b| b.id != BodyId::MARS)
        .map(|b| (b, solar.position(b.id, clock.jd) - position))
        .collect();
    bodies.sort_by(|a, b| b.1.length_squared().total_cmp(&a.1.length_squared()));
    for (i, (body, relative)) in bodies.into_iter().enumerate() {
        let (direction, angle) = apparent_body(relative, body.radius_m, inverse);
        data.directions[i] = direction.as_vec3().extend(angle as f32);
        data.colors[i] =
            Vec3::from_array(body.color).extend(if body.id == BodyId::SUN { 1.0 } else { 0.0 });
        let toward_sun = inverse * (-solar.position(body.id, clock.jd)).normalize_or_zero();
        // Phase moyenne pour un point non résolu ; disque résolu : normale par pixel.
        let phase = (1.0 + toward_sun.dot(-direction)) * 0.5;
        data.lighting[i] = toward_sun.as_vec3().extend(phase as f32);
    }
    data
}
fn sync(
    pilot: Res<Explorer>,
    state: Res<SurfaceState>,
    terrain: Res<MarsTerrain>,
    solar: Res<SolarSystem>,
    clock: Res<SimulationClock>,
    mut sky: Single<(&MeshMaterial3d<SurfaceSkyMaterial>, &mut Visibility), With<SurfaceSky>>,
    mut materials: ResMut<Assets<SurfaceSkyMaterial>>,
) {
    let active = pilot.mode == MovementMode::Surface;
    sky.1.clone_from(&if active {
        Visibility::Visible
    } else {
        Visibility::Hidden
    });
    if active {
        materials.get_mut(&sky.0.0).unwrap().data =
            parameters(&pilot, &state, &terrain, &solar, &clock);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::universe::units::{DAY, J2000};
    fn test_terrain() -> MarsTerrain {
        let mut world = World::new();
        world.init_resource::<SolarSystem>();
        MarsTerrain::from_world(&mut world)
    }
    #[test]
    fn angular_radius_and_direction_use_si_f64_before_conversion() {
        let rotation = DQuat::from_rotation_y(0.73);
        let (direction, angle) = apparent_body(DVec3::new(3e11, 0.0, 4e11), 6.957e8, rotation);
        assert!(direction.distance(rotation * DVec3::new(0.6, 0.0, 0.8)) < 1e-15);
        assert!((angle - (6.957e8_f64 / 5e11).asin()).abs() < 1e-15);
        assert!(angle > 0.001 && angle < 0.002);
    }
    #[test]
    fn sky_and_camera_project_the_same_inertial_directions_at_multiple_epochs() {
        let solar = SolarSystem::default();
        for jd in [J2000, J2000 + 0.5, 2461325.5] {
            let mut terrain = test_terrain();
            let mut state = SurfaceState::default();
            let mut pilot = Explorer::default();
            state.enter(&mut pilot, &mut terrain, &solar, jd);
            state.yaw = 0.4;
            state.pitch = -0.3;
            state.sync(
                &mut pilot,
                &terrain,
                solar.body(BodyId::MARS).rotation((jd - J2000) * DAY),
            );
            let clock = SimulationClock {
                jd,
                speed_index: 0,
                paused: true,
            };
            let data = parameters(&pilot, &state, &terrain, &solar, &clock);
            let local_rotation = state.local_rotation(&terrain);
            for body in &solar.bodies {
                if body.id == BodyId::MARS {
                    continue;
                }
                let relative = solar.position(body.id, jd) - pilot.global(&solar, jd);
                let local = (pilot.rotation.inverse() * relative.normalize()).as_vec3();
                let frame =
                    terrain.inertial_frame(solar.body(BodyId::MARS).rotation((jd - J2000) * DAY));
                let (direction, angle) =
                    apparent_body(relative, body.radius_m, frame.orientation.inverse());
                assert!(
                    (local_rotation.inverse() * direction)
                        .as_vec3()
                        .distance(local)
                        < 1e-6
                );
                assert!(
                    data.directions
                        .iter()
                        .any(|d| d.truncate().distance(direction.as_vec3()) < 1e-7
                            && (d.w - angle as f32).abs() < 1e-9)
                );
            }
            let sun = (terrain
                .inertial_frame(solar.body(BodyId::MARS).rotation((jd - J2000) * DAY))
                .orientation
                .inverse()
                * (-pilot.global(&solar, jd)).normalize())
            .as_vec3();
            assert!(data.sun.truncate().distance(sun) < 1e-7);
        }
    }
    #[test]
    fn rebase_and_walk_keep_inertial_stars_and_celestial_angles() {
        let solar = SolarSystem::default();
        let clock = SimulationClock {
            jd: J2000,
            speed_index: 0,
            paused: true,
        };
        let mut terrain = test_terrain();
        terrain.globe_enabled = true;
        let mut state = SurfaceState::default();
        let mut pilot = Explorer::default();
        state.enter(&mut pilot, &mut terrain, &solar, clock.jd);
        state.feet.x = 64.1;
        state.sync(
            &mut pilot,
            &terrain,
            solar
                .body(BodyId::MARS)
                .rotation(clock.seconds_since_epoch()),
        );
        let before = parameters(&pilot, &state, &terrain, &solar, &clock);
        assert!(state.migrate(&mut terrain));
        state.sync(
            &mut pilot,
            &terrain,
            solar
                .body(BodyId::MARS)
                .rotation(clock.seconds_since_epoch()),
        );
        let after = parameters(&pilot, &state, &terrain, &solar, &clock);
        for i in 0..9 {
            let a = before
                .inertial_from_local
                .transform_vector3(before.directions[i].truncate());
            let b = after
                .inertial_from_local
                .transform_vector3(after.directions[i].truncate());
            assert!(a.distance(b) < 3e-7);
            assert!((before.directions[i].w - after.directions[i].w).abs() < 1e-9);
        }
    }
    #[test]
    fn sky_assets_and_entity_are_reused_across_rotation_and_mode_changes() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<Assets<Mesh>>()
            .init_resource::<Assets<Image>>()
            .init_resource::<Assets<SurfaceSkyMaterial>>()
            .init_resource::<Explorer>()
            .init_resource::<SurfaceState>()
            .init_resource::<SolarSystem>()
            .init_resource::<MarsTerrain>()
            .insert_resource(SimulationClock {
                jd: J2000,
                speed_index: 0,
                paused: true,
            })
            .add_systems(Startup, setup)
            .add_systems(Update, sync);
        app.update();
        let entity = app
            .world_mut()
            .query_filtered::<Entity, With<SurfaceSky>>()
            .single(app.world())
            .unwrap();
        let mesh = app.world().get::<Mesh3d>(entity).unwrap().0.clone();
        let material = app
            .world()
            .get::<MeshMaterial3d<SurfaceSkyMaterial>>(entity)
            .unwrap()
            .0
            .clone();
        for i in 0..20 {
            let surface = i % 2 == 0;
            app.world_mut().resource_mut::<Explorer>().mode = if surface {
                MovementMode::Surface
            } else {
                MovementMode::Space
            };
            app.world_mut().resource_mut::<SimulationClock>().jd += 0.1;
            app.update();
            assert_eq!(app.world().get::<Mesh3d>(entity).unwrap().0, mesh);
            assert_eq!(
                app.world()
                    .get::<MeshMaterial3d<SurfaceSkyMaterial>>(entity)
                    .unwrap()
                    .0,
                material
            );
            assert_eq!(
                *app.world().get::<Visibility>(entity).unwrap(),
                if surface {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                }
            );
            assert_eq!(app.world().resource::<Assets<Mesh>>().len(), 1);
            assert_eq!(app.world().resource::<Assets<Image>>().len(), 1);
            assert_eq!(
                app.world().resource::<Assets<SurfaceSkyMaterial>>().len(),
                1
            );
        }
    }
}
