use super::{
    metronome::Metronome,
    process::{AudioProcessRunner, NoopProcess},
    sampler_converter::{SampleConversionResult, SampleConverter},
    sequencer::Sequencer,
};
use crate::bloop::AudioEngineStatus;
use crate::bloop::AudioPreferences;
use crate::{
    audio::process::{create_audio_process, create_dummy_process},
    bloop::{AudioStatus, Response},
};
use crate::{
    model::{PlaybackState, PlayingState, Progress, Project, ID},
    samples::SamplesCache,
};
use futures::StreamExt;
use futures_channel::mpsc;
use log::{error, info};
use rawdio::{connect_nodes, create_engine_with_options, AudioBuffer, Context, EngineOptions, Mixer, Sampler};
use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};
use tokio::sync::broadcast;

/// Tracks whether the audio backend is healthy, stopped, or failed to initialise.
#[derive(Debug, Clone, PartialEq)]
pub enum AudioEngineState {
    /// Engine is running and the audio device opened successfully.
    Running,
    /// Engine was explicitly stopped via `stop_audio`.
    Stopped,
    /// Engine is running but the audio device failed to open; audio is silent.
    Failed { reason: String },
}

/// The short-lived audio backend. Dropped and recreated by `stop_audio`/`start_audio`.
struct AudioEngine {
    context: Box<dyn Context>,
    mixer: Mixer,
    metronome: Metronome,
    samplers: HashMap<ID, Sampler>,
    sequencer: Sequencer,
    #[allow(dead_code)]
    realtime_process: Box<dyn AudioProcessRunner>,
    tick_interval: tokio::time::Interval,
}

fn build_audio_engine(preferences: &AudioPreferences, use_dummy_audio: bool) -> (AudioEngine, AudioEngineState) {
    let (mut context, process) = create_engine_with_options(
        EngineOptions::default()
            .with_sample_rate(preferences.sample_rate as usize)
            .with_maximum_channel_count(preferences.output_channel_count as usize),
    );

    let mixer = Mixer::unity(context.as_ref(), preferences.output_channel_count as usize);
    connect_nodes!(mixer => "output");

    let metronome = Metronome::new(context.as_ref());

    let click_offset = preferences.click_channel_offset as usize;
    if preferences.output_channel_count as usize >= click_offset + 2 {
        metronome
            .output_node()
            .connect_channels_to(&mixer.node, 0, click_offset, 2);
    }

    context.start();

    let (realtime_process, state) = if use_dummy_audio {
        (
            create_dummy_process(process, preferences.clone()),
            AudioEngineState::Running,
        )
    } else {
        match create_audio_process(process, preferences.clone()) {
            Ok(process) => (process, AudioEngineState::Running),
            Err(reason) => (
                Box::new(NoopProcess) as Box<dyn AudioProcessRunner>,
                AudioEngineState::Failed { reason },
            ),
        }
    };

    (
        AudioEngine {
            context,
            mixer,
            metronome,
            samplers: HashMap::new(),
            sequencer: Sequencer::default(),
            realtime_process,
            tick_interval: tokio::time::interval(Duration::from_secs_f64(1.0 / 60.0)),
        },
        state,
    )
}

fn add_samples_from_project(
    engine: &mut AudioEngine,
    project: &Project,
    samples_cache: &SamplesCache,
    sample_converter: &SampleConverter,
    samples_being_converted: &mut HashSet<ID>,
) {
    for song in project.songs.iter() {
        let Some(sample) = song.sample.as_ref() else {
            continue;
        };
        if engine.samplers.contains_key(&sample.id) {
            continue;
        }
        let Some(cached_sample) = samples_cache.get_sample(sample.id) else {
            continue;
        };
        if !cached_sample.is_cached() {
            continue;
        }
        if samples_being_converted.contains(&sample.id) {
            continue;
        }
        samples_being_converted.insert(sample.id);
        sample_converter.convert(sample.id, cached_sample.get_path().to_path_buf());
    }
}

fn remove_samples_from_engine(engine: &mut AudioEngine, project: &Project) {
    let samples_to_remove: HashSet<ID> = engine
        .samplers
        .iter()
        .filter(|(&sample_id, _)| project.find_sample(sample_id).is_none())
        .map(|(sample_id, _)| *sample_id)
        .collect();

    for sample_id in samples_to_remove {
        if let Some(mut sampler) = engine.samplers.remove(&sample_id) {
            sampler.node.disconnect_from_node(&engine.mixer.node);
            sampler.stop_now();
        }
    }
}

