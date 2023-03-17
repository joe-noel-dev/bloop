use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Host, SampleRate, Stream,
};
use rawdio::{AudioBuffer, AudioProcess, BorrowedAudioBuffer, MutableBorrowedAudioBuffer, OwnedAudioBuffer};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, path::Path};

const SAMPLE_RATE: u32 = 44100;

pub struct Process {
    _output_stream: Stream,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
struct Preferences {
    #[serde(skip_serializing_if = "Option::is_none")]
    output_device: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_rate: Option<u32>,
}

fn read_preferences(preferences_dir: &Path) -> anyhow::Result<Preferences> {
    let mut preferences_path = preferences_dir.to_path_buf();
    preferences_path.push("audio.json");

    let file = File::open(preferences_path)?;
    let reader = BufReader::new(file);
    let preferences = serde_json::from_reader(reader)?;
    Ok(preferences)
}

fn print_output_devices(host: &Host) {
    println!("Output devices: ");
    host.output_devices().unwrap().for_each(|device| {
        let device_name = match device.name() {
            Ok(name) => name,
            Err(_) => return,
        };

        println!("{device_name}");
    });
    println!();
}

impl Process {
    pub fn new(mut audio_process: Box<dyn AudioProcess + Send>, preferences_dir: &Path) -> Self {
        let host = cpal::default_host();
        println!("Using audio host: {}", host.id().name());

        print_output_devices(&host);

        let preferences = read_preferences(preferences_dir).unwrap_or_default();

        let preferred_device = match preferences.output_device {
            Some(preferred_device_name) => host
                .output_devices()
                .unwrap()
                .find(|device| match device.name() {
                    Ok(device_name) => device_name.contains(&preferred_device_name),
                    Err(_) => false,
                })
                .or_else(|| host.default_output_device()),
            None => host.default_output_device(),
        };

        let device = preferred_device.expect("Couldn't connect to output audio device");
        println!("Connecting to device: {}", device.name().unwrap());
        println!();

        let mut output_configs = device.supported_output_configs().unwrap();
        let config = output_configs
            .next()
            .expect("No configs supported")
            .with_sample_rate(SampleRate(preferences.sample_rate.unwrap_or(SAMPLE_RATE)));

        println!("Sample rate: {}", config.sample_rate().0);
        println!();

        let maximum_frame_count = match config.buffer_size() {
            cpal::SupportedBufferSize::Range { min: _, max } => *max as usize,
            cpal::SupportedBufferSize::Unknown => 4096,
        };

        let channel_count = config.channels() as usize;
        let sample_rate = config.sample_rate().0 as usize;

        let input_buffer = OwnedAudioBuffer::new(maximum_frame_count, 0, sample_rate);
        let mut output_buffer = OwnedAudioBuffer::new(maximum_frame_count, channel_count, sample_rate);

        let stream = device
            .build_output_stream(
                &config.config(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let frame_count = data.len() / channel_count;

                    let input_slice = BorrowedAudioBuffer::slice_frames(&input_buffer, 0, frame_count);
                    let mut output_slice = MutableBorrowedAudioBuffer::slice_frames(&mut output_buffer, 0, frame_count);

                    output_slice.clear();

                    audio_process.process(&input_slice, &mut output_slice);

                    output_slice.copy_to_interleaved(data, channel_count, frame_count);
                },
                move |err| eprintln!("Stream error: {err:?}"),
            )
            .expect("Couldn't create output stream");

        stream.play().expect("Couldn't start output stream");

        Self { _output_stream: stream }
    }
}
