use crate::bloop::Tempo;

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

    pub fn new_with_bpm(bpm: f64) -> Self {
        assert!(Self::min() <= bpm && bpm <= Self::max());
        let mut tempo = Tempo::new();
        tempo.bpm = 120.0;
        tempo
    }

    pub fn beat_frequency(&self) -> f64 {
        self.bpm / 60.0
    }
}
