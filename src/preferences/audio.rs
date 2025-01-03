use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AudioPreferences {
    #[serde(default)]
    pub output_device: Option<String>,

    #[serde(default = "default_sample_rate")]
    pub sample_rate: usize,

    #[serde(default = "default_buffer_size")]
    pub buffer_size: usize,

    #[serde(default = "default_output_channel_count")]
    pub output_channel_count: usize,
}

fn default_sample_rate() -> usize {
    48_000
}

fn default_buffer_size() -> usize {
    512
}

fn default_output_channel_count() -> usize {
    2
}

impl Default for AudioPreferences {
    fn default() -> Self {
        Self {
            output_device: None,
            sample_rate: default_sample_rate(),
            buffer_size: default_buffer_size(),
            output_channel_count: default_output_channel_count(),
        }
    }
}
