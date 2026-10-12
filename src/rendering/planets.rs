use crate::universe::{
    SimulationClock,
    catalog::SolarSystem,
    celestial_body::{BodyId, CelestialBody},
    coordinates::{AstronomicalPosition, RenderOrigin, render_proxy},
};
use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::ShaderRef,
};

#[derive(Clone, Copy, Debug, ShaderType)]
pub struct PlanetUniform {
    pub sun: Vec4,
    pub color_kind: Vec4,
    pub parameters: Vec4,
}
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PlanetMaterial {
    #[uniform(0)]
    pub data: PlanetUniform,
}
impl Material for PlanetMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet.wgsl".into()
    }
}
#[derive(Resource)]
pub struct SphereLods {
    pub near: Handle<Mesh>,
    pub far: Handle<Mesh>,
}
#[derive(Component)]
pub struct BodyVisual {
    pub id: BodyId,
    pub ring: bool,
}

pub fn spawn(
    mut commands: Commands,
    solar: Res<SolarSystem>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PlanetMaterial>>,
    mut standard: ResMut<Assets<StandardMaterial>>,
) {
    let near = meshes.add(
        Sphere::new(1.0)
            .mesh()
            .uv(128, 64)
            .rotated_by(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    );
    let far = meshes.add(
        Sphere::new(1.0)
            .mesh()
            .uv(16, 8)
            .rotated_by(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    );
    for body in &solar.bodies {
        let material = materials.add(PlanetMaterial {
            data: PlanetUniform {
                sun: Vec4::new(0.0, 0.0, 1.0, 0.0),
                color_kind: Vec4::new(
                    body.color[0],
                    body.color[1],
                    body.color[2],
                    body.id.0 as f32,
                ),
                parameters: Vec4::new((body.generation.seed % 4096) as f32, 0.0, 0.0, 0.0),
            },
        });
        commands.spawn((
            Name::new(body.name),
            BodyVisual {
                id: body.id,
                ring: false,
            },
            Mesh3d(near.clone()),
            MeshMaterial3d(material),
            Transform::default(),
        ));
    }
    // Anneaux principaux de Saturne: approximations géométriques, pas de particules individuelles.
    let ring = meshes.add(ring_mesh());
    commands.spawn((
        BodyVisual {
            id: BodyId(6),
            ring: true,
        },
        Mesh3d(ring),
        MeshMaterial3d(standard.add(StandardMaterial {
            base_color: Color::WHITE,
            unlit: true,
            cull_mode: None,
            alpha_mode: AlphaMode::Blend,
            ..default()
        })),
        Transform::default(),
    ));
    commands.insert_resource(SphereLods { near, far });
}
fn ring_mesh() -> Mesh {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut colors = Vec::new();
    let mut indices = Vec::new();
    let saturn_radius = 58_232.0;
    // C-ring inner edge → A-ring outer edge; NASA Saturnian Rings Fact Sheet, km.
    for band in 0..48 {
        let inner =
            74_658.0 / saturn_radius + (136_780.0 - 74_658.0) / saturn_radius * band as f32 / 48.0;
        let outer = 74_658.0 / saturn_radius
            + (136_780.0 - 74_658.0) / saturn_radius * (band + 1) as f32 / 48.0;
        let physical_mid = (inner + outer) * 0.5 * saturn_radius;
        let opacity = if (117_507.0..122_340.0).contains(&physical_mid) {
            0.06
        } else {
            0.45 + 0.3 * (band as f32 * 2.3).sin().abs()
        };
        for sector in 0..128 {
            let first = positions.len() as u32;
            for (r, a) in [
                (inner, sector),
                (outer, sector),
                (outer, sector + 1),
                (inner, sector + 1),
            ] {
                let angle = a as f32 / 128.0 * std::f32::consts::TAU;
                positions.push([r * angle.cos(), 0.0, r * angle.sin()]);
                normals.push([0.0, 1.0, 0.0]);
                colors.push([0.52, 0.44, 0.30, opacity]);
            }
            indices.extend_from_slice(&[first, first + 2, first + 1, first, first + 3, first + 2]);
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
pub fn sync(
    clock: Res<SimulationClock>,
    origin: Res<RenderOrigin>,
    lod: Res<SphereLods>,
    bodies: Query<(&CelestialBody, &AstronomicalPosition)>,
    mut visuals: Query<(
        &BodyVisual,
        &mut Transform,
        &mut Mesh3d,
        Option<&MeshMaterial3d<PlanetMaterial>>,
    )>,
    mut materials: ResMut<Assets<PlanetMaterial>>,
) {
    for (visual, mut transform, mut mesh, material) in &mut visuals {
        let Some((body, position)) = bodies.iter().find(|(body, _)| body.id == visual.id) else {
            continue;
        };
        let relative = position.0 - origin.0;
        let (center, radius) = render_proxy(relative, body.radius_m);
        transform.translation = center;
        transform.scale = Vec3::splat(radius);
        let angle = (clock.seconds_since_epoch() / body.rotation_period_s).rem_euclid(1.0)
            * std::f64::consts::TAU;
        let tilt = if body.rotation_period_s < 0.0 {
            180.0 - body.axial_tilt_deg
        } else {
            body.axial_tilt_deg
        };
        transform.rotation =
            Quat::from_rotation_z(tilt.to_radians() as f32) * Quat::from_rotation_y(angle as f32);
        if !visual.ring {
            let desired = if body.radius_m / relative.length().max(1.0) > 0.005 {
                &lod.near
            } else {
                &lod.far
            };
            if mesh.0 != *desired {
                mesh.0 = desired.clone();
            }
            if let Some(mut material) = material.and_then(|handle| materials.get_mut(&handle.0)) {
                let sun = (-position.0).normalize_or_zero().as_vec3();
                material.data.sun = sun.extend(0.0);
            }
        }
    }
}
