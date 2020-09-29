use super::id::ID;
use super::state::State;

#[derive(Debug)]
pub struct Channel {
    pub id: ID,
    pub state: State,
    pub name: String,
    pub mute: bool,
    pub solo: bool,
    pub colour: String,
}
