use super::random_id;
use crate::bloop;

impl bloop::Section {
    pub fn empty() -> Self {
        Self {
            id: random_id(),
            name: "Section".to_string(),
            start: 0.0,
            loop_: false,
            metronome: false,
            ..Default::default()
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn with_start(mut self, start: f64) -> Self {
        self.start = start;
        self
    }

    pub fn with_loop(mut self, loop_: bool) -> Self {
        self.loop_ = loop_;
        self
    }

    pub fn with_metronome(mut self, metronome: bool) -> Self {
        self.metronome = metronome;
        self
    }

    pub fn is_valid(&self) -> bool {
        self.start >= 0.0
    }

    pub fn replace_ids(mut self) -> Self {
        self.id = random_id();
        self
    }
}
