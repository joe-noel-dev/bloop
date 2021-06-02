use std::cmp::min;

use crate::model::id::ID;

use super::{
    buffer::{AudioBuffer, SampleLocation},
    fade::Fade,
    pool::Pool,
};

pub struct Sampler {
    position: usize,
    sample_id: Option<ID>,

    fade: Fade,
    voices: Vec<Voice>,
    active_voice: Option<usize>,
}

const NUM_VOICES: usize = 4;
const FADE_LENGTH_MS: f32 = 10.0;

impl Default for Sampler {
    fn default() -> Self {
        Self {
            position: 0,
            sample_id: None,
            fade: Fade::new(FADE_LENGTH_MS, 44100),
            voices: (0..NUM_VOICES).map(|_| Voice::default()).collect(),
            active_voice: None,
        }
    }
}

impl Sampler {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            fade: Fade::new(FADE_LENGTH_MS, sample_rate),
            ..Default::default()
        }
    }

    fn use_next_voice(&mut self) {
        self.stop();

        if let Some((index, free_voice)) = self.voices.iter_mut().enumerate().find(|(_, voice)| voice.is_stopped()) {
            free_voice.sample_id = self.sample_id;
            free_voice.position = self.position;
            free_voice.phase = Phase::FadingIn(0);
            self.active_voice = Some(index);
        }
    }

    fn get_active_voice(&self) -> Option<&Voice> {
        if let Some(active_voice_index) = self.active_voice {
            return self.voices.get(active_voice_index);
        }

        None
    }

    pub fn prepare(&mut self, position: usize, sample_id: Option<ID>) {
        self.position = position;
        self.sample_id = sample_id;

        if let Some(active_voice) = self.get_active_voice() {
            if active_voice.position == position && active_voice.sample_id == sample_id {
                return;
            }
        }

        if sample_id.is_some() {
            self.use_next_voice();
        }

        if sample_id.is_none() {
            self.stop();
        }
    }

    pub fn stop(&mut self) {
        self.voices.iter_mut().for_each(|voice| voice.stop());
        self.active_voice = None;
    }

    pub fn render<T, U>(&mut self, output: &mut T, pool: &Pool<U>)
    where
        T: AudioBuffer,
        U: AudioBuffer,
    {
        let fade = &self.fade;
        self.voices
            .iter_mut()
            .for_each(|voice| voice.render(output, pool, fade));
    }
}

#[derive(PartialEq)]
enum Phase {
    Stopped,
    FadingIn(usize),
    Playing,
    FadingOut(usize),
}

struct Voice {
    position: usize,
    sample_id: Option<ID>,
    phase: Phase,
}

impl Default for Voice {
    fn default() -> Self {
        Self {
            position: 0,
            sample_id: None,
            phase: Phase::Stopped,
        }
    }
}

impl Voice {
    fn is_stopped(&self) -> bool {
        self.phase == Phase::Stopped
    }

    fn stop(&mut self) {
        match self.phase {
            Phase::Stopped => (),
            Phase::FadingIn(_) => self.phase = Phase::FadingOut(0),
            Phase::Playing => self.phase = Phase::FadingOut(0),
            Phase::FadingOut(_) => (),
        }
    }

    pub fn render<T, U>(&mut self, output: &mut T, pool: &Pool<U>, fade: &Fade)
    where
        T: AudioBuffer,
        U: AudioBuffer,
    {
        if self.is_stopped() {
            return;
        }

        let mut destination_offset = 0;

        let sample_id = match self.sample_id {
            Some(id) => id,
            None => {
                self.phase = Phase::Stopped;
                return;
            }
        };

        let source = match pool.get(&sample_id) {
            Some(buffer) => buffer,
            None => {
                self.phase = Phase::Stopped;
                return;
            }
        };

        while destination_offset < output.num_frames() {
            match self.phase {
                Phase::Stopped => break,
                Phase::FadingIn(fade_position) => {
                    let num_frames = self.render_fade(output, destination_offset, source, fade, fade_position, true);

                    self.position += num_frames;
                    destination_offset += num_frames;

                    let fade_position = fade_position + num_frames;

                    if fade_position < fade.len() {
                        self.phase = Phase::FadingIn(fade_position);
                    } else {
                        self.phase = Phase::Playing;
                    }
                }
                Phase::Playing => {
                    self.render_playing(output, destination_offset, source);

                    self.position += output.num_frames() - destination_offset;
                    destination_offset = output.num_frames();
                }
                Phase::FadingOut(fade_position) => {
                    let num_frames = self.render_fade(output, destination_offset, source, fade, fade_position, false);

                    self.position += num_frames;
                    destination_offset += num_frames;

                    let fade_position = fade_position + num_frames;
                    if fade_position < fade.len() {
                        self.phase = Phase::FadingOut(fade_position);
                    } else {
                        self.phase = Phase::Stopped;
                    }
                }
            };
        }
    }

    pub fn render_playing<T, U>(&mut self, output: &mut T, destination_offset: usize, source: &U)
    where
        T: AudioBuffer,
        U: AudioBuffer,
    {
        let num_channels = min(source.num_channels(), output.num_channels());

        if self.position >= source.num_frames() {
            return;
        }

        let num_frames = std::cmp::min(
            output.num_frames() - destination_offset,
            source.num_frames() - self.position,
        );

        let source_location = SampleLocation {
            channel: 0,
            frame: self.position,
        };

        let destination_location = SampleLocation {
            channel: 0,
            frame: destination_offset,
        };

        output.add_from(
            source,
            &source_location,
            &destination_location,
            num_channels,
            num_frames,
        );
    }

