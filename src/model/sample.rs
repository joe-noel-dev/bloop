use super::id::ID;
use super::state::State;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Sample {
    pub id: ID,
    pub state: State,
    pub path: String,
    pub tempo: f64,
}
