use super::id::ID;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Selections {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub song: Option<ID>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<ID>,
}

impl Selections {
    pub fn new() -> Self {
        Self {
            song: None,
            section: None,
        }
    }
}
