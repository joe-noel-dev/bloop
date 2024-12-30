use std::{fs::File, io::BufReader, path::Path};

use serde::{Deserialize, Serialize};

fn default_sample_rate() -> usize {
    48_000
}

fn default_buffer_size() -> usize {
    512
}

fn default_output_channel_count() -> usize {
    2
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Preferences {
    #[serde(default)]
    pub output_device: Option<String>,

    #[serde(default = "default_sample_rate")]
    pub sample_rate: usize,

    #[serde(default = "default_buffer_size")]
    pub buffer_size: usize,

    #[serde(default = "default_output_channel_count")]
    pub output_channel_count: usize,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            output_device: None,
            sample_rate: default_sample_rate(),
            buffer_size: default_buffer_size(),
            output_channel_count: default_output_channel_count(),
        }
    }
}

pub fn read_preferences(preferences_dir: &Path) -> anyhow::Result<Preferences> {
    let mut preferences_path = preferences_dir.to_path_buf();
    preferences_path.push("audio.json");

    let file = File::open(preferences_path)?;
    let reader = BufReader::new(file);
    let preferences = serde_json::from_reader(reader)?;
    Ok(preferences)
}
