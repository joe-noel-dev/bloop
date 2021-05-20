pub trait AudioBuffer {
    fn num_channels(&self) -> usize;
    fn num_frames(&self) -> usize;
    fn sample_rate(&self) -> u32;
    fn clear(&mut self);
}

pub struct BorrowedAudioBuffer<'a> {
    data: &'a mut [f32],
    num_channels: usize,
    sample_rate: u32,
}

impl<'a> BorrowedAudioBuffer<'a> {
    pub fn new(data: &'a mut [f32], num_channels: usize, sample_rate: u32) -> Self {
        Self {
            data,
            num_channels,
            sample_rate,
        }
    }
}

impl<'a> AudioBuffer for BorrowedAudioBuffer<'a> {
    fn num_channels(&self) -> usize {
        self.num_channels
    }

    fn num_frames(&self) -> usize {
        self.data.len() / self.num_channels
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn clear(&mut self) {
        for value in self.data.iter_mut() {
            *value = 0.0;
        }
    }
}
