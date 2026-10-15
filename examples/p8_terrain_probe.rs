//! Diagnostic CPU indépendant du GPU. Comparaison sphère historique / relief+couronne.
use bevy::math::{DQuat, DVec3};
use solar_survival::planets::globe_terrain::GlobeTerrain;
use std::time::Instant;
fn main() {
    let radius = 3_389_500.0;
    for relief in [false, true] {
        let mut samples = Vec::new();
        let mut geometry = 0;
        let mut landscape = (0, 0);
        let mut g = if relief {
            GlobeTerrain::with_relief_lod(DVec3::Y, DQuat::IDENTITY, radius, 42, true)
        } else {
            GlobeTerrain::with_lod(DVec3::Y, DQuat::IDENTITY, radius, true)
        };
        for i in 1..=200 {
            let up = (DVec3::Y * radius + DVec3::X * (i as f64 * 64.1)).normalize();
            let q = DQuat::from_rotation_arc(DVec3::Y, up);
            let start = Instant::now();
            g = g.rebased(up, q, radius);
            samples.push(start.elapsed().as_secs_f64() * 1000.0);
            geometry = geometry.max(g.detail_counts().2);
            if let Some(p) = &g.landscape {
                landscape.0 = landscape.0.max(p.positions_m.len());
                landscape.1 = landscape.1.max(p.indices.len() / 3);
            }
        }
        samples.sort_by(f64::total_cmp);
        println!(
            "relief={relief} rebase_mean_ms={:.4} p95_ms={:.4} max_ms={:.4} local_max_triangles={} landscape_max_vertices={} landscape_max_triangles={}",
            samples.iter().sum::<f64>() / samples.len() as f64,
            samples[189],
            samples[199],
            geometry,
            landscape.0,
            landscape.1
        );
    }
}
