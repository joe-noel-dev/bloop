use log::info;
use tokio::sync::mpsc;

use crate::{model::Action, preferences::MidiPreferences};

pub struct MidiController {}

impl MidiController {
    pub fn new(_action_tx: mpsc::Sender<Action>, _preferences: MidiPreferences) -> Self {
        info!("MIDI feature not enabled");
        Self {}
    }
}
