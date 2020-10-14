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
