use bevy::{asset::RenderAssetUsages, mesh::PrimitiveTopology, prelude::*};
/// Un mesh partagé, 2 400 étoiles artistiques déterministes, à l'infini de la caméra.
pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut vertices = Vec::with_capacity(7200);
    let mut colors = Vec::with_capacity(7200);
    for n in 0..2400 {
        let y = 1.0 - 2.0 * (n as f32 + 0.5) / 2400.0;
        let phi = n as f32 * 2.3999631;
        let radial = (1.0 - y * y).sqrt();
        let normal = Vec3::new(radial * phi.cos(), y, radial * phi.sin());
        let center = normal * 25_000.0;
        let tangent = normal.cross(Vec3::Y).normalize_or_zero();
        let bitangent = normal.cross(tangent);
        let size = 1.8 + (n % 13) as f32 * 0.35;
        let brightness = 0.35 + (n % 9) as f32 * 0.06;
        for offset in [
            tangent * size,
            -tangent * size + bitangent * size,
            -tangent * size - bitangent * size,
        ] {
            vertices.push((center + offset).to_array());
            colors.push([brightness * 0.85, brightness * 0.92, brightness, 1.0]);
        }
    }
    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    commands.spawn((
        Name::new("Champ d'étoiles"),
        Mesh3d(meshes.add(mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            unlit: true,
            cull_mode: None,
            ..default()
        })),
    ));
}
