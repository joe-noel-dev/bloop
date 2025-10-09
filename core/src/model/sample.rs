use super::{id::ID, random_id, Tempo};
use crate::bloop::Sample;

impl Sample {
    pub fn empty() -> Self {
        Self {
            id: random_id(),
            name: "".to_string(),
            tempo: Some(Tempo::new_with_bpm(120.0)).into(),
            sample_rate: 0,
            sample_count: 0,
            channel_count: 0,
            ..Default::default()
        }
    }

    #[cfg(test)]
    pub fn with_beat_length(mut self, tempo: Tempo, beat_length: f64, sample_rate: i32) -> Self {
        let seconds = beat_length / tempo.beat_frequency();
        let samples = (seconds * sample_rate as f64).ceil() as i64;

        self.sample_rate = sample_rate;
        self.sample_count = samples;
        self.tempo = Some(tempo).into();

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
