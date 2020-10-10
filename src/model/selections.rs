use super::id::ID;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Selections {
    pub song: Option<ID>,
    pub section: Option<ID>,
    pub channel: Option<ID>,
}

impl Selections {
    pub fn new() -> Self {
        Self {
            song: None,
            section: None,
            channel: None,
        }
    }
}
