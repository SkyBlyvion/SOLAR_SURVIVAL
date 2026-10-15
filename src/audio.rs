//! P11 : signaux de combinaison/interface, sans autorité sur le gameplay.
use crate::{
    GameSet,
    config::GameConfig,
    player::{Explorer, MovementMode},
    survival::{LOW_SECONDS, Oxygen},
    ui::menu::{MenuState, PlayerSettings},
};
use bevy::{
    audio::{AudioSinkPlayback, Volume},
    prelude::*,
};

pub const MAX_VOICES: usize = 8;
const VOICE_TTL_S: f64 = 2.0;

/// Déclenché après le résultat réel d'une commande, jamais depuis un texte affiché.
#[derive(Event, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundCue {
    Ui,
    Refused,
    Harvest,
    Built,
    Survey,
    Report,
    Refill,
    Low,
    Critical,
    Failed,
    Saved,
}
impl SoundCue {
    pub const ALL: [Self; 11] = [
        Self::Ui,
        Self::Refused,
        Self::Harvest,
        Self::Built,
        Self::Survey,
        Self::Report,
        Self::Refill,
        Self::Low,
        Self::Critical,
        Self::Failed,
        Self::Saved,
    ];
    pub fn path(self) -> &'static str {
        match self {
            Self::Ui => "audio/ui.wav",
            Self::Refused => "audio/refused.wav",
            Self::Harvest => "audio/harvest.wav",
            Self::Built => "audio/built.wav",
            Self::Survey => "audio/survey.wav",
            Self::Report => "audio/report.wav",
            Self::Refill => "audio/refill.wav",
            Self::Low => "audio/low.wav",
            Self::Critical => "audio/critical.wav",
            Self::Failed => "audio/failed.wav",
            Self::Saved => "audio/saved.wav",
        }
    }
    fn priority(self) -> u8 {
        match self {
            Self::Failed => 4,
            Self::Critical => 3,
            Self::Low => 2,
            Self::Ui => 0,
            _ => 1,
        }
    }
}
#[derive(Resource, Default)]
struct Pending(Vec<SoundCue>);
fn collect(event: On<SoundCue>, mut pending: ResMut<Pending>) {
    let cue = *event.event();
    // Au plus une occurrence de chaque signal en attente, même sans périphérique.
    if !pending.0.contains(&cue) {
        pending.0.push(cue);
    }
}
#[derive(Resource)]
struct Clips(Vec<Handle<AudioSource>>);
#[derive(Component)]
struct Voice {
    expires: f64,
    cue: SoundCue,
}
#[derive(Resource)]
struct Mixer {
    last: [f64; 11],
}
impl Default for Mixer {
    fn default() -> Self {
        Self {
            last: [f64::NEG_INFINITY; 11],
        }
    }
}
#[derive(Resource, Default)]
struct Alerts {
    active: bool,
    failed: bool,
    remaining: f64,
    charging: bool,
    band: u8,
    repeat_s: f64,
}
impl Alerts {
    fn advance(&mut self, oxygen: &Oxygen, active: bool, dt: f64) -> Option<SoundCue> {
        let charging = active
            && self.active
            && !oxygen.failed
            && !self.failed
            && oxygen.shelter.is_some()
            && oxygen.remaining_seconds > self.remaining + 1e-9;
        let band = if !active || oxygen.failed || charging {
            0
        } else if oxygen.remaining_seconds <= 10.0 {
            2
        } else if oxygen.remaining_seconds <= LOW_SECONDS {
            1
        } else {
            0
        };
        let cue = if active && self.active && oxygen.failed && !self.failed {
            Some(SoundCue::Failed)
        } else if charging && !self.charging {
            Some(SoundCue::Refill)
        } else if band > 0 {
            self.repeat_s -= dt.max(0.0);
            if band != self.band || self.repeat_s <= 0.0 {
                self.repeat_s = if band == 2 { 1.5 } else { 5.0 };
                Some(if band == 2 {
                    SoundCue::Critical
                } else {
                    SoundCue::Low
                })
            } else {
                None
            }
        } else {
            None
        };
        self.active = active;
        self.failed = oxygen.failed;
        self.remaining = oxygen.remaining_seconds;
        self.charging = charging;
        self.band = band;
        cue
    }
}

