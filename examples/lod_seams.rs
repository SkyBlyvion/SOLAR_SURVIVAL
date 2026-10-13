//! Diagnostic graphique de trois résolutions, faces/arêtes/coins et transitions.
//! cargo build --release --locked --example lod_seams
//! target/release/examples/lod_seams [nouveau-dossier-captures]
use bevy::{
    app::AppExit,
    asset::RenderAssetUsages,
    math::{DQuat, DVec3},
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};
use solar_survival::{
    planets::{CubeFace, PatchEdge, PatchKey, globe_terrain::SurfacePatch},
    universe::{catalog::SolarSystem, celestial_body::BodyId, coordinates::local_to_render},
};
use std::path::PathBuf;

const STAGE_SECONDS: f64 = 6.0;

#[derive(Resource)]
struct Demo {
    meshes: [Handle<Mesh>; 4],
    radius_m: f64,
    stage: usize,
    captured: bool,
    output: Option<PathBuf>,
}
#[derive(Component)]
struct Label;

fn main() {
    let output = std::env::args_os().nth(1).map(PathBuf::from);
    if let Some(path) = &output {
        // Refuse de remplacer une campagne déjà existante.
        std::fs::create_dir(path).expect("Choisir un nouveau dossier dans un parent existant");
    }
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Solar Survival - Adaptive resolution seams".into(),
                resolution: (1440, 900).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.015, 0.025, 0.04)))
        .insert_resource(Demo {
            meshes: std::array::from_fn(|_| Handle::default()),
            radius_m: SolarSystem::default().body(BodyId::MARS).radius_m,
            stage: 0,
            captured: false,
            output,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, cycle)
        .run();
}

fn case(stage: usize) -> (PatchKey, PatchEdge, &'static str) {
    let last = (1 << 15) - 1;
    let middle = last / 2;
    let (face, x, y, edge, title) = match stage {
        0 | 1 => (
            CubeFace::PositiveX,
            middle,
            middle,
            PatchEdge::PositiveU,
            "Same face",
        ),
        2 => (
            CubeFace::PositiveX,
            last,
            middle,
            PatchEdge::PositiveU,
            "+X / -Z face edge",
        ),
        3 => (
            CubeFace::PositiveX,
            middle,
            0,
            PatchEdge::NegativeV,
            "+X / -Y reversed edge",
        ),
        _ => (
            CubeFace::NegativeY,
            last,
            last,
            PatchEdge::PositiveU,
            "Cube corner: -Y / +X reversed edge",
        ),
    };
    (
        PatchKey {
            face,
            level: 15,
            x,
            y,
        },
        edge,
        title,
    )
}

