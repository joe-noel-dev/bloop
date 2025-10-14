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

    /// Channel offset for main audio output (e.g., 0 for channels 1-2, 2 for channels 3-4).
    /// This determines which output channels the main audio (samplers) will be routed to.
    #[serde(default = "default_main_channel_offset")]
    pub main_channel_offset: usize,

    /// Channel offset for click/metronome output (e.g., 0 for channels 1-2, 2 for channels 3-4).
    /// This determines which output channels the metronome will be routed to.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_audio_preferences() {
        let prefs = AudioPreferences::default();
        assert_eq!(prefs.sample_rate, 48_000);
        assert_eq!(prefs.buffer_size, 512);
        assert_eq!(prefs.output_channel_count, 2);
        assert_eq!(prefs.main_channel_offset, 0);
        assert_eq!(prefs.click_channel_offset, 2);
        assert_eq!(prefs.use_jack, false);
    }

    #[test]
    fn test_serde_with_defaults() {
        // Test that missing fields use defaults
        let json = r#"{}"#;
        let prefs: AudioPreferences = serde_json::from_str(json).unwrap();
        assert_eq!(prefs.sample_rate, 48_000);
        assert_eq!(prefs.main_channel_offset, 0);
        assert_eq!(prefs.click_channel_offset, 2);
    }

    #[test]
    fn test_serde_with_custom_offsets() {
        // Test serialization/deserialization with custom offsets
        let json = r#"{"mainChannelOffset": 2, "clickChannelOffset": 4}"#;
        let prefs: AudioPreferences = serde_json::from_str(json).unwrap();
        assert_eq!(prefs.main_channel_offset, 2);
        assert_eq!(prefs.click_channel_offset, 4);
    }
}

