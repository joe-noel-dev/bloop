use crate::bloop::*;
use log::info;
use std::{fs::File, io::Read, path::Path};

pub fn read_preferences_from_str(preferences_str: &str) -> anyhow::Result<Preferences> {
    let mut preferences = default_preferences();
    let parse_options = protobuf_json_mapping::ParseOptions {
        ignore_unknown_fields: true,
        ..Default::default()
    };
    protobuf_json_mapping::merge_from_str_with_options(&mut preferences, preferences_str, &parse_options)?;
    validate_preferences(&mut preferences);
    Ok(preferences)
}

fn validate_preferences(preferences: &mut Preferences) {
    if let Some(audio_prefs) = preferences.audio.as_mut() {
        if audio_prefs.output_channel_count == 0 || audio_prefs.output_channel_count > 64 {
            info!(
                "Invalid output channel count of {}, resetting to 2",
                audio_prefs.output_channel_count
            );
            audio_prefs.output_channel_count = 2;
        }

        if audio_prefs.buffer_size == 0 || audio_prefs.buffer_size > 8192 {
            info!("Invalid buffer size of {}, resetting to 512", audio_prefs.buffer_size);
            audio_prefs.buffer_size = 512;
        }

        if audio_prefs.sample_rate == 0 || audio_prefs.sample_rate > 192_000 {
            info!("Invalid sample rate of {}, resetting to 48000", audio_prefs.sample_rate);
            audio_prefs.sample_rate = 48_000;
        }
    }
}

pub fn read_preferences(preferences_dir: &Path) -> anyhow::Result<Preferences> {
    let mut preferences_path = preferences_dir.to_path_buf();
    preferences_path.push("preferences.json");

    info!("Reading preferences from {preferences_path:?}");

    let mut file = File::open(preferences_path)?;
    let mut json = String::new();
    file.read_to_string(&mut json)?;
    read_preferences_from_str(&json)
}

fn default_preferences() -> Preferences {
    Preferences {
        audio: Some(AudioPreferences {
            output_device: String::new(),
            sample_rate: 48_000,
            buffer_size: 512,
            output_channel_count: 2,
            use_jack: false,
            main_channel_offset: 0,
            click_channel_offset: 2,
            ..Default::default()
        })
        .into(),
        midi: None.into(),
        switch: None.into(),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_json_uses_defaults() {
        let prefs = read_preferences_from_str("{}").unwrap();

        let audio_prefs = prefs.audio.unwrap();
        assert_eq!(audio_prefs.sample_rate, 48_000);
        assert_eq!(audio_prefs.buffer_size, 512);
        assert_eq!(audio_prefs.output_channel_count, 2);
        assert_eq!(audio_prefs.main_channel_offset, 0);
        assert_eq!(audio_prefs.click_channel_offset, 2);
        assert!(!audio_prefs.use_jack);
    }

    #[test]
    fn test_serialize_and_deserialize_with_defaults() {
        // Test that missing fields use defaults
        let json = r#"{"audio": {}}"#;
        let prefs = read_preferences_from_str(json).unwrap();
        let audio_prefs = prefs.audio.unwrap();
        assert_eq!(audio_prefs.sample_rate, 48_000);
        assert_eq!(audio_prefs.buffer_size, 512);
        assert_eq!(audio_prefs.main_channel_offset, 0);
    }

    #[test]
    fn test_serialize_and_deserialize_with_custom_offsets() {
        // Test serialization/deserialization with custom offsets
        let json = r#"{ "audio": {"mainChannelOffset": 2, "clickChannelOffset": 4}}"#;
        let prefs = read_preferences_from_str(json).unwrap();
        let audio_prefs = prefs.audio.unwrap();
        assert_eq!(audio_prefs.main_channel_offset, 2);
        assert_eq!(audio_prefs.click_channel_offset, 4);
    }
}
