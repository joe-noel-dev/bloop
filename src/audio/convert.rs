use crate::audio::buffer::OwnedAudioBuffer;
use hound::SampleFormat;
use num_traits::pow::Pow;
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

pub fn convert_sample(sample_path: &Path) -> Result<Box<OwnedAudioBuffer>, String> {
    println!("Converting sample @ {}", sample_path.display());
    let mut reader = match hound::WavReader::open(sample_path) {
        Ok(reader) => reader,
        Err(error) => return Err(format!("Error reading audio file: {}", error)),
    };

    let spec = reader.spec();

    if spec.sample_rate != 44100 {
        return Err("Only samples at 44.1 kHz are supported at present".to_string());
    }

    let samples = match spec.sample_format {
        SampleFormat::Float => read_samples::<f32, _>(&mut reader, 1.0),
        SampleFormat::Int => read_samples::<i32, _>(&mut reader, 2.0_f64.pow(spec.bits_per_sample)),
    };

    Ok(Box::new(OwnedAudioBuffer::new(
        samples,
        spec.channels.into(),
        spec.sample_rate,
    )))
}
