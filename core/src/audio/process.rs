use crate::bloop::AudioPreferences;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, Host, Sample, SampleFormat, SizedSample, Stream, StreamConfig,
};
use log::debug;
use log::{error, info, warn};
use rawdio::{AudioBuffer, AudioProcess, BorrowedAudioBuffer, MutableBorrowedAudioBuffer, OwnedAudioBuffer};

pub trait AudioProcessRunner {}

#[allow(dead_code)]
pub struct Process {
    output_stream: Stream,
    output_channel_count: usize,
}

fn print_output_devices(host: &Host) {
    let mut output = String::from("Output devices: \n");

    let devices = match host.output_devices() {
        Ok(devices) => devices,
        Err(err) => {
            warn!("Unable to enumerate output devices: {err}");
            return;
        }
    };

    devices.for_each(|device| {
        let device_name = match device.description() {
            Ok(description) => description.name().to_string(),
            Err(_) => return,
        };

        output.push_str(format!("{device_name}\n").as_str());
    });

    info!("{output}");
}

struct SelectedStreamConfig {
    config: StreamConfig,
    sample_format: SampleFormat,
}

#[cfg(target_os = "linux")]
fn select_stream_config(preferences: &AudioPreferences, _device: &Device) -> Option<SelectedStreamConfig> {
    Some(SelectedStreamConfig {
        config: StreamConfig {
            channels: preferences.output_channel_count as u16,
            sample_rate: preferences.sample_rate,
            buffer_size: cpal::BufferSize::Fixed(preferences.buffer_size),
        },
        sample_format: SampleFormat::F32,
    })
}

#[cfg(target_os = "android")]
fn select_stream_config(preferences: &AudioPreferences, device: &Device) -> Option<SelectedStreamConfig> {
    let mut fallback = None;

    if let Ok(configs) = device.supported_output_configs() {
        for config in configs {
            if config.channels() < preferences.output_channel_count as u16 {
                continue;
            }

            if config.min_sample_rate() > preferences.sample_rate || config.max_sample_rate() < preferences.sample_rate
            {
                continue;
            }

            let selected = SelectedStreamConfig {
                config: config.with_sample_rate(preferences.sample_rate).config(),
                sample_format: config.sample_format(),
            };

            if config.sample_format() == SampleFormat::F32 {
                return Some(selected);
            }

            if fallback.is_none() {
                fallback = Some(selected);
            }
        }
    }

    if fallback.is_some() {
        return fallback;
    }

    match device.default_output_config() {
        Ok(config) => Some(SelectedStreamConfig {
            config: config.config(),
            sample_format: config.sample_format(),
        }),
        Err(err) => {
            error!("Unable to determine default output config: {err}");
            None
        }
    }
}

#[cfg(all(not(target_os = "linux"), not(target_os = "android")))]
fn select_stream_config(preferences: &AudioPreferences, device: &Device) -> Option<SelectedStreamConfig> {
    let mut fallback = None;

    if let Ok(configs) = device.supported_output_configs() {
        for config in configs {
            if config.channels() < preferences.output_channel_count as u16 {
                continue;
            }

            if config.min_sample_rate() > preferences.sample_rate || config.max_sample_rate() < preferences.sample_rate
            {
                continue;
            }

            let selected = SelectedStreamConfig {
                config: config.with_sample_rate(preferences.sample_rate).config(),
                sample_format: config.sample_format(),
            };

            if config.sample_format() == SampleFormat::F32 {
                return Some(selected);
            }

            if fallback.is_none() {
                fallback = Some(selected);
            }
        }
    }

    if fallback.is_some() {
        return fallback;
    }

    match device.default_output_config() {
        Ok(config) => Some(SelectedStreamConfig {
            config: config.config(),
            sample_format: config.sample_format(),
        }),
        Err(err) => {
            error!("Unable to determine default output config: {err}");
            None
        }
    }
}

