use log::info;
use std::path::Path;
use tokio::sync::{broadcast, mpsc};

use crate::{bloop::{MidiPreferences, Response}, model::Action};

pub struct MidiController {}

impl MidiController {
    pub fn new(
        _action_tx: mpsc::Sender<Action>,
        _preferences: MidiPreferences,
        _midi_mappings_dir: &Path,
        _response_tx: broadcast::Sender<Response>,
    ) -> Self {
        info!("MIDI feature not enabled");
        Self {}
    }

    pub fn update_preferences(&self, _preferences: MidiPreferences) {}
}

pub fn get_midi_devices() -> crate::bloop::MidiDevices {
    crate::bloop::MidiDevices::default()
}
