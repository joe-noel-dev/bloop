mod audio;
mod midi;
mod switch;

pub use audio::AudioPreferences;
use log::info;
pub use midi::MidiPreferences;
#[allow(unused_imports)]
pub use switch::{Gesture, SwitchMapping, SwitchPreferences};

use std::{fs::File, io::BufReader, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Preferences {
    #[serde(default)]
    pub audio: Option<AudioPreferences>,

    #[serde(default)]
    pub midi: Option<MidiPreferences>,

    #[serde(default)]
    pub switch: Option<SwitchPreferences>,
}

pub fn read_preferences(preferences_dir: &Path) -> anyhow::Result<Preferences> {
    let mut preferences_path = preferences_dir.to_path_buf();
    preferences_path.push("preferences.json");

    info!("Reading preferences from {preferences_path:?}");

    let file = File::open(preferences_path)?;
    let reader = BufReader::new(file);
    let preferences = serde_json::from_reader(reader)?;
    Ok(preferences)
}
