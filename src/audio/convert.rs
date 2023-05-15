use anyhow::Context;
use hound::SampleFormat;
use num_traits::pow::Pow;
use rawdio::{AudioBuffer, OwnedAudioBuffer, SampleLocation};
use std::convert::From;
use std::path::Path;

fn read_samples<S, R>(reader: &mut hound::WavReader<R>, scale: f64) -> Vec<f32>
where
    f64: From<S>,
    S: hound::Sample,
    R: std::io::Read,
{
    reader
        .samples::<S>()
        .map(|s| f64::from(s.unwrap()))
        .map(|value| (value / scale) as f32)
        .collect()
}

pub fn convert_sample(sample_path: &Path, target_sample_rate: usize) -> anyhow::Result<OwnedAudioBuffer> {
    let mut reader = hound::WavReader::open(sample_path).context("Unable to open file for conversion")?;

    let spec = reader.spec();

    let samples = match spec.sample_format {
        SampleFormat::Float => read_samples::<f32, _>(&mut reader, 1.0),
        SampleFormat::Int => read_samples::<i32, _>(&mut reader, 2.0_f64.pow(spec.bits_per_sample - 1)),
    };

    let channel_count = spec.channels as usize;
    let frame_count = samples.len() / channel_count;
    let file_sample_rate = spec.sample_rate as usize;

    let mut buffer = OwnedAudioBuffer::new(frame_count, channel_count, file_sample_rate);

    buffer.fill_from_interleaved(&samples, channel_count, frame_count);

    if file_sample_rate == target_sample_rate {
        return Ok(buffer);
    }

    let new_frame_count = (frame_count as f64 * target_sample_rate as f64 / file_sample_rate as f64).ceil() as usize;
    let mut convert_buffer = OwnedAudioBuffer::new(new_frame_count, channel_count, target_sample_rate);
    convert_buffer.sample_rate_convert_from(&buffer, SampleLocation::origin(), SampleLocation::origin());
    Ok(convert_buffer)
}
