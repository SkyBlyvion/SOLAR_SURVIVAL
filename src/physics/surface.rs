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
pub(crate) const SKIN_M: f64 = 0.002;

#[derive(Clone, Copy, Debug)]
pub struct SolidBox {
    pub min: DVec3,
    pub max: DVec3,
}
/// Boîte autoritaire fixe dans le repère du corps, en mètres/f64.
#[derive(Clone, Copy, Debug)]
pub struct BodyBox {
    pub center: DVec3,
    pub size: DVec3,
    pub orientation: DQuat,
}
impl BodyBox {
    pub fn chart_pose(&self, terrain: &MarsTerrain) -> (DVec3, DVec3, DQuat) {
        (
            terrain.chart(self.center),
            self.size,
            terrain.orientation.inverse() * self.orientation,
        )
    }
}
fn oriented_poses(terrain: &MarsTerrain) -> impl Iterator<Item = (DVec3, DVec3, DQuat)> + '_ {
    (0..3).map(|i| obstacle_pose(terrain, i)).chain(
        terrain
            .building_solids
            .iter()
            .map(|b| b.chart_pose(terrain)),
    )
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
            // P3.3/P3.4 : expansion bornée/testée pour les deux cartes seulement.
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
    if terrain.globe.is_some() || !terrain.building_solids.is_empty() {
        for (center, size, rotation) in oriented_poses(terrain) {
            if center.distance(feet) > size.length() * 0.5 + PLAYER_HEIGHT_M + 2.0 * HALF_WIDTH_M {
                continue;
            }
            // Faces supérieures de la boîte exacte, découpées sous l'empreinte.
            for axis in 0..3 {
                for sign in [-1.0, 1.0] {
                    let mut normal = DVec3::ZERO;
                    normal[axis] = sign;
                    if (rotation * normal).y <= 1e-10 {
                        continue;
                    }
                    let a = (axis + 1) % 3;
                    let b = (axis + 2) % 3;
                    let points =
                        [(-0.5, -0.5), (0.5, -0.5), (-0.5, 0.5), (0.5, 0.5)].map(|(x, z)| {
                            let mut corner = DVec3::ZERO;
                            corner[axis] = sign * 0.5;
                            corner[a] = x;
                            corner[b] = z;
                            center + rotation * (corner * size)
                        });
                    for ids in [[0, 1, 2], [1, 3, 2]] {
                        if let Some(h) = crate::planets::terrain::triangle_support(
                            ids.map(|i| points[i]),
                            feet.x - HALF_WIDTH_M,
                            feet.x + HALF_WIDTH_M,
                            feet.z - HALF_WIDTH_M,
                            feet.z + HALF_WIDTH_M,
                        ) && feet.y >= h - SKIN_M
                        {
                            floor = floor.max(h);
                        }
                    }
                }
            }
        }
        return floor;
    }
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

