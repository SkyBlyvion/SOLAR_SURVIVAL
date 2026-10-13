//! Profilage opt-in : scène fixe, variantes isolées et mesures CSV en millisecondes.
//! Aucun système ni diagnostic GPU ajouté en partie normale.
use crate::{
    GameSet,
    camera::{NavigationRequest, SpaceCamera},
    config::GameConfig,
    physics::surface::HALF_WIDTH_M,
    planets::terrain::MarsTerrain,
    player::{Explorer, MovementMode, surface::SurfaceState},
    rendering::terrain::SurfaceCamera,
    universe::{SimulationClock, units::J2000},
};
use bevy::{
    app::AppExit,
    camera::visibility::RenderLayers,
    diagnostic::DiagnosticsStore,
    math::DVec3,
    prelude::*,
    render::{
        RenderApp, diagnostic::RenderDiagnosticsPlugin, renderer::RenderDevice,
        settings::WgpuFeatures,
    },
};
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
    time::Instant,
};

const WARMUP_SECONDS: f64 = 10.0;
const SETTLE_SECONDS: f64 = 1.0;
const VARIANTS: [&str; 6] = [
    "baseline",
    "no_space_camera",
    "no_surface_camera",
    "half_resolution",
    "msaa_off",
    "surface_unlit",
];

#[derive(Debug)]
pub struct ProfileConfig {
    pub seconds: f64,
    pub variant: String,
    pub output: PathBuf,
    pub gpu_diagnostics: bool,
    pub global_terrain: bool,
}
impl ProfileConfig {
    pub fn from_env() -> Option<Self> {
        let seconds = std::env::var("SOLAR_PROFILE_SECONDS").ok()?;
        let variant = std::env::var("SOLAR_PROFILE_VARIANT").unwrap_or_else(|_| "baseline".into());
        let seconds = seconds
            .parse::<f64>()
            .ok()
            .filter(|v| v.is_finite() && (30.0..=300.0).contains(v));
        if seconds.is_none() || !VARIANTS.contains(&variant.as_str()) {
            eprintln!(
                "Profilage désactivé : SOLAR_PROFILE_SECONDS doit être entre 30 et 300, variante parmi {VARIANTS:?}."
            );
            return None;
        }
        Some(Self {
            global_terrain: std::env::var("SOLAR_PROFILE_TERRAIN").as_deref() == Ok("global"),
            gpu_diagnostics: std::env::var("SOLAR_PROFILE_GPU").as_deref() != Ok("0"),
            seconds: seconds.unwrap(),
            output: std::env::var_os("SOLAR_PROFILE_OUTPUT")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(format!("docs/profile-{variant}.csv"))),
            variant,
        })
    }
    fn phase(&self, elapsed: f64) -> (&'static str, f64) {
        let duration = (self.seconds - WARMUP_SECONDS) / 4.0;
        if elapsed < WARMUP_SECONDS {
            return ("warmup", elapsed);
        }
        let local = elapsed - WARMUP_SECONDS;
        let index = (local / duration) as usize;
        (
            ["rest", "contact", "streaming", "space", "finished"][index.min(4)],
            local - index as f64 * duration,
        )
    }
}

