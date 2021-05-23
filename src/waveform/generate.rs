use std::{borrow::Borrow, collections::HashSet, sync::Arc, thread::spawn};

use crate::audio::buffer::{AudioBuffer, ImmutableAudioBufferSlice, OwnedAudioBuffer, SampleLocation};

use super::data::{Algorithm, Properties, WaveformData};
use std::convert::TryInto;

#[derive(Clone)]
pub struct Options {
    lengths: HashSet<i32>,
    algorithms: HashSet<Algorithm>,
    num_channels: i32,
}

pub fn generate_waveform_from_audio(audio: OwnedAudioBuffer, mut options: Options) -> Result<WaveformData, String> {
    let min_num_peaks = 10;
    let num_frames: i32 = audio.num_frames().try_into().unwrap();
    let max_peak_length: i32 = num_frames / min_num_peaks;

    options.lengths = options
        .lengths
        .iter()
        .filter(|length| **length <= max_peak_length)
        .copied()
        .collect();

    let audio: Arc<OwnedAudioBuffer> = Arc::from(audio);

    let data = WaveformData::new(audio.sample_rate().try_into().unwrap());

    let data = options
        .lengths
        .iter()
        .map(|peak_length| {
            let mut options = options.clone();
            options.lengths = HashSet::new();
            options.lengths.insert(*peak_length);

            let audio = audio.clone();

            spawn(move || process_waveform(options, audio))
        })
        .fold(data, |mut data, handle| {
            data.add(handle.join().unwrap());
            data
        });

    Ok(data)
}

fn process_waveform(options: Options, audio: Arc<dyn AudioBuffer>) -> WaveformData {
    let mut data = WaveformData::new(audio.sample_rate().try_into().unwrap());

    for length in options.lengths.iter() {
        let length: usize = (*length).try_into().unwrap();
        for frame in (0..audio.num_frames()).step_by(length) {
            let slice = ImmutableAudioBufferSlice::new(audio.borrow(), frame);

            for channel in 0..options.num_channels {
                process_channel(
                    &mut data,
                    &slice,
                    &options.algorithms,
                    length,
                    channel.try_into().unwrap(),
                )
            }
        }
    }

    data
}

fn process_channel(
    waveform: &mut WaveformData,
    audio: &dyn AudioBuffer,
    algorithms: &HashSet<Algorithm>,
    length: usize,
    channel: usize,
) {
    let mut min_sample = 0.0_f32;
    let mut max_sample = 0.0_f32;
    let mut squared_total = 0.0_f64;

    for frame in 0..length {
        let sample = audio.get_sample(&SampleLocation { channel, frame });
        min_sample = min_sample.min(sample);
        max_sample = max_sample.max(sample);
        squared_total += sample as f64 * sample as f64;
    }

    if algorithms.contains(&Algorithm::Min) {
        waveform.push(
            &Properties {
                length: length.try_into().unwrap(),
                algorithm: Algorithm::Min,
                channel: channel.try_into().unwrap(),
            },
            min_sample,
        );
    }

    if algorithms.contains(&Algorithm::Max) {
        waveform.push(
            &Properties {
                length: length.try_into().unwrap(),
                algorithm: Algorithm::Max,
                channel: channel.try_into().unwrap(),
            },
            max_sample,
        );
    }

    if algorithms.contains(&Algorithm::Rms) {
        let mean_squared = squared_total / length as f64;
        let rms = mean_squared.sqrt();
        waveform.push(
            &Properties {
                length: length.try_into().unwrap(),
                algorithm: Algorithm::Rms,
                channel: channel.try_into().unwrap(),
            },
            rms as f32,
        );
    }
}