fn build_output_stream<T>(
    device: &Device,
    config: &StreamConfig,
    mut audio_process: Box<dyn AudioProcess + Send>,
    preferences: &AudioPreferences,
) -> Result<Stream, String>
where
    T: Sample + SizedSample + cpal::FromSample<f32>,
{
    // Allocate larger buffer size in case the system ignores our requested buffer size.
    let maximum_buffer_size = 8192;
    let input_buffer = OwnedAudioBuffer::new(maximum_buffer_size, 0, preferences.sample_rate as usize);
    let mut output_buffer = OwnedAudioBuffer::new(
        maximum_buffer_size,
        preferences.output_channel_count as usize,
        preferences.sample_rate as usize,
    );
    let stream_channel_count = config.channels as usize;
    let timeout = None;
    let mut interleaved_f32 = vec![0.0f32; maximum_buffer_size * stream_channel_count.max(1)];

    let audio_callback = move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
        if stream_channel_count == 0 {
            return;
        }

        let frame_count = data.len() / stream_channel_count;
        let input_slice = BorrowedAudioBuffer::slice_frames(&input_buffer, 0, frame_count);
        let mut output_slice = MutableBorrowedAudioBuffer::slice_frames(&mut output_buffer, 0, frame_count);

        output_slice.clear();
        audio_process.process(&input_slice, &mut output_slice);

        let required_len = frame_count * stream_channel_count;
        if interleaved_f32.len() < required_len {
            interleaved_f32.resize(required_len, 0.0);
        }

        let write_slice = &mut interleaved_f32[..required_len];
        output_slice.copy_to_interleaved(write_slice, stream_channel_count, frame_count);

        for (dst, src) in data.iter_mut().zip(write_slice.iter()) {
            *dst = T::from_sample(*src);
        }
    };

    let error_callback = move |err| error!("Stream error: {err:?}");

    device
        .build_output_stream(config, audio_callback, error_callback, timeout)
        .map_err(|err| format!("Couldn't create output stream: {err}"))
}

impl Process {
    #[allow(dead_code)]
    pub fn new(audio_process: Box<dyn AudioProcess + Send>, preferences: AudioPreferences) -> Result<Self, String> {
        #[cfg(target_os = "linux")]
        let host = if preferences.use_jack {
            cpal::host_from_id(cpal::HostId::Jack).unwrap_or_else(|_| cpal::default_host())
        } else {
            cpal::default_host()
        };

        #[cfg(not(target_os = "linux"))]
        let host = cpal::default_host();

        info!("Using audio host: {}", host.id().name());

        print_output_devices(&host);

        let preferred_device = if !preferences.output_device.is_empty() {
            match host.output_devices() {
                Ok(mut devices) => devices
                    .find(|device| match device.description() {
                        Ok(description) => description.name().contains(&preferences.output_device),
                        Err(_) => false,
                    })
                    .or_else(|| host.default_output_device()),
                Err(err) => {
                    warn!("Unable to enumerate output devices for selection: {err}");
                    host.default_output_device()
                }
            }
        } else {
            host.default_output_device()
        };

        let Some(device) = preferred_device else {
            return Err("Couldn't connect to output audio device".to_string());
        };
        let device_name = device
            .description()
            .map(|description| description.name().to_string())
            .unwrap_or_else(|_| "unknown".to_string());
        info!("Connecting to device: {}\n", device_name);

        if let Ok(configs) = device.supported_output_configs() {
            configs.for_each(|config| {
                let buffer_size_range = match config.buffer_size() {
                    cpal::SupportedBufferSize::Range { min, max } => (*min, *max),
                    cpal::SupportedBufferSize::Unknown => (0, 0),
                };

                debug!(
                    "Supported output config: channels {}, sample rate {}-{}, buffer size {}-{}, format {:?}",
                    config.channels(),
                    config.min_sample_rate(),
                    config.max_sample_rate(),
                    buffer_size_range.0,
                    buffer_size_range.1,
                    config.sample_format()
                );
            });
        }

        let selected_config = match select_stream_config(&preferences, &device) {
            Some(config) => config,
            None => return Err("Unable to select a usable output stream config".to_string()),
        };
        let config = selected_config.config;

        info!("Preferences buffer size: {}\n", preferences.buffer_size);
        info!("Preferences channel count: {}\n", preferences.output_channel_count);
        info!("Preferences sample rate: {}\n", preferences.sample_rate);

        info!("Config buffer size: {:#?}\n", config.buffer_size);
        info!("Config channel count: {}\n", config.channels);
        info!("Config sample rate: {}\n", config.sample_rate);

        info!("Config sample format: {:?}\n", selected_config.sample_format);

        let stream = match selected_config.sample_format {
            SampleFormat::F32 => build_output_stream::<f32>(&device, &config, audio_process, &preferences),
            SampleFormat::I16 => build_output_stream::<i16>(&device, &config, audio_process, &preferences),
            SampleFormat::U16 => build_output_stream::<u16>(&device, &config, audio_process, &preferences),
            unsupported => Err(format!("Unsupported output sample format: {unsupported:?}")),
        }?;

        stream
            .play()
            .map_err(|err| format!("Couldn't start output stream: {err}"))?;

        Ok(Self {
            output_stream: stream,
            output_channel_count: config.channels as usize,
        })
    }
}
impl AudioProcessRunner for Process {
    // Implementation for real process
}

