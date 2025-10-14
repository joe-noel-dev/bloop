use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AudioPreferences {
    #[serde(default)]
    pub output_device: Option<String>,

    #[serde(default = "default_sample_rate")]
    pub sample_rate: usize,

    #[serde(default = "default_buffer_size")]
    pub buffer_size: usize,

    #[serde(default = "default_output_channel_count")]
    pub output_channel_count: usize,

    #[serde(default)]
    pub use_jack: bool,

    #[serde(default = "default_main_channel_offset")]
    pub main_channel_offset: usize,

    #[serde(default = "default_click_channel_offset")]
    pub click_channel_offset: usize,
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

fn default_main_channel_offset() -> usize {
    0
}

fn default_click_channel_offset() -> usize {
    2
}

impl Default for AudioPreferences {
    fn default() -> Self {
        Self {
            output_device: None,
            sample_rate: default_sample_rate(),
            buffer_size: default_buffer_size(),
            output_channel_count: default_output_channel_count(),
            use_jack: false,
            main_channel_offset: default_main_channel_offset(),
            click_channel_offset: default_click_channel_offset(),
        }
    }
}