/// Le corps reste au rayon SI en f64 ; seule cette couche soustrait l'origine
/// et aligne le repère de visualisation avant conversion bornée vers f32.
fn presentation(stage: usize, radius_m: f64) -> ([Mesh; 4], Transform, String) {
    let (key, edge, title) = case(stage);
    let other = if stage == 3 {
        PatchEdge::PositiveU
    } else {
        PatchEdge::PositiveV
    };
    let keys = [
        key,
        key.neighbor(edge).unwrap().key,
        key.neighbor(other).unwrap().key,
    ];
    let details = if stage == 1 || stage == 5 {
        [8, 32, 16]
    } else {
        [32, 16, 8]
    };
    let patches = keys
        .into_iter()
        .zip(details)
        .map(|(k, s)| SurfacePatch::with_segments(k, radius_m, s))
        .collect::<Vec<_>>();
    let origin = key.center_direction().unwrap() * radius_m;
    let rotation = DQuat::from_rotation_arc(origin.normalize(), DVec3::Y);
    let mut all_positions = Vec::new();
    let mut lines = Vec::new();
    let mut line_normals = Vec::new();
    let mut line_colors = Vec::new();
    let mut meshes = Vec::new();
    for patch in &patches {
        let positions: Vec<_> = patch
            .positions_m
            .iter()
            .map(|p| {
                local_to_render(rotation * (*p - origin))
                    .unwrap()
                    .to_array()
            })
            .collect();
        let normals: Vec<_> = patch
            .positions_m
            .iter()
            .map(|p| (rotation * p.normalize()).as_vec3().to_array())
            .collect();
        all_positions.extend(positions.iter().map(|p| Vec3::from_array(*p)));
        meshes.push(
            Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::RENDER_WORLD,
            )
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_inserted_indices(Indices::U32(patch.indices.clone())),
        );
        let mut edges = std::collections::BTreeSet::new();
        for ids in patch.indices.as_chunks::<3>().0 {
            for (a, b) in [(ids[0], ids[1]), (ids[1], ids[2]), (ids[2], ids[0])] {
                edges.insert((a.min(b), a.max(b)));
            }
        }
        for (a, b) in edges {
            for id in [a, b] {
                let p = patch.positions_m[id as usize];
                lines.push(
                    local_to_render(rotation * (p - origin + p.normalize() * 0.05))
                        .unwrap()
                        .to_array(),
                );
                line_normals.push((rotation * p.normalize()).as_vec3().to_array());
                line_colors.push([0.08, 0.1, 0.12, 1.0]);
            }
        }
    }
    // Raccords communs : seulement une ligne relevée pour l'inspection,
    // aucune jupe ni modification de la surface physique.
    for edge in [edge, other] {
        for k in 0..32 {
            for t in [k, k + 1] {
                let (i, j) = edge.sample(t, 32).unwrap();
                let p = key.vertex(i, j, 32, radius_m).unwrap();
                lines.push(
                    local_to_render(rotation * (p - origin + p.normalize() * 0.1))
                        .unwrap()
                        .to_array(),
                );
                line_normals.push((rotation * p.normalize()).as_vec3().to_array());
                line_colors.push([1.0, 0.95, 0.2, 1.0]);
            }
        }
    }
    meshes.push(
        Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, lines)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, line_normals)
            .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, line_colors),
    );
    let (minimum, maximum) = all_positions.into_iter().fold(
        (Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY)),
        |(lo, hi), p| (lo.min(p), hi.max(p)),
    );
    let center = (minimum + maximum) * 0.5;
    let span = (maximum - minimum).max_element();
    let camera =
        Transform::from_translation(center + Vec3::new(span * 0.15, span * 1.15, span * 0.95))
            .looking_at(center, Vec3::Y);
    let counts: Vec<_> = patches
        .iter()
        .map(|p| (p.positions_m.len(), p.indices.len() / 3))
        .collect();
    let text = format!(
        "LOD - {title} - scene {}/6\nOrange / blue / green interiors: {:?} cells per side\nAll boundaries: 32 segments, shared f64 positions, no skirts\n(vertices, triangles): {:?}\nResolution transitions in scenes 2 and 6; geometry diagnostic, no player",
        stage + 1,
        details,
        counts
    );
    info!("{text}; keys {keys:?}");
    (meshes.try_into().unwrap(), camera, text)
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut demo: ResMut<Demo>,
) {
    let (geometry, camera, text) = presentation(0, demo.radius_m);
    for (i, (mesh, color)) in geometry
        .into_iter()
        .zip([
            Color::srgb(0.72, 0.3, 0.12),
            Color::srgb(0.12, 0.46, 0.72),
            Color::srgb(0.2, 0.7, 0.35),
            Color::WHITE,
        ])
        .enumerate()
    {
        let handle = meshes.add(mesh);
        demo.meshes[i] = handle.clone();
        commands.spawn((
            Mesh3d(handle),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                unlit: true,
                ..default()
            })),
        ));
    }
    commands.spawn((
        Camera3d::default(),
        camera,
        Projection::Perspective(PerspectiveProjection {
            near: 1.0,
            far: 5000.0,
            ..default()
        }),
        Msaa::Sample4,
    ));
    commands.spawn((
        Label,
        Text::new(text),
        TextFont {
            font_size: FontSize::Px(22.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            left: px(24),
            top: px(20),
            ..default()
        },
    ));
}

fn cycle(
    time: Res<Time<Real>>,
    mut demo: ResMut<Demo>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut camera: Query<&mut Transform, With<Camera3d>>,
    mut label: Query<&mut Text, With<Label>>,
    mut commands: Commands,
    mut exit: MessageWriter<AppExit>,
) {
    let elapsed = time.elapsed_secs_f64();
    if elapsed >= STAGE_SECONDS * 6.0 {
        exit.write(AppExit::Success);
        return;
    }
    let stage = (elapsed / STAGE_SECONDS) as usize;
    if stage != demo.stage {
        let (geometry, transform, text) = presentation(stage, demo.radius_m);
        for (handle, mesh) in demo.meshes.iter().zip(geometry) {
            *meshes.get_mut(handle).unwrap() = mesh;
        }
        *camera.single_mut().unwrap() = transform;
        label.single_mut().unwrap().0 = text;
        demo.stage = stage;
        demo.captured = false;
        assert!(demo.meshes.iter().all(|handle| meshes.contains(handle)));
        info!(
            "Demo keeps 4 mesh handles; {} total assets including Bevy internal meshes",
            meshes.len()
        );
    }
    if !demo.captured && elapsed % STAGE_SECONDS >= 4.0 {
        if let Some(path) = &demo.output {
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(path.join(format!("seam-{stage:02}.png"))));
        }
        demo.captured = true;
    }
}
