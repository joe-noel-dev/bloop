use super::{Mapping, MidiDeviceMapping};
use crate::bloop::Action;
use crate::midi::matcher::ExactMatcher;
use regex::Regex;

pub const DEVICE_REGEX: &str = "FootCtrl Bluetooth";

pub fn device_mapping() -> MidiDeviceMapping {
    MidiDeviceMapping {
        device_regex: Regex::new(DEVICE_REGEX).expect("valid regex"),
        mappings: vec![
            Mapping {
                matcher: Box::new(ExactMatcher::new(&[0xB0, 0x28, 0x7F])),
                action: Action::ACTION_PREVIOUS_SONG,
            },
            Mapping {
                matcher: Box::new(ExactMatcher::new(&[0xB0, 0x29, 0x7F])),
                action: Action::ACTION_NEXT_SONG,
            },
            Mapping {
                matcher: Box::new(ExactMatcher::new(&[0xB0, 0x2E, 0x7F])),
                action: Action::ACTION_TOGGLE_LOOP,
            },
            Mapping {
                matcher: Box::new(ExactMatcher::new(&[0xB0, 0x2F, 0x7F])),
                action: Action::ACTION_TOGGLE_PLAY,
            },
        ],
    }
}
