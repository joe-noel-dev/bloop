use super::id::ID;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