#[derive(Resource)]
struct ProfileSamples {
    phase: &'static str,
    cursor: Instant,
    metric: &'static str,
    cpu_frame: Vec<(&'static str, f64)>,
    values: BTreeMap<(&'static str, String), Vec<f64>>,
    seen: BTreeMap<String, bevy::platform::time::Instant>,
}
impl Default for ProfileSamples {
    fn default() -> Self {
        Self {
            phase: "",
            cursor: Instant::now(),
            metric: "",
            cpu_frame: Vec::new(),
            values: BTreeMap::new(),
            seen: BTreeMap::new(),
        }
    }
}
impl ProfileSamples {
    fn mark(&mut self, metric: &'static str) {
        let now = Instant::now();
        if !self.metric.is_empty() {
            self.cpu_frame
                .push((self.metric, (now - self.cursor).as_secs_f64() * 1000.0));
        }
        self.metric = metric;
        self.cursor = now;
    }
    fn add(&mut self, metric: String, value: f64) {
        if value.is_finite() && value >= 0.0 {
            self.values
                .entry((self.phase, metric))
                .or_default()
                .push(value);
        }
    }
    fn write_csv(&self, path: &PathBuf) -> std::io::Result<()> {
        let mut writer = BufWriter::new(File::create(path)?);
        writeln!(writer, "phase,metric,value_ms")?;
        for ((phase, metric), samples) in &self.values {
            for value in samples {
                writeln!(
                    writer,
                    "{phase},\"{}\",{value:.6}",
                    metric.replace('"', "\"\"")
                )?;
            }
        }
        writer.flush()
    }
}

pub struct ProfilingPlugin;
impl Plugin for ProfilingPlugin {
    fn build(&self, app: &mut App) {
        if app.world().resource::<GameConfig>().profile.is_none() {
            return;
        }
        if app
            .world()
            .resource::<GameConfig>()
            .profile
            .as_ref()
            .unwrap()
            .gpu_diagnostics
        {
            app.add_plugins(RenderDiagnosticsPlugin);
        }
        app.init_resource::<ProfileSamples>()
            .add_systems(
                Update,
                prepare.in_set(GameSet::Input).after(crate::ui::HudInputSet),
            )
            .add_systems(
                Update,
                (
                    begin.after(GameSet::Input).before(GameSet::Simulation),
                    navigation
                        .after(GameSet::Simulation)
                        .before(GameSet::Navigation),
                    rendering
                        .after(GameSet::Navigation)
                        .before(GameSet::Rendering),
                    interface
                        .after(GameSet::Rendering)
                        .before(GameSet::Interface),
                    finish.after(GameSet::Interface),
                ),
            )
            .add_systems(PostUpdate, isolate)
            .add_systems(Last, collect);
    }
    fn finish(&self, app: &mut App) {
        if app.world().resource::<GameConfig>().profile.is_none() {
            return;
        }
        if let Some(render_app) = app.get_sub_app(RenderApp)
            && let Some(device) = render_app.world().get_resource::<RenderDevice>()
        {
            let features = device.features();
            info!(
                "Profilage GPU : timestamps {}, encoders {}, passes {}",
                features.contains(WgpuFeatures::TIMESTAMP_QUERY),
                features.contains(WgpuFeatures::TIMESTAMP_QUERY_INSIDE_ENCODERS),
                features.contains(WgpuFeatures::TIMESTAMP_QUERY_INSIDE_PASSES)
            );
        }
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "Paramètres injectés par Bevy ECS pour le scénario de mesure."
)]
fn prepare(
    time: Res<Time<Real>>,
    config: Res<GameConfig>,
    mut samples: ResMut<ProfileSamples>,
    mut pilot: ResMut<Explorer>,
    mut surface: ResMut<SurfaceState>,
    mut terrain: ResMut<MarsTerrain>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut request: ResMut<NavigationRequest>,
    mut clock: ResMut<SimulationClock>,
) {
    let profile = config.profile.as_ref().unwrap();
    let (phase, _) = profile.phase(time.elapsed_secs_f64());
    keys.reset_all();
    pilot.captured = phase == "contact" || phase == "streaming";
    pilot.boosted = phase == "streaming";
    if samples.phase != phase {
        match phase {
            "warmup" => {
                clock.jd = J2000;
                clock.paused = true;
                clock.speed_index = 0;
                request.surface = true;
            }
            "streaming" => {
                // Départ canonique proche d'une limite : franchissement garanti
                // sans dépendre du nombre de frames de la phase contact.
                surface.feet = DVec3::new(116.0, 0.0, 0.0);
                if terrain.globe.is_some() {
                    surface.migrate(&mut terrain);
                }
                surface.feet.y = terrain.support_height(surface.feet, HALF_WIDTH_M) + 0.002;
                surface.velocity = DVec3::ZERO;
                surface.grounded = true;
                surface.accumulator = 0.0;
            }
            "space" => request.surface = true,
            _ => {}
        }
        info!(
            "Profilage {} phase {phase}, pieds {:?}",
            profile.variant, surface.feet
        );
        samples.phase = phase;
    }
    match phase {
        "contact" => keys.press(KeyCode::KeyW),
        "streaming" => keys.press(KeyCode::KeyD),
        _ => {}
    }
    if phase == "warmup" {
        surface.pitch = -0.12;
    }
}
fn begin(mut samples: ResMut<ProfileSamples>) {
    samples.cpu_frame.clear();
    samples.metric = "";
    samples.mark("cpu/simulation");
}
fn navigation(mut samples: ResMut<ProfileSamples>) {
    samples.mark("cpu/navigation");
}
fn rendering(mut samples: ResMut<ProfileSamples>) {
    samples.mark("cpu/rendering_sync");
}
fn interface(mut samples: ResMut<ProfileSamples>) {
    samples.mark("cpu/interface");
}
fn finish(mut samples: ResMut<ProfileSamples>) {
    samples.mark("");
}

fn isolate(
    config: Res<GameConfig>,
    pilot: Res<Explorer>,
    mut cameras: Query<(
        &mut Camera,
        &mut Msaa,
        Option<&SpaceCamera>,
        Option<&SurfaceCamera>,
    )>,
    visuals: Query<(&MeshMaterial3d<StandardMaterial>, &RenderLayers)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let variant = config.profile.as_ref().unwrap().variant.as_str();
    let surface = pilot.mode == MovementMode::Surface;
    for (mut camera, mut msaa, space, ground) in &mut cameras {
        if space.is_some() {
            camera.is_active = !(surface && variant == "no_space_camera");
        }
        if ground.is_some() {
            camera.is_active = surface && variant != "no_surface_camera";
        }
        if variant == "msaa_off" {
            *msaa = Msaa::Off;
        }
    }
    if variant == "surface_unlit" {
        for (handle, layers) in &visuals {
            if layers.intersects(&RenderLayers::layer(1))
                && let Some(mut material) = materials.get_mut(&handle.0)
                && !material.unlit
            {
                material.unlit = true;
            }
        }
    }
}

fn collect(
    time: Res<Time<Real>>,
    config: Res<GameConfig>,
    diagnostics: Res<DiagnosticsStore>,
    mut samples: ResMut<ProfileSamples>,
    mut exit: MessageWriter<AppExit>,
) {
    let profile = config.profile.as_ref().unwrap();
    let (phase, phase_seconds) = profile.phase(time.elapsed_secs_f64());
    if phase == "finished" {
        match samples.write_csv(&profile.output) {
            Ok(()) => {
                info!(
                    "Profilage terminé : {} métriques, CSV {}",
                    samples.values.len(),
                    profile.output.display()
                );
                exit.write(AppExit::Success);
            }
            Err(error) => {
                error!("Écriture du profilage impossible : {error}");
                exit.write(AppExit::error());
            }
        }
        return;
    }
    // Dédoublonner les diagnostics asynchrones même pendant l'échauffement :
    // aucune ancienne mesure ne doit être réattribuée à la phase suivante.
    let measuring = phase != "warmup" && phase_seconds >= SETTLE_SECONDS;
    for diagnostic in diagnostics.iter() {
        let path = diagnostic.path().to_string();
        if !path.starts_with("render/")
            || !(path.ends_with("elapsed_cpu") || path.ends_with("elapsed_gpu"))
        {
            continue;
        }
        if let Some(measurement) = diagnostic.measurement()
            && samples
                .seen
                .get(&path)
                .is_none_or(|old| *old < measurement.time)
        {
            samples.seen.insert(path.clone(), measurement.time);
            if measuring {
                samples.add(path, measurement.value);
            }
        }
    }
    if measuring {
        samples.add("frame".into(), time.delta_secs_f64() * 1000.0);
        for (metric, value) in samples.cpu_frame.clone() {
            samples.add(metric.into(), value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn profile_boundaries_keep_warmup_and_four_equal_phases() {
        let profile = ProfileConfig {
            seconds: 42.0,
            variant: "baseline".into(),
            output: "unused.csv".into(),
            gpu_diagnostics: true,
            global_terrain: false,
        };
        for (seconds, phase, local) in [
            (0.0, "warmup", 0.0),
            (9.9, "warmup", 9.9),
            (10.0, "rest", 0.0),
            (17.9, "rest", 7.9),
            (18.0, "contact", 0.0),
            (26.0, "streaming", 0.0),
            (34.0, "space", 0.0),
            (42.0, "finished", 0.0),
        ] {
            let actual = profile.phase(seconds);
            assert_eq!(actual.0, phase);
            assert!((actual.1 - local).abs() < 1e-9);
        }
    }

    #[test]
    fn normal_game_does_not_install_profiling_systems_or_resources() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(GameConfig {
                survival_validation_dir: None,
                start_jd: J2000,
                smoke_seconds: None,
                surface_smoke_seconds: None,
                rotation_smoke_seconds: None,
                p2_seconds: None,
                profile: None,
                cube_surface: false,
                cube_migration: false,
                cube_offcenter: false,
                mars_global: false,
                globe_validation_dir: None,
                cube_validation_dir: None,
            })
            .add_plugins(ProfilingPlugin);
        app.update();
        assert!(!app.world().contains_resource::<ProfileSamples>());
    }
}
