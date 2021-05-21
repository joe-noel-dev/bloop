use crate::types::beats::Beats;

pub struct Timeline {
    sample_rate: f64,
    tempo_bpm: f64,
    sample_reference: i64,
    beat_reference: Beats,
}

impl Timeline {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate: f64::from(sample_rate),
            tempo_bpm: 0.0,
            sample_reference: 0,
            beat_reference: Beats::from_num(0.0),
        }
    }

    pub fn set_reference_point(&mut self, beat_position: Beats, sample_position: i64) {
        self.beat_reference = beat_position;
        self.sample_reference = sample_position;
    }

    pub fn set_tempo(&mut self, tempo_bpm: f64) {
        self.tempo_bpm = tempo_bpm;
    }

    pub fn beat_frequency(&self) -> f64 {
        self.tempo_bpm / 60.0
    }

    pub fn beat_position_for_samples(&self, sample_position: i64) -> Beats {
        let sample_offset = sample_position - self.sample_reference;
        self.beat_reference + Beats::from_num(sample_offset) * Beats::from_num(self.beat_frequency() / self.sample_rate)
    }

    pub fn _sample_position_for_beats(&self, beat_position: Beats) -> f64 {
        let beat_offset = beat_position - self.beat_reference;
        self.sample_reference as f64 + beat_offset.to_num::<f64>() * self.sample_rate / self.beat_frequency()
    }
}
