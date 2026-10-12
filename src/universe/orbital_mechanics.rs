use super::units::{AU, DAY, J2000, JULIAN_CENTURY};
use bevy::math::DVec3;

/// Table 1 JPL: a (UA), e, I/L/longitude du périhélie/nœud (degrés), taux/siècle.
#[derive(Clone, Copy, Debug)]
pub struct OrbitalElements {
    pub epoch: [f64; 6],
    pub rates: [f64; 6],
}
impl OrbitalElements {
    pub fn position(self, jd: f64) -> DVec3 {
        let centuries = (jd - J2000) * DAY / JULIAN_CENTURY;
        let v: [f64; 6] = std::array::from_fn(|i| self.epoch[i] + self.rates[i] * centuries);
        kepler_position(
            v[0] * AU,
            v[1],
            v[2].to_radians(),
            v[5].to_radians(),
            (v[4] - v[5]).to_radians(),
            (v[3] - v[4]).to_radians(),
        )
    }
    pub fn period_seconds(self) -> f64 {
        360.0 / (self.rates[3] - self.rates[4]).abs() * JULIAN_CENTURY
    }
}

/// Newton-Raphson en radians, e < 1. Les éléments du catalogue sont elliptiques.
pub fn eccentric_anomaly(mean: f64, eccentricity: f64) -> f64 {
    let mean =
        (mean + std::f64::consts::PI).rem_euclid(std::f64::consts::TAU) - std::f64::consts::PI;
    let mut e = if eccentricity < 0.8 {
        mean
    } else {
        std::f64::consts::PI.copysign(mean)
    };
    for _ in 0..32 {
        let step = (e - eccentricity * e.sin() - mean) / (1.0 - eccentricity * e.cos());
        e -= step;
        if step.abs() < 1e-13 {
            break;
        }
    }
    e
}

/// Coordonnées écliptiques J2000 transformées vers Bevy (x, z_ecl, -y_ecl), repère direct Y-up.
pub fn kepler_position(a: f64, e: f64, i: f64, node: f64, peri: f64, mean: f64) -> DVec3 {
    let anomaly = eccentric_anomaly(mean, e);
    let x = a * (anomaly.cos() - e);
    let y = a * (1.0 - e * e).sqrt() * anomaly.sin();
    let (sw, cw) = peri.sin_cos();
    let (sn, cn) = node.sin_cos();
    let (si, ci) = i.sin_cos();
    let xe = (cw * cn - sw * sn * ci) * x + (-sw * cn - cw * sn * ci) * y;
    let ye = (cw * sn + sw * cn * ci) * x + (-sw * sn + cw * cn * ci) * y;
    let ze = sw * si * x + cw * si * y;
    DVec3::new(xe, ze, -ye)
}
pub fn kepler_period(a: f64, mu: f64) -> f64 {
    std::f64::consts::TAU * (a.powi(3) / mu).sqrt()
}

/// Orbite lunaire moyenne, orientation et phase conventionnelles à J2000 (pas une éphéméride).
pub fn moon_relative(jd: f64) -> DVec3 {
    kepler_position(
        384_400_000.0,
        0.0549,
        5.145_f64.to_radians(),
        0.0,
        0.0,
        std::f64::consts::TAU * (jd - J2000) / 27.3217,
    )
}
