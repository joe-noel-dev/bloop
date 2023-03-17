use anyhow::{anyhow, Context};
use hound::SampleFormat;
use num_traits::pow::Pow;
use rawdio::{AudioBuffer, OwnedAudioBuffer};
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

pub fn convert_sample(sample_path: &Path) -> anyhow::Result<OwnedAudioBuffer> {
    let mut reader = hound::WavReader::open(sample_path).context("Unable to open file for conversion")?;

    let spec = reader.spec();

    if spec.sample_rate != 44100 {
        return Err(anyhow!("Only samples at 44.1 kHz are supported at present"));
    }

    let samples = match spec.sample_format {
        SampleFormat::Float => read_samples::<f32, _>(&mut reader, 1.0),
        SampleFormat::Int => read_samples::<i32, _>(&mut reader, 2.0_f64.pow(spec.bits_per_sample - 1)),
    };

    let channel_count = spec.channels as usize;
    let frame_count = samples.len() / channel_count;
    let sample_rate = spec.sample_rate as usize;

    let mut buffer = OwnedAudioBuffer::new(frame_count, channel_count, sample_rate);
    buffer.fill_from_interleaved(&samples, channel_count, frame_count);

    Ok(buffer)
}
