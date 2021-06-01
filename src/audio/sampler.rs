use std::cmp::min;

use crate::model::id::ID;

use super::{
    buffer::{AudioBuffer, SampleLocation},
    pool::Pool,
};

pub struct Sampler {
    position: usize,
    sample_id: Option<ID>,
    playing: bool,
}

impl Default for Sampler {
    fn default() -> Self {
        Self {
            position: 0,
            sample_id: None,
            playing: false,
        }
    }
}

impl Sampler {
    pub fn set_position(&mut self, position: usize) {
        self.position = position
    }

    pub fn play(&mut self) {
        self.playing = true
    }

    pub fn stop(&mut self) {
        self.playing = false
    }

    pub fn set_sample_id(&mut self, sample_id: &ID) {
        self.sample_id = Some(*sample_id)
    }

    pub fn clear_sample_id(&mut self) {
        self.sample_id = None
    }

    pub fn render<T, U>(&mut self, output: &mut T, pool: &Pool<U>)
    where
        T: AudioBuffer,
        U: AudioBuffer,
    {
        if !self.playing {
            return;
        }

        let position = self.position;
        self.position += output.num_frames();

        let sample_id = match self.sample_id {
            Some(id) => id,
            None => return,
        };

        let source = match pool.get(&sample_id) {
            Some(buffer) => buffer,
            None => return,
        };

        let num_channels = min(source.num_channels(), output.num_channels());

        if position >= source.num_frames() {
            return;
        }

        let mut end_position = position + output.num_frames();
        end_position = min(end_position, source.num_frames());

        if end_position < position {
            return;
        }

        let num_frames = end_position - position;

        let source_location = SampleLocation {
            channel: 0,
            frame: position,
        };

        let destination_location = SampleLocation { channel: 0, frame: 0 };

        output.add_from(
            source,
            &source_location,
            &destination_location,
            num_channels,
            num_frames,
        );
    }
}
