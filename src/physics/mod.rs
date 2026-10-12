//! Garde de navigation sphérique, sans moteur physique ni collision de terrain.
use bevy::math::DVec3;

/// Intersection balayée: ne pas traverser une planète même à une vitesse fictive élevée.
pub fn stop_before_sphere(start: DVec3, delta: DVec3, center: DVec3, radius: f64) -> Option<f64> {
    let length = delta.length();
    if length < 1e-12 {
        return None;
    }
    let direction = delta / length;
    let relative = start - center;
    let along = -relative.dot(direction);
    let closest = relative + direction * along;
    let perpendicular_squared = closest.length_squared();
    if perpendicular_squared >= radius * radius {
        return None;
    }
    let entry = along - (radius * radius - perpendicular_squared).sqrt();
    (entry >= 0.0 && entry <= length).then(|| (entry / length - 1e-10).max(0.0))
}

/// Gravité radiale préparée pour Mars; accélération SI, jamais axe Y global.
pub fn radial_gravity(relative: DVec3, mu: f64) -> DVec3 {
    let r = relative.length();
    if r <= 1e-6 {
        DVec3::ZERO
    } else {
        -relative * (mu / r.powi(3))
    }
}