/// Long-lived controller. The rawdio context and cpal stream live inside the
/// `Option<AudioEngine>` so they can be dropped and rebuilt without affecting
/// project state, samples cache, or the response channel.
#[allow(dead_code)]
pub struct AudioController {
    engine: Option<AudioEngine>,
    engine_state: AudioEngineState,
    use_dummy_audio: bool,
    sample_converter: SampleConverter,
    conversion_rx: mpsc::Receiver<SampleConversionResult>,
    response_tx: broadcast::Sender<Response>,
    samples_being_converted: HashSet<ID>,
    playback_state: PlaybackState,
    progress: Progress,
    project: Project,
    preferences: AudioPreferences,
}

impl AudioController {
    pub fn new(response_tx: broadcast::Sender<Response>, preferences: AudioPreferences, use_dummy_audio: bool) -> Self {
        let (conversion_tx, conversion_rx) = mpsc::channel(64);
        let (engine, engine_state) = build_audio_engine(&preferences, use_dummy_audio);

        Self {
            engine: Some(engine),
            engine_state,
            use_dummy_audio,
            sample_converter: SampleConverter::new(conversion_tx, preferences.sample_rate as usize),
            conversion_rx,
            response_tx,
            samples_being_converted: HashSet::new(),
            playback_state: PlaybackState::default(),
            progress: Progress::default(),
            project: Project::empty(),
            preferences,
        }
    }

    /// Returns the current state of the audio engine.
    #[allow(dead_code)]
    pub fn engine_state(&self) -> &AudioEngineState {
        &self.engine_state
    }

    /// Build an `AudioStatus` proto reflecting the current engine state and
    /// preferences.
    pub fn get_audio_status(&self) -> AudioStatus {
        let (engine_status, error) = match &self.engine_state {
            AudioEngineState::Running => (AudioEngineStatus::AUDIO_ENGINE_RUNNING, String::new()),
            AudioEngineState::Stopped => (AudioEngineStatus::AUDIO_ENGINE_STOPPED, String::new()),
            AudioEngineState::Failed { reason } => (AudioEngineStatus::AUDIO_ENGINE_FAILED, reason.clone()),
        };
        AudioStatus {
            current_device_id: self.preferences.output_device.clone(),
            current_device_name: self.preferences.output_device.clone(),
            current_sample_rate: self.preferences.sample_rate,
            current_channel_count: self.preferences.output_channel_count,
            current_buffer_size: self.preferences.buffer_size,
            engine_status: engine_status.into(),
            error,
            ..Default::default()
        }
    }

    fn broadcast_audio_status(&self) {
        let status = self.get_audio_status();
        let _ = self.response_tx.send(Response::default().with_audio_status(&status));
    }

    /// Build and start a fresh audio engine, re-triggering sample conversions
    /// for the current project. No-ops if the engine is already running.
    pub fn start_audio(&mut self, samples_cache: &SamplesCache) {
        if self.engine.is_some() {
            info!("Audio engine already running");
            return;
        }

        let (engine, state) = build_audio_engine(&self.preferences, self.use_dummy_audio);
        self.engine = Some(engine);
        self.engine_state = state;

        if matches!(self.engine_state, AudioEngineState::Failed { .. }) {
            self.broadcast_stopped_playback();
        }

        self.broadcast_audio_status();
        self.samples_being_converted.clear();
        let project = self.project.clone();
        if let Some(engine) = self.engine.as_mut() {
            add_samples_from_project(
                engine,
                &project,
                samples_cache,
                &self.sample_converter,
                &mut self.samples_being_converted,
            );
        }

        info!("Audio engine started (state: {:?})", self.engine_state);
    }

    /// Stop the audio engine, cleanly dropping the cpal stream and rawdio
    /// context so the OS releases the device. No-ops if already stopped.
    pub fn stop_audio(&mut self) {
        let Some(mut engine) = self.engine.take() else {
            info!("Audio engine already stopped");
            return;
        };

        engine.sequencer.stop(&mut engine.samplers);
        drop(engine);

        self.engine_state = AudioEngineState::Stopped;
        self.samples_being_converted.clear();
        self.broadcast_stopped_playback();
        self.broadcast_audio_status();
        info!("Audio engine stopped");
    }

    /// Reset playback/progress state to stopped and broadcast both to clients.
    fn broadcast_stopped_playback(&mut self) {
        self.playback_state = PlaybackState::default();
        self.progress = Progress::default();
        let _ = self
            .response_tx
            .send(Response::default().with_playback_state(&self.playback_state));
        let _ = self.response_tx.send(Response::default().with_progress(&self.progress));
    }

