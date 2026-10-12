//! Données embarquées: sources et conventions dans docs/ASTRONOMY.md.
use super::{
    celestial_body::*,
    orbital_mechanics::{OrbitalElements, moon_relative},
    units::*,
};
use crate::planets::GenerationSettings;
use bevy::{math::DVec3, prelude::*};

#[derive(Resource)]
pub struct SolarSystem {
    pub bodies: Vec<CelestialBody>,
    cached_jd: Option<f64>,
    cached_positions: Vec<DVec3>,
}
impl Default for SolarSystem {
    fn default() -> Self {
        let names = [
            "Soleil", "Mercure", "Vénus", "Terre", "Mars", "Jupiter", "Saturne", "Uranus",
            "Neptune", "Lune",
        ];
        let radii_km = [
            695700.0, 2439.4, 6051.8, 6371.0084, 3389.5, 69911.0, 58232.0, 25362.0, 24622.0, 1737.4,
        ];
        let masses = [
            1.9884e30,
            0.330103e24,
            4.86731e24,
            5.97217e24,
            0.641691e24,
            1898.125e24,
            568.317e24,
            86.8099e24,
            102.4092e24,
            0.07346e24,
        ];
        let mu = [
            SUN_MU,
            22031.868551e9,
            324858.592e9,
            398600.435507e9,
            42828.375816e9,
            126712764.1e9,
            37940584.8418e9,
            5794556.4e9,
            6836527.100580e9,
            4902.800118e9,
        ];
        // Signe rétrograde déjà porté par la rotation; inclinaison visuelle 180°-obliquité dans ces cas.
        let rotations_days = [
            25.38, 58.6462, -243.018, 0.99726968, 1.02595676, 0.41354, 0.44401, -0.71833, 0.67125,
            27.3217,
        ];
        let tilts = [
            7.25, 0.034, 177.4, 23.44, 25.19, 3.13, 26.73, 97.77, 28.32, 6.68,
        ];
        let colors = [
            [1.0, 0.65, 0.14],
            [0.50, 0.47, 0.43],
            [0.91, 0.71, 0.39],
            [0.07, 0.32, 0.67],
            [0.72, 0.25, 0.10],
            [0.76, 0.60, 0.43],
            [0.85, 0.74, 0.49],
            [0.37, 0.78, 0.81],
            [0.08, 0.27, 0.77],
            [0.53, 0.52, 0.50],
        ];
        let epochs = [
            [
                0.38709927,
                0.20563593,
                7.00497902,
                252.25032350,
                77.45779628,
                48.33076593,
            ],
            [
                0.72333566,
                0.00677672,
                3.39467605,
                181.97909950,
                131.60246718,
                76.67984255,
            ],
            [
                1.00000261,
                0.01671123,
                -0.00001531,
                100.46457166,
                102.93768193,
                0.0,
            ],
            [
                1.52371034,
                0.09339410,
                1.84969142,
                -4.55343205,
                -23.94362959,
                49.55953891,
            ],
            [
                5.20288700,
                0.04838624,
                1.30439695,
                34.39644051,
                14.72847983,
                100.47390909,
            ],
            [
                9.53667594,
                0.05386179,
                2.48599187,
                49.95424423,
                92.59887831,
                113.66242448,
            ],
            [
                19.18916464,
                0.04725744,
                0.77263783,
                313.23810451,
                170.95427630,
                74.01692503,
            ],
            [
                30.06992276,
                0.00859048,
                1.77004347,
                -55.12002969,
                44.96476227,
                131.78422574,
            ],
        ];
        let rates = [
            [
                0.00000037,
                0.00001906,
                -0.00594749,
                149472.67411175,
                0.16047689,
                -0.12534081,
            ],
            [
                0.00000390,
                -0.00004107,
                -0.00078890,
                58517.81538729,
                0.00268329,
                -0.27769418,
            ],
            [
                0.00000562,
                -0.00004392,
                -0.01294668,
                35999.37244981,
                0.32327364,
                0.0,
            ],
            [
                0.00001847,
                0.00007882,
                -0.00813131,
                19140.30268499,
                0.44441088,
                -0.29257343,
            ],
            [
                -0.00011607,
                -0.00013253,
                -0.00183714,
                3034.74612775,
                0.21252668,
                0.20469106,
            ],
            [
                -0.00125060,
                -0.00050991,
                0.00193609,
                1222.49362201,
                -0.41897216,
                -0.28867794,
            ],
            [
                -0.00196176,
                -0.00004397,
                -0.00242939,
                428.48202785,
                0.40805281,
                0.04240589,
            ],
            [
                0.00026291,
                0.00005105,
                0.00035372,
                218.45945325,
                -0.32241464,
                -0.00508664,
            ],
        ];
        Self {
            bodies: (0..10)
                .map(|n| {
                    let kind = match n {
                        0 => BodyKind::Star,
                        5 | 6 => BodyKind::GasGiant,
                        7 | 8 => BodyKind::IceGiant,
                        9 => BodyKind::Satellite,
                        _ => BodyKind::RockyPlanet,
                    };
                    CelestialBody {
                        id: BodyId(n),
                        name: names[n],
                        kind,
                        radius_m: radii_km[n] * 1000.0,
                        mass_kg: masses[n],
                        gravitational_parameter: mu[n],
                        rotation_period_s: rotations_days[n] * DAY,
                        axial_tilt_deg: tilts[n],
                        orbit: (1..=8).contains(&n).then(|| OrbitalElements {
                            epoch: epochs[n - 1],
                            rates: rates[n - 1],
                        }),
                        parent: match n {
                            0 => None,
                            9 => Some(BodyId::EARTH),
                            _ => Some(BodyId::SUN),
                        },
                        atmosphere: match n {
                            0 => Atmosphere::Stellar,
                            1 | 9 => Atmosphere::Exosphere,
                            2 => Atmosphere::DenseCarbonDioxide,
                            3 => Atmosphere::EarthLike,
                            4 => Atmosphere::ThinCarbonDioxide,
                            _ => Atmosphere::GiantEnvelope,
                        },
                        surface: match kind {
                            BodyKind::Star => SurfaceKind::Plasma,
                            BodyKind::GasGiant => SurfaceKind::Gas,
                            BodyKind::IceGiant => SurfaceKind::IceGiant,
                            _ if n == 3 => SurfaceKind::OceanAndLand,
                            _ => SurfaceKind::Rock,
                        },
                        color: colors[n],
                        generation: GenerationSettings {
                            seed: 0x501A_0000 + n as u64,
                            max_lod: 12,
                        },
                    }
                })
                .collect(),
            cached_jd: None,
            cached_positions: vec![DVec3::ZERO; 10],
        }
    }
}
impl SolarSystem {
    pub fn body(&self, id: BodyId) -> &CelestialBody {
        &self.bodies[id.0]
    }
    /// Un cache partagé évite de résoudre Kepler à chaque requête caméra/UI/physique.
    pub fn cache_positions(&mut self, jd: f64) {
        if self.cached_jd == Some(jd) {
            return;
        }
        self.cached_jd = None;
        self.cached_positions.resize(self.bodies.len(), DVec3::ZERO);
        for i in 0..self.bodies.len() {
            self.cached_positions[i] = self.position(BodyId(i), jd);
        }
        self.cached_jd = Some(jd);
    }
    pub fn position(&self, id: BodyId, jd: f64) -> DVec3 {
        if self.cached_jd == Some(jd) {
            return self.cached_positions[id.0];
        }
        if id == BodyId::SUN {
            return DVec3::ZERO;
        }
        // Table JPL Terre = barycentre Terre-Lune; déplacer la Terre et la Lune autour de lui.
        if id == BodyId::EARTH || id == BodyId::MOON {
            let moon = moon_relative(jd);
            let bary = self.body(BodyId::EARTH).orbit.unwrap().position(jd);
            let fraction = self.body(BodyId::MOON).mass_kg
                / (self.body(BodyId::EARTH).mass_kg + self.body(BodyId::MOON).mass_kg);
            return bary
                + moon
                    * if id == BodyId::EARTH {
                        -fraction
                    } else {
                        1.0 - fraction
                    };
        }
        self.body(id).orbit.unwrap().position(jd)
    }
}
