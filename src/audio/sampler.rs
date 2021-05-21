use std::cmp::min;

use super::buffer::{AudioBuffer, SampleLocation};

pub struct Sampler {
    position: usize,
    playing: bool,
    end_position: Option<usize>,
}

impl Sampler {
    pub fn new() -> Self {
        Sampler {
            position: 0,
            playing: false,
            end_position: None,
        }
    }

    pub fn set_position(&mut self, position: usize) {
        self.position = position
    }

    pub fn set_end_position(&mut self, end_position: usize) {
        self.end_position = Some(end_position)
    }

    pub fn play(&mut self) {
        self.playing = true
    }

    pub fn stop(&mut self) {
        self.playing = false
    }

    pub fn render<T, U>(&mut self, output: &mut T, source: &U) -> usize
    where
        T: AudioBuffer,
        U: AudioBuffer,
    {
        if !self.playing {
            return 0;
        }

        let num_channels = min(source.num_channels(), output.num_channels());

        if self.position >= source.num_frames() {
            return 0;
        }

        let mut end_position = self.position + output.num_frames();

        end_position = min(end_position, source.num_frames());

        if let Some(stop) = self.end_position {
            end_position = min(end_position, stop);
        }

        if end_position < self.position {
            return 0;
        }

        let num_frames = end_position - self.position;

        let source_location = SampleLocation {
            channel: 0,
            frame: self.position,
        };

        let destination_location = SampleLocation { channel: 0, frame: 0 };

        output.add_from(
            source,
            &source_location,
            &destination_location,
            num_channels,
            num_frames,
        );

        self.position += num_frames;
        num_frames
    }
}