    /// Drive one iteration of the audio controller event loop. Safe to call
    /// whether the engine is running or stopped — never panics, never busy-loops.
    pub async fn run(&mut self) {
        if let Some(engine) = self.engine.as_mut() {
            tokio::select! {
                Some(conversion_result) = self.conversion_rx.next() => {
                    self.on_sample_converted(conversion_result)
                },
                _ = engine.tick_interval.tick() => {
                    self.interval_tick()
                },
            }
        } else {
            tokio::select! {
                Some(conversion_result) = self.conversion_rx.next() => {
                    // Engine not running; discard the stale result. Samples will
                    // be re-converted when start_audio is called.
                    self.samples_being_converted.remove(&conversion_result.sample_id);
                },
                _ = std::future::pending::<()>() => (),
            }
        }
    }

    pub fn play(&mut self) {
        let Some(engine) = self.engine.as_mut() else {
            return;
        };
        let lookahead = engine.context.current_time().incremented_by_seconds(0.001);
        engine
            .sequencer
            .play(lookahead, self.project.clone(), &mut engine.samplers);
    }

    pub fn stop(&mut self) {
        let Some(engine) = self.engine.as_mut() else {
            return;
        };
        engine.sequencer.stop(&mut engine.samplers);
    }

    pub fn enter_loop(&mut self) {
        let Some(engine) = self.engine.as_mut() else {
            return;
        };
        let lookahead = engine.context.current_time().incremented_by_seconds(0.001);
        engine.sequencer.enter_loop(lookahead, &mut engine.samplers);
    }

    pub fn exit_loop(&mut self) {
        let Some(engine) = self.engine.as_mut() else {
            return;
        };
        let lookahead = engine.context.current_time().incremented_by_seconds(0.001);
        engine.sequencer.exit_loop(lookahead, &mut engine.samplers);
    }

    pub fn queue(&mut self, song_id: ID, section_id: ID) {
        let Some(engine) = self.engine.as_mut() else {
            return;
        };
        let lookahead = engine.context.current_time().incremented_by_seconds(0.001);
        engine
            .sequencer
            .queue(lookahead, song_id, section_id, &mut engine.samplers);
    }

    pub fn toggle_loop(&mut self) {
        if self.playback_state.looping {
            self.exit_loop();
        } else {
            self.enter_loop();
        }
    }

    pub fn toggle_play(&mut self) {
        match self.playback_state.playing.enum_value_or_default() {
            PlayingState::STOPPED => self.play(),
            PlayingState::PLAYING => self.stop(),
        }
    }

    pub fn get_playback_state(&self) -> &PlaybackState {
        &self.playback_state
    }

    pub fn on_project_updated(&mut self, project: &Project, samples_cache: &SamplesCache) {
        if let Some(engine) = self.engine.as_mut() {
            add_samples_from_project(
                engine,
                project,
                samples_cache,
                &self.sample_converter,
                &mut self.samples_being_converted,
            );
            remove_samples_from_engine(engine, project);
        }
        self.project = project.clone();
    }

    fn interval_tick(&mut self) {
        let Some(engine) = self.engine.as_mut() else {
            return;
        };

        let current_time = engine.context.current_time();
        engine.context.process_notifications();
        engine.sequencer.set_current_time(current_time);
        engine.metronome.schedule(&current_time, &engine.sequencer);

        let playback_state = engine.sequencer.get_playback_state();
        let progress = engine.sequencer.get_progress();
        // NLL ends the engine borrow here; safe to access other self fields below.

        if self.playback_state != playback_state {
            self.playback_state = playback_state;
            let _ = self
                .response_tx
                .send(Response::default().with_playback_state(&self.playback_state));
        }

        if self.progress != progress {
            self.progress = progress;
            let _ = self.response_tx.send(Response::default().with_progress(&self.progress));
        }
    }

