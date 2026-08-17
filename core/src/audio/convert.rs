use anyhow::{Context, Result};
use hound::SampleFormat;
use log::warn;
use num_traits::pow::Pow;
use rawdio::{AudioBuffer, OwnedAudioBuffer, SampleLocation};
use std::convert::From;
use std::io::ErrorKind;
use std::path::Path;

fn read_samples<S, R>(reader: &mut hound::WavReader<R>, scale: f64, channel_count: usize) -> Result<(Vec<f32>, bool)>
where
    f64: From<S>,
    S: hound::Sample,
    R: std::io::Read,
{
    let mut samples = Vec::new();

    for sample in reader.samples::<S>() {
        match sample {
            Ok(sample) => samples.push((f64::from(sample) / scale) as f32),
            Err(hound::Error::IoError(error))
                if error.kind() == ErrorKind::UnexpectedEof || error.to_string() == "Failed to read enough bytes." =>
            {
                samples.truncate(samples.len() / channel_count * channel_count);
                return Ok((samples, true));
            }
            Err(error) => return Err(error).context("Unable to decode WAV sample data"),
        }
    }

    Ok((samples, false))
}

pub fn convert_sample(sample_path: &Path, target_sample_rate: usize) -> anyhow::Result<OwnedAudioBuffer> {
    let mut reader = hound::WavReader::open(sample_path).context("Unable to open file for conversion")?;

    let spec = reader.spec();
    let channel_count = spec.channels as usize;

    let (samples, truncated) = match spec.sample_format {
        SampleFormat::Float => read_samples::<f32, _>(&mut reader, 1.0, channel_count)?,
        SampleFormat::Int => read_samples::<i32, _>(&mut reader, 2.0_f64.pow(spec.bits_per_sample - 1), channel_count)?,
    };

    if truncated {
        warn!(
            "WAV file {} ended before its declared data chunk; playing the complete frames that were available",
            sample_path.display()
        );
    }

    let frame_count = samples.len() / channel_count;
    let file_sample_rate = spec.sample_rate as usize;

    let mut buffer = OwnedAudioBuffer::new(frame_count, channel_count, file_sample_rate);

    buffer.fill_from_interleaved(&samples, channel_count, frame_count);

    if file_sample_rate == target_sample_rate {
        return Ok(buffer);
    }

    let new_frame_count = (frame_count as f64 * target_sample_rate as f64 / file_sample_rate as f64).ceil() as usize;
    let mut convert_buffer = OwnedAudioBuffer::new(new_frame_count, channel_count, target_sample_rate);

    convert_buffer.sample_rate_convert_from(
        &buffer,
        SampleLocation::origin(),
        SampleLocation::origin(),
        channel_count,
    );

    Ok(convert_buffer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn converts_complete_frames_from_a_truncated_wav() {
        let directory = tempdir().unwrap();
        let sample_path = directory.path().join("truncated.wav");
        let spec = hound::WavSpec {
            channels: 2,
            sample_rate: 48_000,
            bits_per_sample: 24,
            sample_format: SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(&sample_path, spec).unwrap();
        writer.write_sample::<i32>(0).unwrap();
        writer.write_sample::<i32>(0).unwrap();
        writer.write_sample::<i32>(0).unwrap();
        writer.write_sample::<i32>(0).unwrap();
        writer.finalize().unwrap();

        let file = fs::OpenOptions::new().write(true).open(&sample_path).unwrap();
        file.set_len(fs::metadata(&sample_path).unwrap().len() - 1).unwrap();

        convert_sample(&sample_path, 48_000).unwrap();
    }
}
