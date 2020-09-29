use super::id::ID;
use super::state::State;

#[derive(Debug)]
pub struct Sample {
    pub id: ID,
    pub state: State,
    pub path: String,
    pub tempo: f64,
}