    fn on_sample_converted(&mut self, result: SampleConversionResult) {
        self.samples_being_converted.remove(&result.sample_id);

        let audio_data = match result.result {
            Ok(data) => data,
            Err(error) => {
                error!("Error converting audio file {}: {}", result.sample_id, error);
                return;
            }
        };

        // Extract preferences values before taking the engine borrow.
        let main_offset = self.preferences.main_channel_offset as usize;
        let output_channel_count = self.preferences.output_channel_count as usize;

        let Some(engine) = self.engine.as_mut() else {
            info!(
                "Sample converted but engine is stopped, discarding: {}",
                result.sample_id
            );
            return;
        };

        info!("Sample converted: {}", result.sample_id);

        let audio_channel_count = audio_data.channel_count();
        let channel_count = audio_channel_count.min(output_channel_count - main_offset);

        let sampler = Sampler::new_with_event_capacity(engine.context.as_ref(), audio_data, 1024);
        sampler
            .node
            .connect_channels_to(&engine.mixer.node, 0, main_offset, channel_count);

        engine.samplers.insert(result.sample_id, sampler);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preferences::default_audio_preferences;
    use crate::samples::SamplesCache;
    use tempfile::tempdir;
    use tokio::sync::broadcast;

    fn test_controller() -> AudioController {
        let (response_tx, _) = broadcast::channel(100);
        AudioController::new(response_tx, default_audio_preferences(), true)
    }

    #[tokio::test]
    async fn stop_audio_cleanly_drops_engine() {
        let mut controller = test_controller();
        assert!(controller.engine.is_some());
        controller.stop_audio();
        assert!(controller.engine.is_none());
    }

    #[tokio::test]
    async fn start_audio_rebuilds_engine() {
        let dir = tempdir().unwrap();
        let samples_cache = SamplesCache::new(dir.path());

        let mut controller = test_controller();
        controller.stop_audio();
        assert!(controller.engine.is_none());
        controller.start_audio(&samples_cache);
        assert!(controller.engine.is_some());
    }

    #[tokio::test]
    async fn run_makes_progress_when_engine_running() {
        let mut controller = test_controller();
        assert!(controller.engine.is_some());

        // run() should complete without panicking (tick_interval fires within timeout)
        tokio::time::timeout(std::time::Duration::from_millis(200), controller.run())
            .await
            .expect("run() timed out while engine was running");
    }

    #[tokio::test]
    async fn run_does_not_panic_when_engine_stopped() {
        let mut controller = test_controller();
        controller.stop_audio();
        assert!(controller.engine.is_none());

        // With engine stopped and no pending conversions, run() waits indefinitely via
        // pending(). The timeout is expected to fire — no panic is the assertion.
        let result = tokio::time::timeout(std::time::Duration::from_millis(50), controller.run()).await;
        assert!(result.is_err(), "Expected timeout, not an early return");
    }

    #[tokio::test]
    async fn stop_then_start_then_run_without_panic() {
        let dir = tempdir().unwrap();
        let samples_cache = SamplesCache::new(dir.path());

        let mut controller = test_controller();
        controller.stop_audio();
        controller.start_audio(&samples_cache);

        tokio::time::timeout(std::time::Duration::from_millis(200), controller.run())
            .await
            .expect("run() timed out after stop/start cycle");
    }

    #[tokio::test]
    async fn new_with_dummy_audio_has_running_state() {
        let controller = test_controller();
        assert_eq!(*controller.engine_state(), AudioEngineState::Running);
    }

    #[tokio::test]
    async fn stop_audio_sets_stopped_state() {
        let mut controller = test_controller();
        controller.stop_audio();
        assert_eq!(*controller.engine_state(), AudioEngineState::Stopped);
    }

    #[tokio::test]
    async fn stop_audio_broadcasts_stopped_playback_state() {
        let (response_tx, mut response_rx) = broadcast::channel(100);
        let mut controller = AudioController::new(response_tx, default_audio_preferences(), true);

        controller.stop_audio();

        let mut got_stopped = false;
        while let Ok(response) = response_rx.try_recv() {
            if let Some(ps) = response.playback_state.as_ref() {
                if ps.playing.enum_value_or_default() == PlayingState::STOPPED {
                    got_stopped = true;
                }
            }
        }
        assert!(
            got_stopped,
            "Expected a STOPPED PlaybackState broadcast after stop_audio"
        );
    }

    #[tokio::test]
    async fn stop_audio_broadcasts_zeroed_progress() {
        let (response_tx, mut response_rx) = broadcast::channel(100);
        let mut controller = AudioController::new(response_tx, default_audio_preferences(), true);

        controller.stop_audio();

        let mut got_zeroed_progress = false;
        while let Ok(response) = response_rx.try_recv() {
            if let Some(p) = response.progress.as_ref() {
                if p.song_progress == 0.0 && p.section_progress == 0.0 && p.section_beat == 0.0 {
                    got_zeroed_progress = true;
                }
            }
        }
        assert!(
            got_zeroed_progress,
            "Expected zeroed Progress broadcast after stop_audio"
        );
    }

    #[tokio::test]
    async fn start_stop_start_sets_running_state() {
        let dir = tempdir().unwrap();
        let samples_cache = SamplesCache::new(dir.path());

        let (response_tx, mut response_rx) = broadcast::channel(100);
        let mut controller = AudioController::new(response_tx, default_audio_preferences(), true);
        assert_eq!(*controller.engine_state(), AudioEngineState::Running);

        controller.stop_audio();
        assert_eq!(*controller.engine_state(), AudioEngineState::Stopped);

        controller.start_audio(&samples_cache);
        assert_eq!(*controller.engine_state(), AudioEngineState::Running);

        // After stop, a STOPPED playback broadcast must have been sent.
        let stopped_broadcasts: Vec<_> = std::iter::from_fn(|| response_rx.try_recv().ok())
            .filter_map(|r| r.playback_state.into_option())
            .filter(|ps| ps.playing.enum_value_or_default() == PlayingState::STOPPED)
            .collect();
        assert!(
            !stopped_broadcasts.is_empty(),
            "Expected at least one STOPPED PlaybackState during stop/start cycle"
        );
    }

    #[tokio::test]
    async fn get_audio_status_reflects_running_engine() {
        let controller = test_controller();
        let status = controller.get_audio_status();
        assert_eq!(
            status.engine_status.enum_value_or_default(),
            crate::bloop::AudioEngineStatus::AUDIO_ENGINE_RUNNING
        );
    }

    #[tokio::test]
    async fn stop_audio_broadcasts_audio_status_stopped() {
        let (response_tx, mut response_rx) = broadcast::channel(100);
        let mut controller = AudioController::new(response_tx, default_audio_preferences(), true);

        controller.stop_audio();

        let statuses: Vec<_> = std::iter::from_fn(|| response_rx.try_recv().ok())
            .filter_map(|r| r.audio_status.into_option())
            .collect();

        assert!(
            statuses
                .iter()
                .any(|s| s.engine_status.enum_value_or_default()
                    == crate::bloop::AudioEngineStatus::AUDIO_ENGINE_STOPPED),
            "Expected an AUDIO_ENGINE_STOPPED AudioStatus after stop_audio"
        );
    }

    #[tokio::test]
    async fn start_audio_broadcasts_audio_status_running() {
        let dir = tempdir().unwrap();
        let samples_cache = SamplesCache::new(dir.path());

        let (response_tx, mut response_rx) = broadcast::channel(100);
        let mut controller = AudioController::new(response_tx, default_audio_preferences(), true);

        controller.stop_audio();
        // drain
        while response_rx.try_recv().is_ok() {}

        controller.start_audio(&samples_cache);

        let statuses: Vec<_> = std::iter::from_fn(|| response_rx.try_recv().ok())
            .filter_map(|r| r.audio_status.into_option())
            .collect();

        assert!(
            statuses
                .iter()
                .any(|s| s.engine_status.enum_value_or_default()
                    == crate::bloop::AudioEngineStatus::AUDIO_ENGINE_RUNNING),
            "Expected an AUDIO_ENGINE_RUNNING AudioStatus after start_audio"
        );
    }

    #[tokio::test]
    async fn stop_start_stop_produces_correct_status_sequence() {
        let dir = tempdir().unwrap();
        let samples_cache = SamplesCache::new(dir.path());

        let (response_tx, mut response_rx) = broadcast::channel(100);
        let mut controller = AudioController::new(response_tx, default_audio_preferences(), true);

        controller.stop_audio();
        controller.start_audio(&samples_cache);
        controller.stop_audio();

        let statuses: Vec<_> = std::iter::from_fn(|| response_rx.try_recv().ok())
            .filter_map(|r| r.audio_status.into_option())
            .map(|s| s.engine_status.enum_value_or_default())
            .collect();

        // Must contain at least one STOPPED and one RUNNING in order.
        let first_running = statuses
            .iter()
            .position(|s| *s == crate::bloop::AudioEngineStatus::AUDIO_ENGINE_RUNNING);
        let last_stopped = statuses
            .iter()
            .rposition(|s| *s == crate::bloop::AudioEngineStatus::AUDIO_ENGINE_STOPPED);

        assert!(first_running.is_some(), "No RUNNING status found");
        assert!(last_stopped.is_some(), "No STOPPED status found");
        assert!(
            first_running.unwrap() < last_stopped.unwrap(),
            "Expected RUNNING before final STOPPED"
        );
    }
}
