use super::id::ID;
use super::state::State;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub id: ID,
    pub state: State,
    pub name: String,
    pub mute: bool,
    pub solo: bool,
    pub colour: String,
}
