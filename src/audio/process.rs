use super::{
    buffer::BorrowedAudioBuffer, command::Command, engine::AudioEngine, engine::Engine, notification::Notification,
};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Host, SampleRate, Stream,
};
use futures_channel::mpsc::{Receiver, Sender};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, path::Path};

const SAMPLE_RATE: u32 = 44100;

pub struct Process {
    _output_stream: Stream,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Preferences {
    #[serde(skip_serializing_if = "Option::is_none")]
    output_device: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sample_rate: Option<u32>,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            output_device: None,
            sample_rate: None,
        }
    }
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

        println!("{}", device_name);
    });
    println!();
}

impl Process {
    pub fn new(command_rx: Receiver<Command>, notification_tx: Sender<Notification>, preferences_dir: &Path) -> Self {
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

        let mut engine = AudioEngine::new(command_rx, notification_tx);

        let stream = device
            .build_output_stream(
                &config.config(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let mut audio_buffer =
                        BorrowedAudioBuffer::new(data, usize::from(config.channels()), config.sample_rate().0);
                    engine.render(&mut audio_buffer);
                },
                move |err| eprintln!("Stream error: {:?}", err),
            )
            .expect("Couldn't create output stream");

        stream.play().expect("Couldn't start output stream");

        Self { _output_stream: stream }
    }
}
