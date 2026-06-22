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

/// Selects the preferred output device from `host` based on the name in `device_name`.
/// Falls back to the system default when the name is empty or not found.
fn select_output_device(host: &Host, device_name: &str) -> Option<Device> {
    if !device_name.is_empty() {
        match host.output_devices() {
            Ok(mut devices) => devices
                .find(|d| {
                    d.description()
                        .ok()
                        .map(|desc| desc.name().contains(device_name))
                        .unwrap_or(false)
                })
                .or_else(|| host.default_output_device()),
            Err(err) => {
                warn!("Unable to enumerate output devices for selection: {err}");
                host.default_output_device()
            }
        }
    } else {
        host.default_output_device()
    }
}

/// Selects the preferred channel count for `device` at the given `sample_rate`.
///
/// Returns the smallest supported channel count that satisfies `min_required`.
/// Falls back to 2 channels, or the smallest available count if `min_required`
/// cannot be met.
fn preferred_channel_count_for_device(device: &Device, sample_rate: u32, min_required: usize) -> usize {
    let mut counts: Vec<u16> = device
        .supported_output_configs()
        .ok()
        .into_iter()
        .flatten()
        .filter(|c| c.min_sample_rate() <= sample_rate && c.max_sample_rate() >= sample_rate)
        .map(|c| c.channels())
        .collect();
    counts.sort_unstable();
    counts.dedup();

    if counts.is_empty() {
        return device
            .default_output_config()
            .map(|c| c.channels() as usize)
            .unwrap_or(2);
    }

    if let Some(&best) = counts.iter().find(|&&c| c as usize >= min_required) {
        return best as usize;
    }
    if counts.contains(&2) {
        return 2;
    }
    counts[0] as usize
}

/// Returns the preferred output channel count for the device selected by `preferences`.
/// Selects the smallest supported channel count that satisfies the routing requirements
/// for the configured main and click channel offsets.
pub fn query_native_channel_count(preferences: &AudioPreferences) -> usize {
    #[cfg(target_os = "linux")]
    let host = if preferences.use_jack {
        cpal::host_from_id(cpal::HostId::Jack).unwrap_or_else(|_| cpal::default_host())
    } else {
        cpal::default_host()
    };

    #[cfg(not(target_os = "linux"))]
    let host = cpal::default_host();

    let Some(device) = select_output_device(&host, &preferences.output_device) else {
        return 2;
    };

    let min_required = (preferences.main_channel_offset as usize + 2)
        .max(preferences.click_channel_offset as usize + 2);
    preferred_channel_count_for_device(&device, preferences.sample_rate, min_required)
}

#[cfg(target_os = "linux")]
fn select_stream_config(
    preferences: &AudioPreferences,
    _device: &Device,
    channel_count: usize,
) -> Option<SelectedStreamConfig> {
    Some(SelectedStreamConfig {
        config: StreamConfig {
            channels: channel_count as u16,
            sample_rate: preferences.sample_rate,
            buffer_size: cpal::BufferSize::Fixed(preferences.buffer_size),
        },
        sample_format: SampleFormat::F32,
    })
}

#[cfg(target_os = "android")]
fn select_stream_config(
    preferences: &AudioPreferences,
    device: &Device,
    channel_count: usize,
) -> Option<SelectedStreamConfig> {
    select_stream_config_for_channel_count(preferences, device, channel_count)
}

#[cfg(all(not(target_os = "linux"), not(target_os = "android")))]
fn select_stream_config(
    preferences: &AudioPreferences,
    device: &Device,
    channel_count: usize,
) -> Option<SelectedStreamConfig> {
    select_stream_config_for_channel_count(preferences, device, channel_count)
}

/// Platform-independent config selection: prefers `channel_count` channels at
/// `preferences.sample_rate`, falling back gracefully when unavailable.
fn select_stream_config_for_channel_count(
    preferences: &AudioPreferences,
    device: &Device,
    channel_count: usize,
) -> Option<SelectedStreamConfig> {
    let mut channel_match: Option<SelectedStreamConfig> = None;
    let mut f32_rate_match: Option<SelectedStreamConfig> = None;
    let mut rate_match: Option<SelectedStreamConfig> = None;

    if let Ok(configs) = device.supported_output_configs() {
        for config in configs {
            if config.min_sample_rate() > preferences.sample_rate
                || config.max_sample_rate() < preferences.sample_rate
            {
                continue;
            }

            let is_channel_match = config.channels() as usize == channel_count;
            let selected = SelectedStreamConfig {
                config: config.with_sample_rate(preferences.sample_rate).config(),
                sample_format: config.sample_format(),
            };

            if is_channel_match && config.sample_format() == SampleFormat::F32 {
                return Some(selected); // optimal
            }
            if is_channel_match && channel_match.is_none() {
                channel_match = Some(selected);
            } else if config.sample_format() == SampleFormat::F32 && f32_rate_match.is_none() {
                f32_rate_match = Some(selected);
            } else if rate_match.is_none() {
                rate_match = Some(selected);
            }
        }
    }

    channel_match.or(f32_rate_match).or(rate_match).or_else(|| {
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
    })
}