pub struct GameAudioPlugin;
impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Pending>()
            .init_resource::<Alerts>()
            .init_resource::<Mixer>()
            .add_observer(collect)
            .add_systems(Startup, load)
            .add_systems(Update, alerts.in_set(GameSet::Rendering))
            .add_systems(PostUpdate, playback);
    }
}
fn load(
    mut commands: Commands,
    server: Option<Res<AssetServer>>,
    sources: Option<Res<Assets<AudioSource>>>,
) {
    if let (Some(server), Some(_)) = (server, sources) {
        commands.insert_resource(Clips(SoundCue::ALL.map(|s| server.load(s.path())).into()));
    }
}
fn alerts(
    mut commands: Commands,
    time: Res<Time<Real>>,
    oxygen: Res<Oxygen>,
    pilot: Res<Explorer>,
    menu: Option<Res<MenuState>>,
    config: Res<GameConfig>,
    mut state: ResMut<Alerts>,
) {
    let active = oxygen.enabled
        && !config.automated()
        && pilot.mode == MovementMode::Surface
        && menu.is_none_or(|m| !m.is_open());
    if let Some(cue) = state.advance(&oxygen, active, time.delta_secs_f64()) {
        commands.trigger(cue);
    }
}
#[expect(
    clippy::too_many_arguments,
    reason = "Mixage borné, options et cycle de vie des voix ECS."
)]
fn playback(
    mut commands: Commands,
    time: Res<Time<Real>>,
    settings: Res<PlayerSettings>,
    config: Res<GameConfig>,
    clips: Option<Res<Clips>>,
    mut pending: ResMut<Pending>,
    mut mixer: ResMut<Mixer>,
    mut voices: Query<(
        Entity,
        &Voice,
        &mut PlaybackSettings,
        Option<&mut AudioSink>,
    )>,
    pilot: Res<Explorer>,
    menu: Option<Res<MenuState>>,
    sources: Option<Res<Assets<AudioSource>>>,
) {
    let now = time.elapsed_secs_f64();
    let gain = settings.audio_gain();
    let silent = gain == 0.0 || config.automated();
    let gameplay = pilot.mode == MovementMode::Surface && menu.is_none_or(|m| !m.is_open());
    let is_game = |cue| !matches!(cue, SoundCue::Ui | SoundCue::Saved | SoundCue::Refused);
    let mut live = Vec::with_capacity(MAX_VOICES);
    for (entity, voice, mut playback, sink) in &mut voices {
        if silent || now >= voice.expires || !gameplay && is_game(voice.cue) {
            commands.entity(entity).despawn();
        } else {
            live.push((entity, voice.cue.priority()));
            if settings.is_changed() {
                playback.volume = Volume::Linear(gain);
                if let Some(mut sink) = sink {
                    sink.set_volume(Volume::Linear(gain));
                }
            }
        }
    }
    // Consommer immédiatement : aucun rattrapage sonore après muet ou chargement lent.
    let mut cues = std::mem::take(&mut pending.0);
    if silent {
        return;
    }
    let (Some(clips), Some(sources)) = (clips, sources) else {
        return;
    };
    cues.sort_by_key(|cue| std::cmp::Reverse(cue.priority()));
    for cue in cues {
        let i = cue as usize;
        if now - mixer.last[i] < 0.15
            || !gameplay && is_game(cue)
            || sources.get(&clips.0[i]).is_none()
        {
            continue;
        }
        if live.len() == MAX_VOICES {
            let (j, &(_, priority)) = live
                .iter()
                .enumerate()
                .min_by_key(|(_, (_, p))| *p)
                .unwrap();
            if cue.priority() <= priority {
                continue;
            }
            commands.entity(live.swap_remove(j).0).despawn();
        }
        mixer.last[i] = now;
        let entity = commands
            .spawn((
                AudioPlayer::new(clips.0[i].clone()),
                PlaybackSettings {
                    volume: Volume::Linear(gain),
                    ..PlaybackSettings::DESPAWN
                },
                Voice {
                    expires: now + VOICE_TTL_S,
                    cue,
                },
            ))
            .id();
        live.push((entity, cue.priority()));
    }
}
/// Les sons/timers transitoires ne traversent jamais un chargement/changement de profil.
pub(crate) fn reset(world: &mut World) {
    if world.contains_resource::<Pending>() {
        world.insert_resource(Pending::default());
        world.insert_resource(Alerts::default());
        world.insert_resource(Mixer::default());
        let entities: Vec<_> = world
            .query_filtered::<Entity, With<Voice>>()
            .iter(world)
            .collect();
        for entity in entities {
            world.despawn(entity);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::{
        audio::{Decodable, Source},
        time::TimeUpdateStrategy,
    };
    use std::{path::Path, time::Duration};

    #[test]
    fn original_wavs_decode_with_the_actual_bevy_decoder_and_stay_bounded() {
        let mut bytes = 0;
        for cue in SoundCue::ALL {
            let data = std::fs::read(
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("assets")
                    .join(cue.path()),
            )
            .unwrap();
            bytes += data.len();
            let source = AudioSource { bytes: data.into() };
            let decoder = source.decoder();
            assert_eq!(decoder.sample_rate().get(), 22050);
            assert_eq!(decoder.channels().get(), 1);
            let samples: Vec<_> = decoder.collect();
            assert!((500..22050).contains(&samples.len()));
            assert!(samples.iter().all(|s| s.is_finite() && s.abs() <= 0.111));
            assert!(samples.iter().any(|s| s.abs() > 0.02));
            assert_eq!(samples[0], 0.0);
            assert_eq!(*samples.last().unwrap(), 0.0);
        }
        assert!(bytes < 200_000);
    }

    #[test]
    fn oxygen_thresholds_recharge_failure_and_suspension_use_active_real_seconds() {
        let mut alerts = Alerts::default();
        let mut oxygen = Oxygen {
            remaining_seconds: 31.0,
            ..default()
        };
        assert_eq!(alerts.advance(&oxygen, true, 0.1), None);
        oxygen.remaining_seconds = 30.0;
        assert_eq!(alerts.advance(&oxygen, true, 0.1), Some(SoundCue::Low));
        assert_eq!(alerts.advance(&oxygen, true, 4.9), None);
        assert_eq!(alerts.advance(&oxygen, true, 0.11), Some(SoundCue::Low));
        oxygen.remaining_seconds = 10.0;
        assert_eq!(alerts.advance(&oxygen, true, 0.1), Some(SoundCue::Critical));
        assert_eq!(alerts.advance(&oxygen, true, 1.0), None);
        assert_eq!(alerts.advance(&oxygen, true, 0.5), Some(SoundCue::Critical));
        // Longue frame : une seule alerte, aucune rafale de rattrapage.
        assert_eq!(
            alerts.advance(&oxygen, true, 120.0),
            Some(SoundCue::Critical)
        );
        assert_eq!(alerts.advance(&oxygen, false, 120.0), None);
        assert_eq!(alerts.advance(&oxygen, false, 120.0), None);
        assert_eq!(alerts.advance(&oxygen, true, 0.1), Some(SoundCue::Critical));
        oxygen.shelter = Some(crate::buildings::BuildingId(1));
        oxygen.remaining_seconds = 12.0;
        assert_eq!(alerts.advance(&oxygen, true, 0.1), Some(SoundCue::Refill));
        oxygen.remaining_seconds = 15.0;
        assert_eq!(alerts.advance(&oxygen, true, 0.1), None);
        oxygen.shelter = None;
        oxygen.remaining_seconds = 0.0;
        oxygen.failed = true;
        assert_eq!(alerts.advance(&oxygen, true, 0.1), Some(SoundCue::Failed));
        assert_eq!(alerts.advance(&oxygen, true, 100.0), None);
        oxygen.restart();
        assert_eq!(alerts.advance(&oxygen, true, 0.1), None);
        let mut loaded = Alerts::default();
        oxygen.failed = true;
        oxygen.remaining_seconds = 0.0;
        assert_eq!(loaded.advance(&oxygen, false, 0.1), None);
        assert_eq!(loaded.advance(&oxygen, true, 0.1), None);
    }

    fn app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(
                100,
            )))
            .insert_resource(GameConfig::default())
            .init_resource::<PlayerSettings>()
            .init_resource::<Oxygen>()
            .init_resource::<Explorer>()
            .add_plugins(GameAudioPlugin);
        app.world_mut().resource_mut::<Explorer>().mode = MovementMode::Surface;
        let mut sources = Assets::<AudioSource>::default();
        let clips = SoundCue::ALL.map(|cue| {
            sources.add(AudioSource {
                bytes: std::fs::read(
                    Path::new(env!("CARGO_MANIFEST_DIR"))
                        .join("assets")
                        .join(cue.path()),
                )
                .unwrap()
                .into(),
            })
        });
        app.insert_resource(sources)
            .insert_resource(Clips(clips.into()));
        app.update();
        app
    }
    fn voices(app: &mut App) -> Vec<SoundCue> {
        app.world_mut()
            .query::<&Voice>()
            .iter(app.world())
            .map(|v| v.cue)
            .collect()
    }

    #[test]
    fn mixer_prioritizes_alerts_caps_voices_and_expires_even_without_audio_device() {
        // Aucun AudioPlugin, AudioOutput, fenêtre OS ou périphérique ouvert.
        let mut app = app();
        for _ in 0..100 {
            for cue in SoundCue::ALL {
                app.world_mut().trigger(cue);
            }
        }
        assert_eq!(app.world().resource::<Pending>().0.len(), 11);
        app.update();
        let cues = voices(&mut app);
        assert_eq!(cues.len(), MAX_VOICES);
        assert!(cues.contains(&SoundCue::Failed));
        assert!(cues.contains(&SoundCue::Critical));
        assert!(!cues.contains(&SoundCue::Ui));
        for _ in 0..22 {
            app.update();
        }
        assert!(voices(&mut app).is_empty());
        // Préemption d'une voix moins prioritaire déjà en cours.
        for cue in [
            SoundCue::Ui,
            SoundCue::Refused,
            SoundCue::Harvest,
            SoundCue::Built,
            SoundCue::Survey,
            SoundCue::Report,
            SoundCue::Refill,
            SoundCue::Saved,
        ] {
            app.world_mut().trigger(cue);
        }
        app.update();
        assert_eq!(voices(&mut app).len(), MAX_VOICES);
        app.world_mut().trigger(SoundCue::Critical);
        app.update();
        let cues = voices(&mut app);
        assert_eq!(cues.len(), MAX_VOICES);
        assert!(cues.contains(&SoundCue::Critical));
        assert!(!cues.contains(&SoundCue::Ui));
    }

    #[test]
    fn mute_menus_space_diagnostics_loading_and_reset_do_not_replay_old_sounds() {
        let mut app = app();
        app.world_mut().trigger(SoundCue::Survey);
        app.update();
        assert_eq!(voices(&mut app), vec![SoundCue::Survey]);
        app.world_mut().resource_mut::<PlayerSettings>().muted = true;
        app.world_mut().trigger(SoundCue::Report);
        app.update();
        assert!(voices(&mut app).is_empty());
        app.world_mut().resource_mut::<PlayerSettings>().muted = false;
        app.update();
        assert!(voices(&mut app).is_empty());
        app.insert_resource(MenuState::default());
        app.world_mut().trigger(SoundCue::Survey);
        app.world_mut().trigger(SoundCue::Ui);
        app.update();
        assert_eq!(voices(&mut app), vec![SoundCue::Ui]);
        app.world_mut().remove_resource::<MenuState>();
        app.world_mut().resource_mut::<Explorer>().mode = MovementMode::Space;
        app.world_mut().trigger(SoundCue::Survey);
        app.update();
        assert!(!voices(&mut app).contains(&SoundCue::Survey));
        reset(app.world_mut());
        assert!(voices(&mut app).is_empty());
        app.world_mut().resource_mut::<Explorer>().mode = MovementMode::Surface;
        let clips = app.world_mut().remove_resource::<Clips>().unwrap();
        app.world_mut().trigger(SoundCue::Report);
        app.update();
        app.insert_resource(clips);
        app.update();
        assert!(voices(&mut app).is_empty());
        app.world_mut().resource_mut::<GameConfig>().smoke_seconds = Some(30.0);
        app.world_mut().trigger(SoundCue::Ui);
        app.update();
        assert!(voices(&mut app).is_empty());
        assert!(app.world().resource::<Pending>().0.is_empty());
    }
}
