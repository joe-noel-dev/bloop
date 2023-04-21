use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Debug, Clone, PartialEq)]
pub struct Tempo {
    pub bpm: f64,
}
