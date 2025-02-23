use serde::{Deserialize, Serialize};

use crate::model::Action;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Gesture {
    Press,
    Release,
    Hold,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SwitchMapping {
    pub pin: u8,
    pub gesture: Gesture,
    pub action: Action,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SwitchPreferences {
    #[serde(default)]
    pub mappings: Vec<SwitchMapping>,
}

impl Default for SwitchPreferences {
    fn default() -> Self {
        Self {
            mappings: vec![
                SwitchMapping {
                    pin: 17,
                    gesture: Gesture::Press,
                    action: Action::ToggleLoop,
                },
                SwitchMapping {
                    pin: 27,
                    gesture: Gesture::Release,
                    action: Action::NextSong,
                },
                SwitchMapping {
                    pin: 27,
                    gesture: Gesture::Hold,
                    action: Action::PreviousSong,
                },
                SwitchMapping {
                    pin: 22,
                    gesture: Gesture::Press,
                    action: Action::TogglePlay,
                },
            ],
        }
    }
}
