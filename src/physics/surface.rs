//! Contrôleur cinématique borné : boîte joueur contre relief doux et boîtes statiques.
use super::radial_gravity;
use crate::planets::terrain::MarsTerrain;
use bevy::math::{DQuat, DVec3};

pub const HALF_WIDTH_M: f64 = 0.3;
pub const PLAYER_HEIGHT_M: f64 = 1.8;
pub const EYE_HEIGHT_M: f64 = 1.65;
pub const WALK_MPS: f64 = 4.5;
pub const RUN_MPS: f64 = 8.0;
pub const JUMP_MPS: f64 = 4.0;
pub const STEP_SECONDS: f64 = 1.0 / 120.0;
const SKIN_M: f64 = 0.002;

#[derive(Clone, Copy, Debug)]
pub struct SolidBox {
    pub min: DVec3,
    pub max: DVec3,
}
/// Pose exacte du proxy ; boîte de contact conservatrice dans la carte active.
pub fn obstacle_pose(terrain: &MarsTerrain, index: usize) -> (DVec3, DVec3, DQuat) {
    if let Some((frame, solids)) = &terrain.site_obstacles {
        let solid = solids[index];
        let center = (*frame * ((solid.min + solid.max) * 0.5 + DVec3::Y * terrain.radius_m))
            - terrain.orientation * (DVec3::Y * terrain.radius_m);
        return (
            terrain.orientation.inverse() * center,
            solid.max - solid.min,
            terrain.orientation.inverse() * *frame,
        );
    }
    let solid = obstacles(terrain)[index];
    (
        (solid.min + solid.max) * 0.5,
        solid.max - solid.min,
        DQuat::IDENTITY,
    )
}
pub fn obstacles(terrain: &MarsTerrain) -> [SolidBox; 3] {
    if terrain.site_obstacles.is_some() {
        return std::array::from_fn(|index| {
            let (center, size, rotation) = obstacle_pose(terrain, index);
            // Enveloppe des huit coins, sans déplacer/dupliquer l'obstacle.
            // Inclinaison <= 0,000062 rad dans ce parcours uniquement.
            let half = (rotation * DVec3::X).abs() * size.x * 0.5
                + (rotation * DVec3::Y).abs() * size.y * 0.5
                + (rotation * DVec3::Z).abs() * size.z * 0.5;
            SolidBox {
                min: center - half,
                max: center + half,
            }
        });
    }
    // Repères de taille humaine, conventions de gameplay, pas de roches mesurées.
    [
        (0.0, -12.0, 4.0, 2.0, 2.0),
        (8.0, -22.0, 3.0, 3.0, 3.0),
        (-9.0, -30.0, 5.0, 1.5, 3.0),
    ]
    .map(|(x, z, w, h, d)| {
        let floor = terrain.height(x, z) - 0.5;
        SolidBox {
            min: DVec3::new(x - w / 2.0, floor, z - d / 2.0),
            max: DVec3::new(x + w / 2.0, floor + h, z + d / 2.0),
        }
    })
}

/// Plan de support sous les pieds, y compris le dessus d'un obstacle statique.
pub fn support_height(terrain: &MarsTerrain, feet: DVec3) -> f64 {
    let mut floor = terrain.support_height(feet, HALF_WIDTH_M);
    for solid in obstacles(terrain) {
        let overlaps = feet.x + HALF_WIDTH_M > solid.min.x
            && feet.x - HALF_WIDTH_M < solid.max.x
            && feet.z + HALF_WIDTH_M > solid.min.z
            && feet.z - HALF_WIDTH_M < solid.max.z;
        if overlaps && feet.y >= solid.max.y - SKIN_M {
            floor = floor.max(solid.max.y);
        }
    }
    floor
}

