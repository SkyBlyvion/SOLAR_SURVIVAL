//! Relief artistique mondial : bruit de valeur 3D en mètres corporels, indépendant des faces.
//! Aucune cartographie martienne ni donnée d'altimétrie mesurée.
use bevy::math::DVec3;

pub const MAX_HEIGHT_M: f64 = 85.3;

fn hash(cell: [i64; 3], seed: u64) -> f64 {
    let mut h = seed;
    for (v, salt) in
        cell.into_iter()
            .zip([0x9e3779b97f4a7c15, 0xbf58476d1ce4e5b9, 0x94d049bb133111eb])
    {
        h ^= (v as u64).wrapping_mul(salt);
        h = (h ^ (h >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        h ^= h >> 27;
    }
    (h >> 11) as f64 / (1_u64 << 53) as f64 * 2.0 - 1.0
}
fn noise(point: DVec3, seed: u64) -> f64 {
    let cell = point.floor();
    let f = point - cell;
    let t = f * f * f * (f * (f * 6.0 - DVec3::splat(15.0)) + DVec3::splat(10.0));
    let mut value = 0.0;
    for x in 0..=1 {
        for y in 0..=1 {
            for z in 0..=1 {
                let weight = |v: u32, t: f64| if v == 0 { 1.0 - t } else { t };
                value += hash(
                    [
                        cell.x as i64 + x as i64,
                        cell.y as i64 + y as i64,
                        cell.z as i64 + z as i64,
                    ],
                    seed,
                ) * weight(x, t.x)
                    * weight(y, t.y)
                    * weight(z, t.z);
            }
        }
    }
    value
}
/// Quatre échelles artistiques, longueurs/amplitudes en mètres. |h| <=85,3 m.
pub fn height(direction: DVec3, radius_m: f64, seed: u64) -> f64 {
    let p = direction * radius_m;
    [(3600.0, 70.0), (900.0, 14.0), (240.0, 1.2), (60.0, 0.1)]
        .into_iter()
        .enumerate()
        .map(|(i, (scale, amplitude))| {
            amplitude * noise(p / scale, seed.wrapping_add(i as u64 * 7919))
        })
        .sum()
}
pub fn displace(point: DVec3, radius_m: f64, seed: u64) -> DVec3 {
    point * ((radius_m + height(point / radius_m, radius_m, seed)) / radius_m)
}
/// Normale commune aux patches/LOD : dérivées centrales métriques du même champ.
pub fn normal(point: DVec3, radius_m: f64, seed: u64) -> DVec3 {
    let up = point.normalize();
    let east = up
        .cross(if up.y.abs() < 0.9 { DVec3::Y } else { DVec3::X })
        .normalize();
    let north = up.cross(east);
    let sample = |offset: DVec3| {
        let d = (up * radius_m + offset).normalize();
        d * (radius_m + height(d, radius_m, seed))
    };
    (sample(east) - sample(-east))
        .cross(sample(north) - sample(-north))
        .normalize()
}
