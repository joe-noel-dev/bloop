use super::id::ID;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub id: ID,
    pub name: String,
    pub mute: bool,
    pub solo: bool,
    pub colour: String,
}

impl Channel {
    pub fn new() -> Self {
        Self {
            id: ID::new_v4(),
            name: "Channel".to_string(),
            mute: false,
            solo: false,
            colour: "white".to_string(),
        }
    }

    pub fn _with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
}
