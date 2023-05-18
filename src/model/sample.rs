use super::{id::ID, tempo::Tempo};
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Sample {
    pub id: ID,
    pub name: String,
    pub tempo: Tempo,
    pub sample_rate: i32,
    pub sample_count: i64,
    pub channel_count: i32,
}

impl Sample {
    pub fn new() -> Self {
        Sample {
            id: ID::new_v4(),
            name: "".to_string(),
            tempo: Tempo::new(120.0),
            sample_rate: 0,
            sample_count: 0,
            channel_count: 0,
        }
    }

    #[cfg(test)]
    pub fn with_beat_length(mut self, tempo: Tempo, beat_length: f64, sample_rate: i32) -> Self {
        let seconds = beat_length / tempo.beat_frequency();
        let samples = (seconds * sample_rate as f64).ceil() as i64;

        self.sample_rate = sample_rate;
        self.sample_count = samples;
        self.tempo = tempo;

        self
    }

    pub fn new_with_id(id: &ID) -> Self {
        let mut sample = Sample::new();
        sample.id = *id;
        sample
    }

    pub fn is_valid(&self) -> bool {
        self.sample_rate > 0 && self.sample_count > 0
    }

    pub fn beat_length(&self) -> f64 {
        (self.sample_count as f64 * self.tempo.beat_frequency()) / self.sample_rate as f64
    }
}