/// Balayage continu SAT : boîte joueur alignée sur la carte contre OBB exacte.
/// Les 3+3+9 axes de séparation évitent l'expansion cumulée d'une AABB.
fn sweep_oriented(
    start: DVec3,
    delta: DVec3,
    center: DVec3,
    size: DVec3,
    rotation: DQuat,
) -> Option<(f64, DVec3)> {
    let player_center = start + DVec3::Y * (PLAYER_HEIGHT_M * 0.5);
    let player_half = DVec3::new(HALF_WIDTH_M, PLAYER_HEIGHT_M * 0.5, HALF_WIDTH_M);
    let basis = [DVec3::X, DVec3::Y, DVec3::Z];
    let object = basis.map(|v| rotation * v);
    let mut axes = [DVec3::ZERO; 15];
    axes[..3].copy_from_slice(&basis);
    axes[3..6].copy_from_slice(&object);
    for i in 0..3 {
        for j in 0..3 {
            axes[6 + i * 3 + j] = basis[i].cross(object[j]);
        }
    }
    let mut entry: f64 = f64::NEG_INFINITY;
    let mut exit: f64 = f64::INFINITY;
    let mut hit_normal = DVec3::ZERO;
    for axis in axes {
        if axis.length_squared() < 1e-20 {
            continue;
        }
        let axis = axis.normalize();
        let radius = axis.abs().dot(player_half)
            + object
                .into_iter()
                .enumerate()
                .map(|(i, v)| axis.dot(v).abs() * size[i] * 0.5)
                .sum::<f64>();
        let distance = (player_center - center).dot(axis);
        let speed = delta.dot(axis);
        if speed.abs() < 1e-12 {
            if distance.abs() >= radius {
                return None;
            }
        } else {
            let a = (-radius - distance) / speed;
            let b = (radius - distance) / speed;
            if a.min(b) > entry {
                entry = a.min(b);
                hit_normal = axis * -speed.signum();
            }
            exit = exit.min(a.max(b));
        }
    }
    (entry <= exit && (0.0..=1.0).contains(&entry)).then_some((entry, hit_normal))
}
fn move_on_globe(
    terrain: &MarsTerrain,
    mut feet: DVec3,
    mut delta: DVec3,
    mut velocity: DVec3,
) -> (DVec3, DVec3, bool) {
    let mut grounded = false;
    for _ in 0..4 {
        let hit = oriented_poses(terrain)
            .filter_map(|(center, size, rotation)| {
                // Ignore les objets éloignés ; pas de coordonnées astronomiques en f32.
                if center.distance(feet)
                    > size.length() * 0.5 + PLAYER_HEIGHT_M + 2.0 * HALF_WIDTH_M + delta.length()
                {
                    return None;
                }
                sweep_oriented(feet, delta, center, size, rotation)
            })
            .min_by(|a, b| a.0.total_cmp(&b.0));
        let Some((fraction, normal)) = hit else {
            feet += delta;
            break;
        };
        let safe = (fraction - SKIN_M / delta.length().max(SKIN_M)).max(0.0);
        feet += delta * safe;
        delta *= 1.0 - safe;
        delta -= normal * delta.dot(normal).min(0.0);
        velocity -= normal * velocity.dot(normal).min(0.0);
        grounded |= normal.y > 0.5;
    }
    (feet, velocity, grounded)
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
    let (next, new_velocity, on_box) =
        if terrain.globe.is_some() || !terrain.building_solids.is_empty() {
            move_on_globe(terrain, *feet, *velocity * STEP_SECONDS, *velocity)
        } else {
            move_and_slide(
                *feet,
                *velocity * STEP_SECONDS,
                *velocity,
                &obstacles(terrain),
            )
        };
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

#[cfg(test)]
mod oriented_tests {
    use super::*;
    #[test]
    fn continuous_obb_sweep_matches_axis_aligned_boxes_and_rejects_aabb_false_contact() {
        let center = DVec3::new(0.0, 1.0, 0.0);
        let size = DVec3::new(4.0, 2.0, 0.5);
        let solid = SolidBox {
            min: center - size * 0.5,
            max: center + size * 0.5,
        };
        for start in [
            DVec3::new(-8.0, 0.0, 0.0),
            DVec3::new(0.0, 5.0, 0.0),
            DVec3::new(0.0, 0.0, 5.0),
        ] {
            let delta = (DVec3::ZERO - start) * 2.0;
            let a = sweep(start, delta, solid).unwrap();
            let b = sweep_oriented(start, delta, center, size, DQuat::IDENTITY).unwrap();
            assert!((a.0 - b.0).abs() < 1e-12);
            assert!(a.1.distance(b.1) < 1e-12);
        }
        let q = DQuat::from_rotation_y(std::f64::consts::FRAC_PI_4);
        assert!(
            sweep_oriented(
                DVec3::new(1.6, 0.0, 1.6),
                DVec3::new(-0.1, 0.0, -0.1),
                center,
                size,
                q
            )
            .is_none()
        );
        let hit = sweep_oriented(
            DVec3::new(4.0, 0.0, 4.0),
            DVec3::new(-8.0, 0.0, -8.0),
            center,
            size,
            q,
        )
        .unwrap();
        assert!((0.3..0.5).contains(&hit.0));
        assert!(hit.1.y.abs() < 1e-12 && hit.1.x > 0.0 && hit.1.z > 0.0);
    }
}
