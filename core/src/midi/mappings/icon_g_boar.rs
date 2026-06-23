use super::{Mapping, MidiDeviceMapping};
use crate::bloop::Action;
use crate::midi::matcher::ExactMatcher;
use regex::Regex;

pub const DEVICE_REGEX: &str = "iCON G_Boar";

pub fn device_mapping() -> MidiDeviceMapping {
    MidiDeviceMapping {
        device_regex: Regex::new(DEVICE_REGEX).expect("valid regex"),
        mappings: vec![
            Mapping {
                matcher: Box::new(ExactMatcher::new(&[176, 40, 127])),
                action: Action::ACTION_PREVIOUS_SONG,
            },
            Mapping {
                matcher: Box::new(ExactMatcher::new(&[176, 41, 127])),
                action: Action::ACTION_NEXT_SONG,
            },
            Mapping {
                matcher: Box::new(ExactMatcher::new(&[176, 42, 127])),
                action: Action::ACTION_QUEUE_SELECTED,
            },
            Mapping {
                matcher: Box::new(ExactMatcher::new(&[176, 44, 127])),
                action: Action::ACTION_PREVIOUS_SECTION,
            },
            Mapping {
                matcher: Box::new(ExactMatcher::new(&[176, 45, 127])),
                action: Action::ACTION_NEXT_SECTION,
            },
            Mapping {
                matcher: Box::new(ExactMatcher::new(&[176, 46, 127])),
                action: Action::ACTION_TOGGLE_LOOP,
            },
            Mapping {
                matcher: Box::new(ExactMatcher::new(&[176, 47, 127])),
                action: Action::ACTION_TOGGLE_PLAY,
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn icon_g_boar_has_seven_mappings() {
        let dm = device_mapping();
        assert_eq!(dm.mappings.len(), 7);
    }

    #[test]
    fn regex_matches_full_device_name() {
        let dm = device_mapping();
        assert!(dm.device_regex.is_match("iCON G_Boar V1.03"));
    }

    #[test]
    fn regex_does_not_match_generic_device() {
        let dm = device_mapping();
        assert!(!dm.device_regex.is_match("Generic MIDI"));
    }
}