    pub fn render_fade<T, U>(
        &mut self,
        output: &mut T,
        destination_offset: usize,
        source: &U,
        fade: &Fade,
        fade_position: usize,
        fade_in: bool,
    ) -> usize
    where
        T: AudioBuffer,
        U: AudioBuffer,
    {
        let num_channels = min(source.num_channels(), output.num_channels());

        let num_frames = std::cmp::min(fade.len() - fade_position, output.num_frames() - destination_offset);

        for frame in 0..num_frames {
            if frame + self.position >= source.num_frames() {
                break;
            }

            let fade_value = if fade_in {
                fade.fade_in_value(fade_position + frame)
            } else {
                fade.fade_out_value(fade_position + frame)
            };

            for channel in 0..num_channels {
                let source_location = SampleLocation {
                    channel,
                    frame: self.position + frame,
                };

                let dest_location = SampleLocation {
                    channel,
                    frame: destination_offset + frame,
                };

                let sample = output.get_sample(&dest_location) + fade_value * source.get_sample(&source_location);
                output.set_sample(&dest_location, sample);
            }
        }

        num_frames
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::buffer::OwnedAudioBuffer;
    struct Fixture {
        pub sampler: Sampler,
        pub pool: Pool<OwnedAudioBuffer>,
    }

    impl Default for Fixture {
        fn default() -> Self {
            Self {
                sampler: Sampler::default(),
                pool: Pool::default(),
            }
        }
    }

    impl Fixture {
        fn add_sample(&mut self, num_samples: usize, fill_with_value: f32) -> ID {
            let audio_buffer = Box::new(OwnedAudioBuffer::new(
                (0..num_samples).map(|_| fill_with_value).collect(),
                1,
                44100,
            ));
            let sample_id = ID::new_v4();
            self.pool.add(sample_id, audio_buffer);
            sample_id
        }

        fn render(&mut self, num_samples: usize) -> OwnedAudioBuffer {
            let mut output = OwnedAudioBuffer::new((0..num_samples).map(|_| 0_f32).collect(), 1, 44100);
            self.sampler.render(&mut output, &self.pool);
            output
        }
    }

    #[test]
    fn fades_in() {
        let num_samples = 10000;

        let mut fixture = Fixture::default();
        let sample_id = fixture.add_sample(num_samples, 1.0);

        fixture.sampler.prepare(0, Some(sample_id));
        let output = fixture.render(num_samples);

        approx::assert_relative_eq!(0.0, output.get_sample(&SampleLocation { frame: 0, channel: 0 }));
        approx::assert_relative_eq!(
            0.5,
            output.get_sample(&SampleLocation {
                frame: fixture.sampler.fade.len() / 2,
                channel: 0
            }),
            epsilon = 0.01
        );
        approx::assert_relative_eq!(
            1.0,
            output.get_sample(&SampleLocation {
                frame: fixture.sampler.fade.len(),
                channel: 0
            }),
            epsilon = 0.01
        );
    }

    #[test]
    fn fades_out() {
        let num_samples = 10000;

        let mut fixture = Fixture::default();
        let sample_id = fixture.add_sample(num_samples, 1.0);

        fixture.sampler.prepare(0, Some(sample_id));
        let _ = fixture.render(2 * fixture.sampler.fade.len());
        fixture.sampler.prepare(0, None);
        let output = fixture.render(2 * fixture.sampler.fade.len());

        approx::assert_relative_eq!(1.0, output.get_sample(&SampleLocation { frame: 0, channel: 0 }));
        approx::assert_relative_eq!(
            0.5,
            output.get_sample(&SampleLocation {
                frame: fixture.sampler.fade.len() / 2,
                channel: 0
            }),
            epsilon = 0.01
        );
        approx::assert_relative_eq!(
            0.0,
            output.get_sample(&SampleLocation {
                frame: fixture.sampler.fade.len(),
                channel: 0
            }),
            epsilon = 0.01
        );
    }

    #[test]
    fn crossfades() {
        let num_samples = 10000;

        let mut fixture = Fixture::default();
        let sample_id_1 = fixture.add_sample(num_samples, 1.0);
        let sample_id_2 = fixture.add_sample(num_samples, -1.0);

        fixture.sampler.prepare(0, Some(sample_id_1));
        let _ = fixture.render(2 * fixture.sampler.fade.len());
        fixture.sampler.prepare(0, Some(sample_id_2));

        let output = fixture.render(2 * fixture.sampler.fade.len());

        approx::assert_relative_eq!(1.0, output.get_sample(&SampleLocation { frame: 0, channel: 0 }));
        approx::assert_relative_eq!(
            0.0,
            output.get_sample(&SampleLocation {
                frame: fixture.sampler.fade.len() / 2,
                channel: 0
            }),
            epsilon = 0.01
        );
        approx::assert_relative_eq!(
            -1.0,
            output.get_sample(&SampleLocation {
                frame: fixture.sampler.fade.len(),
                channel: 0
            }),
            epsilon = 0.01
        );
    }

    #[test]
    fn fade_out_beyond_sample() {
        let num_samples = 1000;

        let mut fixture = Fixture::default();
        let sample_id = fixture.add_sample(num_samples, 1.0);

        fixture.sampler.prepare(0, Some(sample_id));
        let _ = fixture.render(num_samples - fixture.sampler.fade.len() / 2);
        fixture.sampler.stop();

        let output = fixture.render(2 * fixture.sampler.fade.len());

        approx::assert_relative_eq!(1.0, output.get_sample(&SampleLocation { frame: 0, channel: 0 }));

        approx::assert_relative_eq!(
            0.0,
            output.get_sample(&SampleLocation {
                frame: fixture.sampler.fade.len(),
                channel: 0
            }),
            epsilon = 0.01
        );
    }

    // TODO: Fade out beyond the end of sample
    // TODO: Re-uses voices
}