fn build_output_stream<T>(
    device: &Device,
    config: &StreamConfig,
    mut audio_process: Box<dyn AudioProcess + Send>,
) -> Result<Stream, String>
where
    T: Sample + SizedSample + cpal::FromSample<f32>,
{
    // Allocate larger buffer size in case the system ignores our requested buffer size.
    let maximum_buffer_size = 8192;
    let channel_count = config.channels as usize;
    let sample_rate = config.sample_rate as usize;
    let input_buffer = OwnedAudioBuffer::new(maximum_buffer_size, 0, sample_rate);
    let mut output_buffer = OwnedAudioBuffer::new(maximum_buffer_size, channel_count, sample_rate);
    let timeout = None;
    let mut interleaved_f32 = vec![0.0f32; maximum_buffer_size * channel_count.max(1)];

    let audio_callback = move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
        if channel_count == 0 {
            return;
        }

        let frame_count = data.len() / channel_count;
        let input_slice = BorrowedAudioBuffer::slice_frames(&input_buffer, 0, frame_count);
        let mut output_slice = MutableBorrowedAudioBuffer::slice_frames(&mut output_buffer, 0, frame_count);

        output_slice.clear();
        audio_process.process(&input_slice, &mut output_slice);

        let required_len = frame_count * channel_count;
        if interleaved_f32.len() < required_len {
            interleaved_f32.resize(required_len, 0.0);
        }

        let write_slice = &mut interleaved_f32[..required_len];
        output_slice.copy_to_interleaved(write_slice, channel_count, frame_count);

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

        let Some(device) = select_output_device(&host, &preferences.output_device) else {
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

        let min_required = (preferences.main_channel_offset as usize + 2)
            .max(preferences.click_channel_offset as usize + 2);
        let channel_count = preferred_channel_count_for_device(&device, preferences.sample_rate, min_required);
        info!("Selected channel count: {}\n", channel_count);

        let selected_config = match select_stream_config(&preferences, &device, channel_count) {
            Some(config) => config,
            None => return Err("Unable to select a usable output stream config".to_string()),
        };
        let config = selected_config.config;

        info!("Preferences buffer size: {}\n", preferences.buffer_size);
        info!("Preferences sample rate: {}\n", preferences.sample_rate);

        info!("Config buffer size: {:#?}\n", config.buffer_size);
        info!("Config channel count: {}\n", config.channels);
        info!("Config sample rate: {}\n", config.sample_rate);

        info!("Config sample format: {:?}\n", selected_config.sample_format);

        let stream = match selected_config.sample_format {
            SampleFormat::F32 => build_output_stream::<f32>(&device, &config, audio_process),
            SampleFormat::I16 => build_output_stream::<i16>(&device, &config, audio_process),
            SampleFormat::U16 => build_output_stream::<u16>(&device, &config, audio_process),
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
    pub fn new(
        mut audio_process: Box<dyn AudioProcess + Send>,
        preferences: AudioPreferences,
        channel_count: usize,
    ) -> DummyProcess {
        let input_buffer = OwnedAudioBuffer::new(
            preferences.buffer_size as usize,
            channel_count,
            preferences.sample_rate as usize,
        );

        let mut output_buffer = OwnedAudioBuffer::new(
            preferences.buffer_size as usize,
            channel_count,
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

/// Returns the realtime audio process runner when the backend initialises
/// successfully, or an error reason when it fails.
pub fn create_audio_process(
    audio_process: Box<dyn AudioProcess + Send>,
    preferences: AudioPreferences,
) -> Result<Box<dyn AudioProcessRunner>, String> {
    let created = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        Process::new(audio_process, preferences)
    }));

    match created {
        Ok(Ok(process)) => Ok(Box::new(process)),
        Ok(Err(err)) => {
            error!("Audio backend initialization failed; running without realtime audio output: {err}");
            Err(err)
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
            Err(panic_message)
        }
    }
}

/// Creates a dummy process runner. This always succeeds.
pub fn create_dummy_process(
    audio_process: Box<dyn AudioProcess + Send>,
    preferences: AudioPreferences,
    channel_count: usize,
) -> Box<dyn AudioProcessRunner> {
    Box::new(DummyProcess::new(audio_process, preferences, channel_count))
}