/// Balayage de la boîte joueur par expansion de Minkowski des obstacles.
fn sweep(start: DVec3, delta: DVec3, solid: SolidBox) -> Option<(f64, DVec3)> {
    let min = solid.min - DVec3::new(HALF_WIDTH_M, PLAYER_HEIGHT_M, HALF_WIDTH_M);
    let max = solid.max + DVec3::new(HALF_WIDTH_M, 0.0, HALF_WIDTH_M);
    let mut entry: f64 = f64::NEG_INFINITY;
    let mut exit: f64 = f64::INFINITY;
    let mut normal = DVec3::ZERO;
    for axis in 0..3 {
        if delta[axis].abs() < 1e-12 {
            if start[axis] <= min[axis] || start[axis] >= max[axis] {
                return None;
            }
        } else {
            let a = (min[axis] - start[axis]) / delta[axis];
            let b = (max[axis] - start[axis]) / delta[axis];
            if a.min(b) > entry {
                entry = a.min(b);
                normal = DVec3::ZERO;
                normal[axis] = -delta[axis].signum();
            }
            exit = exit.min(a.max(b));
        }
    }
    (entry <= exit && (0.0..=1.0).contains(&entry)).then_some((entry, normal))
}
pub fn move_and_slide(
    mut feet: DVec3,
    mut delta: DVec3,
    mut velocity: DVec3,
    solids: &[SolidBox],
) -> (DVec3, DVec3, bool) {
    let mut on_box = false;
    for _ in 0..4 {
        let hit = solids
            .iter()
            .filter_map(|s| sweep(feet, delta, *s))
            .min_by(|a, b| a.0.total_cmp(&b.0));
        let Some((fraction, normal)) = hit else {
            feet += delta;
            break;
        };
        let safe_fraction = (fraction - SKIN_M / delta.length().max(SKIN_M)).max(0.0);
        feet += delta * safe_fraction;
        delta *= 1.0 - safe_fraction;
        delta -= normal * delta.dot(normal).min(0.0);
        velocity -= normal * velocity.dot(normal).min(0.0);
        on_box |= normal.y > 0.5;
    }
    (feet, velocity, on_box)
}

pub fn step(
    terrain: &MarsTerrain,
    feet: &mut DVec3,
    velocity: &mut DVec3,
    grounded: &mut bool,
    desired: DVec3,
    jump: bool,
    mu: f64,
) {
    let relative = terrain.site_relative(*feet);
    let local_up = relative.normalize();
    // Commande tangentielle ; on conserve la vitesse verticale pour la chute/le saut.
    let vertical = velocity.dot(local_up);
    *velocity = desired - local_up * desired.dot(local_up) + local_up * vertical;
    if jump && *grounded {
        *velocity += local_up * JUMP_MPS;
        *grounded = false;
    }
    // Gravité centrale dans les axes du site : pas de forces d'inertie du
    // repère tournant. Le calendrier orbital n'entre pas dans ce contrôleur.
    *velocity += radial_gravity(relative, mu) * STEP_SECONDS;
    let was_grounded = *grounded;
    // Le contrôleur impose l'adhérence sur ces pentes douces : la composante
    // horizontale de la pesanteur ne doit pas faire dériver un joueur au repos.
    let tangent = desired - local_up * desired.dot(local_up);
    if was_grounded {
        velocity.x = tangent.x;
        velocity.z = tangent.z;
    }
    let old = *feet;
    let (next, new_velocity, on_box) = move_and_slide(
        *feet,
        *velocity * STEP_SECONDS,
        *velocity,
        &obstacles(terrain),
    );
    *feet = next;
    *velocity = new_velocity;
    let unbounded = *feet;
    terrain.clamp_feet(feet, HALF_WIDTH_M);
    // Le bornage fait partie de la résolution : une direction bloquée ne
    // doit pas rester dans la vitesse publiée (paire et site historique).
    if feet.x != unbounded.x {
        velocity.x = 0.0;
    }
    if feet.z != unbounded.z {
        velocity.z = 0.0;
    }
    let floor = support_height(terrain, *feet) + SKIN_M;
    let snap =
        was_grounded && !jump && old.y - floor >= -0.1 && old.y - floor <= 0.1 && velocity.y <= 0.1;
    *grounded = on_box;
    if feet.y <= floor || snap {
        feet.y = floor;
        velocity.y = velocity.y.max(0.0);
        *grounded = true;
        if desired.length_squared() < 1e-12 {
            velocity.x = 0.0;
            velocity.z = 0.0;
        }
    }
}
