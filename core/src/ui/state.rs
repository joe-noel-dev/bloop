use tokio::sync::{broadcast, mpsc};

use crate::{
    bloop::{AudioDevices, AudioStatus, MidiDevices, Preferences, Request, Response},
    model::{PlaybackState, Progress, Project},
};

use super::settings::SettingsUiState;

pub struct State {
    pub response_tx: broadcast::Sender<Response>,
    pub request_tx: mpsc::Sender<Request>,
    pub project: Project,
    pub playback_state: PlaybackState,
    pub progress: Progress,
    pub preferences: Option<Preferences>,
    pub audio_devices: Option<AudioDevices>,
    pub audio_status: Option<AudioStatus>,
    pub midi_devices: Option<MidiDevices>,
    pub settings: SettingsUiState,
}

impl State {
    pub fn new(response_tx: broadcast::Sender<Response>, request_tx: mpsc::Sender<Request>) -> Self {
        Self {
            response_tx,
            request_tx,
            project: Default::default(),
            playback_state: Default::default(),
            progress: Default::default(),
            preferences: None,
            audio_devices: None,
            audio_status: None,
            midi_devices: None,
            settings: SettingsUiState::default(),
        }
    }
}
