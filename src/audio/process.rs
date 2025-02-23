use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Host, Stream, StreamConfig,
};
use log::{debug, error, info};
use rawdio::{AudioBuffer, AudioProcess, BorrowedAudioBuffer, MutableBorrowedAudioBuffer, OwnedAudioBuffer};

use crate::preferences::AudioPreferences;

#[allow(dead_code)]
pub struct Process {
    output_stream: Stream,
    output_channel_count: usize,
}

fn print_output_devices(host: &Host) {
    let mut output = String::from("Output devices: \n");

    host.output_devices().unwrap().for_each(|device| {
        let device_name = match device.name() {
            Ok(name) => name,
            Err(_) => return,
        };

        output.push_str(format!("{device_name}\n").as_str());
    });

    info!("{output}");
}

impl Process {
    pub fn new(mut audio_process: Box<dyn AudioProcess + Send>, preferences: AudioPreferences) -> Self {
        #[cfg(target_os = "linux")]
        let host = cpal::host_from_id(cpal::HostId::Jack).unwrap_or_else(|_| cpal::default_host());

        #[cfg(not(target_os = "linux"))]
        let host = cpal::default_host();

        info!("Using audio host: {}", host.id().name());

        print_output_devices(&host);

        let preferred_device = match preferences.clone().output_device {
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
        info!("Connecting to device: {}\n", device.name().unwrap());

        device
            .supported_output_configs()
            .expect("Unable to get output configurations")
            .for_each(|config| {
                let buffer_size_range = match config.buffer_size() {
                    cpal::SupportedBufferSize::Range { min, max } => (*min, *max),
                    cpal::SupportedBufferSize::Unknown => (0, 0),
                };

                debug!(
                    "Supported output config: channels {}, sample rate {}-{}, buffer size {}-{}, format {:?}",
                    config.channels(),
                    config.min_sample_rate().0,
                    config.max_sample_rate().0,
                    buffer_size_range.0,
                    buffer_size_range.1,
                    config.sample_format()
                );
            });

        info!("Buffer size: {}\n", preferences.buffer_size);
        info!("Channel count: {}\n", preferences.output_channel_count);
        info!("Sample rate: {}\n", preferences.sample_rate);

        let config = StreamConfig {
            channels: preferences.output_channel_count as u16,
            sample_rate: cpal::SampleRate(preferences.sample_rate as u32),
            buffer_size: cpal::BufferSize::Fixed(preferences.buffer_size as u32),
        };

        // Allocate larger buffer size in case the system ignores our requested buffer size
        let buffer_size = preferences.buffer_size * 2;

        let input_buffer = OwnedAudioBuffer::new(buffer_size, 0, preferences.sample_rate);

        let mut output_buffer =
            OwnedAudioBuffer::new(buffer_size, preferences.output_channel_count, preferences.sample_rate);

        let timeout = None;

        let channel_count = preferences.output_channel_count;

        let audio_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let frame_count = data.len() / channel_count;

            let input_slice = BorrowedAudioBuffer::slice_frames(&input_buffer, 0, frame_count);
            let mut output_slice = MutableBorrowedAudioBuffer::slice_frames(&mut output_buffer, 0, frame_count);

            output_slice.clear();

            audio_process.process(&input_slice, &mut output_slice);

            output_slice.copy_to_interleaved(data, channel_count, frame_count);
        };

        let error_callback = move |err| error!("Stream error: {err:?}");

        let stream = device
            .build_output_stream(&config, audio_callback, error_callback, timeout)
            .expect("Couldn't create output stream");

        stream.play().expect("Couldn't start output stream");

        Self {
            output_stream: stream,
            output_channel_count: preferences.output_channel_count,
        }
    }
}
