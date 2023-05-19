use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Debug, Clone, PartialEq)]
pub struct Tempo {
    bpm: f64,
}

impl Default for Tempo {
    fn default() -> Self {
        Self { bpm: 120.0 }
    }
}

impl Tempo {
    pub fn min() -> f64 {
        30.0
    }

    pub fn max() -> f64 {
        300.0
    }

    pub fn get_bpm(&self) -> f64 {
        self.bpm
    }

    pub fn new(bpm: f64) -> Self {
        assert!(Self::min() <= bpm && bpm <= Self::max());
        Self { bpm }
    }

    pub fn beat_frequency(&self) -> f64 {
        self.bpm / 60.0
    }
}
