//! Diagnostic graphique indépendant du jeu : deux patches résidents, quatre cas.
//! cargo build --release --locked --example cube_seam
//! target/release/examples/cube_seam [nouveau-dossier-captures]
use bevy::{
    app::AppExit,
    asset::RenderAssetUsages,
    math::{DQuat, DVec3},
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};
use solar_survival::{
    planets::{CubeFace, PatchEdge, PatchKey, PatchPair},
    universe::{catalog::SolarSystem, celestial_body::BodyId, coordinates::local_to_render},
};
use std::path::PathBuf;

const SEGMENTS: u32 = 32;
const STAGE_SECONDS: f64 = 8.0;

#[derive(Resource)]
struct Demo {
    meshes: [Handle<Mesh>; 3],
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
                title: "Solar Survival - P3.1 cube-sphere seam".into(),
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
        0 => (
            CubeFace::PositiveX,
            middle,
            middle,
            PatchEdge::PositiveU,
            "Same face",
        ),
        1 => (
            CubeFace::PositiveX,
            last,
            middle,
            PatchEdge::PositiveU,
            "+X / -Z face edge",
        ),
        2 => (
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
fn presentation(stage: usize, radius_m: f64) -> ([Mesh; 3], Transform, String) {
    let (key, edge, title) = case(stage);
    let pair = PatchPair::new(key, edge, SEGMENTS, radius_m).unwrap();
    let origin = pair.positions_m[pair.seam[SEGMENTS as usize / 2] as usize];
    let rotation = DQuat::from_rotation_arc(origin.normalize(), DVec3::Y);
    let positions: Vec<[f32; 3]> = pair
        .positions_m
        .iter()
        .map(|p| {
            local_to_render(rotation * (*p - origin))
                .unwrap()
                .to_array()
        })
        .collect();
    let normals: Vec<[f32; 3]> = pair
        .positions_m
        .iter()
        .map(|p| (rotation * p.normalize()).as_vec3().to_array())
        .collect();
    let triangle_mesh = |indices: &[u32]| {
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone())
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals.clone())
        .with_inserted_indices(Indices::U32(indices.to_vec()))
    };
    let mut line_positions = Vec::new();
    let mut line_normals = Vec::new();
    let mut line_colors = Vec::new();
    let mut add_line = |a: u32, b: u32, color: [f32; 4], lift_m: f64| {
        for index in [a, b] {
            let p = pair.positions_m[index as usize];
            line_positions.push(
                local_to_render(rotation * (p - origin + p.normalize() * lift_m))
                    .unwrap()
                    .to_array(),
            );
            line_normals.push(normals[index as usize]);
            line_colors.push(color);
        }
    };
    // Grille et diagonales ; aucune autre surface générée.
    for indices in &pair.indices {
        for triangle in indices.as_chunks::<3>().0 {
            for (a, b) in [
                (triangle[0], triangle[1]),
                (triangle[1], triangle[2]),
                (triangle[2], triangle[0]),
            ] {
                add_line(a, b, [0.12, 0.16, 0.2, 1.0], 0.05);
            }
        }
    }
    for seam in pair.seam.windows(2) {
        add_line(seam[0], seam[1], [1.0, 0.9, 0.1, 1.0], 0.1);
    }
    let lines = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, line_positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, line_normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, line_colors);
    let points = positions.iter().map(|p| Vec3::from_array(*p));
    let (minimum, maximum) = points.fold(
        (Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY)),
        |(lo, hi), p| (lo.min(p), hi.max(p)),
    );
    let center = (minimum + maximum) * 0.5;
    let span = (maximum - minimum).max_element();
    let camera =
        Transform::from_translation(center + Vec3::new(span * 0.2, span * 1.15, span * 1.15))
            .looking_at(center, Vec3::Y);
    let seam_length: f64 = pair
        .seam
        .windows(2)
        .map(|s| pair.positions_m[s[0] as usize].distance(pair.positions_m[s[1] as usize]))
        .sum();
    let text = format!(
        "P3.1 - {title}\nOrange: source / blue: neighbor / yellow: shared edge\nLevel 15 - 32 segments - radius {radius_m:.0} m - seam {seam_length:.3} m\n{} shared vertices / {} unique vertices / 4096 triangles\n{:?} -> {:?}, reversed={}\nGeometry only - no player contacts - scene {}/4",
        pair.seam.len(),
        pair.positions_m.len(),
        edge,
        pair.neighbor.edge,
        pair.neighbor.reversed,
        stage + 1
    );
    info!(
        "{text}; source={key:?}; neighbor={:?}; 2 patches, 3 reused mesh handles",
        pair.neighbor.key
    );
    (
        [
            triangle_mesh(&pair.indices[0]),
            triangle_mesh(&pair.indices[1]),
            lines,
        ],
        camera,
        text,
    )
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
    if elapsed >= STAGE_SECONDS * 4.0 {
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
            "Demo keeps 3 mesh handles; {} total assets including Bevy internal meshes",
            meshes.len()
        );
    }
    if !demo.captured && elapsed % STAGE_SECONDS >= 5.0 {
        if let Some(path) = &demo.output {
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(path.join(format!("seam-{stage:02}.png"))));
        }
        demo.captured = true;
    }
}
