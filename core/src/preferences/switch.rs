use serde::{Deserialize, Serialize};

use crate::model::Action;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
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

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SwitchPreferences {
    #[serde(default)]
    pub mappings: Vec<SwitchMapping>,
}
