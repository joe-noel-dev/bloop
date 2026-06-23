use log::info;
use std::path::Path;
use tokio::sync::mpsc;

use crate::{bloop::MidiPreferences, model::Action};

pub struct MidiController {}

impl MidiController {
    pub fn new(_action_tx: mpsc::Sender<Action>, _preferences: MidiPreferences, _midi_mappings_dir: &Path) -> Self {
        info!("MIDI feature not enabled");
        Self {}
    }
}