pub struct NoopProcess;

impl AudioProcessRunner for NoopProcess {
    // Keeps the core alive when realtime audio backend init fails.
}

/// Dummy audio process for testing - runs audio processing without actual hardware
pub struct DummyProcess {
    #[allow(dead_code)]
    audio_thread: std::thread::JoinHandle<()>,
}

impl DummyProcess {
    #[allow(dead_code)]
    pub fn new(mut audio_process: Box<dyn AudioProcess + Send>, preferences: AudioPreferences) -> DummyProcess {
        let input_buffer = OwnedAudioBuffer::new(
            preferences.buffer_size as usize,
            preferences.output_channel_count as usize,
            preferences.sample_rate as usize,
        );

        let mut output_buffer = OwnedAudioBuffer::new(
            preferences.buffer_size as usize,
            preferences.output_channel_count as usize,
            preferences.sample_rate as usize,
        );

        let audio_thread = std::thread::spawn(move || loop {
            let interval = std::time::Duration::from_millis(1);
            std::thread::sleep(interval);
            output_buffer.clear();
            audio_process.process(&input_buffer, &mut output_buffer);
        });

        DummyProcess { audio_thread }
    }
}

impl AudioProcessRunner for DummyProcess {
    // Implementation for dummy process
}

/// Returns `(process_runner, init_error)`. `init_error` is `None` when the
/// real audio device was opened successfully, or `Some(reason)` when
/// initialization failed and the runner fell back to `NoopProcess`.
pub fn create_audio_process(
    audio_process: Box<dyn AudioProcess + Send>,
    preferences: AudioPreferences,
) -> (Box<dyn AudioProcessRunner>, Option<String>) {
    let created = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        Process::new(audio_process, preferences)
    }));

    match created {
        Ok(Ok(process)) => (Box::new(process), None),
        Ok(Err(err)) => {
            error!("Audio backend initialization failed; running without realtime audio output: {err}");
            (Box::new(NoopProcess), Some(err))
        }
        Err(payload) => {
            let panic_message = if let Some(message) = payload.downcast_ref::<&str>() {
                message.to_string()
            } else if let Some(message) = payload.downcast_ref::<String>() {
                message.clone()
            } else {
                "unknown panic payload".to_string()
            };

            error!("Audio backend initialization panicked; running without realtime audio output: {panic_message}");
            (Box::new(NoopProcess), Some(panic_message))
        }
    }
}

/// Returns `(dummy_process_runner, None)` — always succeeds.
pub fn create_dummy_process(
    audio_process: Box<dyn AudioProcess + Send>,
    preferences: AudioPreferences,
) -> (Box<dyn AudioProcessRunner>, Option<String>) {
    (Box::new(DummyProcess::new(audio_process, preferences)), None)
}
