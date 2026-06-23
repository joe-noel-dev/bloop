use crate::bloop::Action;
use crate::midi::matcher::{ExactMatcher, Matcher};
use anyhow::{anyhow, Result};
use log::warn;
use regex::Regex;
use std::path::Path;

mod icon_g_boar;

/// A single MIDI message-to-action binding.
pub struct Mapping {
    matcher: Box<dyn Matcher + Send>,
    pub action: Action,
}

impl Mapping {
    pub fn matches(&self, message: &[u8]) -> bool {
        self.matcher.matches(message)
    }
}

/// A set of [`Mapping`]s scoped to MIDI ports whose name matches `device_regex`.
pub struct MidiDeviceMapping {
    pub device_regex: Regex,
    pub mappings: Vec<Mapping>,
}

/// Returns all device mappings: compiled-in defaults merged with any valid `.json`
/// files found in `midi_mappings_dir`.
///
/// # User mapping file format
///
/// Place `.json` files in `$BLOOP_HOME/midi_mappings/`. Each file describes one
/// device and its button-to-action bindings:
///
/// ```json
/// {
///   "device_regex": "iCON G_Boar",
///   "mappings": [
///     { "message": [176, 40, 127], "action": "ACTION_PREVIOUS_SONG" },
///     { "message": [176, 41, 127], "action": "ACTION_NEXT_SONG" }
///   ]
/// }
/// ```
///
/// | Field          | Type            | Description                                              |
/// |----------------|-----------------|----------------------------------------------------------|
/// | `device_regex` | string (regex)  | Matched against the MIDI port name; case-sensitive.      |
/// | `mappings`     | array           | One entry per button/control.                            |
/// | `message`      | array of u8     | Raw MIDI bytes to match exactly (e.g. `[176, 40, 127]`). |
/// | `action`       | string          | One of the action names listed below.                    |
///
/// ## Valid action names
///
/// - `ACTION_PREVIOUS_SONG`
/// - `ACTION_NEXT_SONG`
/// - `ACTION_PREVIOUS_SECTION`
/// - `ACTION_NEXT_SECTION`
/// - `ACTION_QUEUE_SELECTED`
/// - `ACTION_TOGGLE_LOOP`
/// - `ACTION_TOGGLE_PLAY`
///
/// Files that are not valid JSON, contain an invalid regex, or reference an
/// unknown action are skipped with a warning; all other files still load.
pub fn load_mappings(midi_mappings_dir: &Path) -> Vec<MidiDeviceMapping> {
    let mut result = vec![icon_g_boar::device_mapping()];

    let entries = match std::fs::read_dir(midi_mappings_dir) {
        Ok(entries) => entries,
        Err(_) => return result,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        match load_mapping_file(&path) {
            Ok(mapping) => result.push(mapping),
            Err(err) => warn!("Skipping MIDI mapping file {}: {}", path.display(), err),
        }
    }

    result
}

#[derive(serde::Deserialize)]
struct FileMidiMapping {
    device_regex: String,
    mappings: Vec<FileMapping>,
}

#[derive(serde::Deserialize)]
struct FileMapping {
    message: Vec<u8>,
    action: String,
}

fn parse_action(s: &str) -> Option<Action> {
    match s {
        "ACTION_UNKNOWN" => Some(Action::ACTION_UNKNOWN),
        "ACTION_PREVIOUS_SONG" => Some(Action::ACTION_PREVIOUS_SONG),
        "ACTION_NEXT_SONG" => Some(Action::ACTION_NEXT_SONG),
        "ACTION_PREVIOUS_SECTION" => Some(Action::ACTION_PREVIOUS_SECTION),
        "ACTION_NEXT_SECTION" => Some(Action::ACTION_NEXT_SECTION),
        "ACTION_QUEUE_SELECTED" => Some(Action::ACTION_QUEUE_SELECTED),
        "ACTION_TOGGLE_LOOP" => Some(Action::ACTION_TOGGLE_LOOP),
        "ACTION_TOGGLE_PLAY" => Some(Action::ACTION_TOGGLE_PLAY),
        _ => None,
    }
}

fn load_mapping_file(path: &Path) -> Result<MidiDeviceMapping> {
    let content = std::fs::read_to_string(path)?;
    let file_mapping: FileMidiMapping = serde_json::from_str(&content)?;

    let device_regex = Regex::new(&file_mapping.device_regex)
        .map_err(|e| anyhow!("Invalid regex '{}': {}", file_mapping.device_regex, e))?;

    let mappings = file_mapping
        .mappings
        .into_iter()
        .map(|fm| {
            let action = parse_action(&fm.action).ok_or_else(|| anyhow!("Unknown action '{}'", fm.action))?;
            Ok(Mapping {
                matcher: Box::new(ExactMatcher::new(&fm.message)),
                action,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(MidiDeviceMapping { device_regex, mappings })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn load_mappings_returns_defaults_when_dir_missing() {
        let dir = TempDir::new().unwrap();
        let missing = dir.path().join("nonexistent");
        let mappings = load_mappings(&missing);
        assert_eq!(mappings.len(), 1);
    }

    #[test]
    fn load_mappings_skips_malformed_file() {
        let dir = TempDir::new().unwrap();

        std::fs::write(
            dir.path().join("custom.json"),
            r#"{"device_regex":"Test Device","mappings":[{"message":[176,10,127],"action":"ACTION_TOGGLE_PLAY"}]}"#,
        )
        .unwrap();

        std::fs::write(dir.path().join("bad.json"), r#"not valid json"#).unwrap();

        let mappings = load_mappings(dir.path());
        assert_eq!(mappings.len(), 2); // 1 built-in + 1 valid user file
    }

    #[test]
    fn load_mappings_skips_non_json_files() {
        let dir = TempDir::new().unwrap();
        std::fs::write(dir.path().join("ignore.txt"), r#"irrelevant"#).unwrap();
        let mappings = load_mappings(dir.path());
        assert_eq!(mappings.len(), 1);
    }

    #[test]
    fn load_mappings_skips_unknown_action_file() {
        let dir = TempDir::new().unwrap();
        std::fs::write(
            dir.path().join("unknown-action.json"),
            r#"{"device_regex":"Test Device","mappings":[{"message":[176,10,127],"action":"ACTION_NOT_REAL"}]}"#,
        )
        .unwrap();

        let mappings = load_mappings(dir.path());
        assert_eq!(mappings.len(), 1);
    }

    #[test]
    fn load_mappings_skips_invalid_regex_file() {
        let dir = TempDir::new().unwrap();
        std::fs::write(
            dir.path().join("invalid-regex.json"),
            r#"{"device_regex":"(","mappings":[{"message":[176,10,127],"action":"ACTION_TOGGLE_PLAY"}]}"#,
        )
        .unwrap();

        let mappings = load_mappings(dir.path());
        assert_eq!(mappings.len(), 1);
    }
}
