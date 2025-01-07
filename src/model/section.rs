use super::id::ID;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Section {
    pub id: ID,
    pub name: String,
    pub start: f64,
    #[serde(rename = "loop")]
    pub looping: bool,

    #[serde(default)]
    pub metronome: bool,
}

impl Default for Section {
    fn default() -> Self {
        Section {
            id: ID::new_v4(),
            name: "Section".to_string(),
            start: 0.0,
            looping: false,
            metronome: false,
        }
    }
}

impl Section {
    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn with_start(mut self, start: f64) -> Self {
        self.start = start;
        self
    }

    pub fn with_looping(mut self, looping: bool) -> Self {
        self.looping = looping;
        self
    }

    pub fn with_metronome(mut self, metronome: bool) -> Self {
        self.metronome = metronome;
        self
    }

    pub fn is_valid(&self) -> bool {
        self.start >= 0.0
    }

    pub fn replace_ids(mut self) -> Self {
        self.id = ID::new_v4();
        self
    }
}
