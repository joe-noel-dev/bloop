use super::{
    buffer::BorrowedAudioBuffer, command::Command, engine::AudioEngine, engine::Engine, notification::Notification,
};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SampleRate, Stream,
};
use futures_channel::mpsc::{Receiver, Sender};

const SAMPLE_RATE: u32 = 44100;

pub struct Process {
    _output_stream: Stream,
}

impl Process {
    pub fn new(command_rx: Receiver<Command>, notification_tx: Sender<Notification>) -> Self {
        let host = cpal::default_host();
        println!("Using audio host: {}", host.id().name());

        let device = host.default_output_device().expect("No output device available");
        println!("Connecting to device: {}", device.name().unwrap());
        println!();

        let mut output_configs = device.supported_output_configs().unwrap();
        let config = output_configs
            .next()
            .expect("No configs supported")
            .with_sample_rate(SampleRate(SAMPLE_RATE));

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

        stream.play().unwrap();

        Self { _output_stream: stream }
    }
}
