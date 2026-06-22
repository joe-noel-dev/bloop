use cpal::traits::{DeviceTrait, HostTrait};
use log::warn;
use std::collections::HashSet;

use crate::bloop::{AudioDevice, AudioDevices, AudioPreferences};

/// Enumerate all available output devices on the selected cpal host and return
/// them as an `AudioDevices` proto message.
pub fn enumerate_output_devices(preferences: &AudioPreferences) -> AudioDevices {
    #[cfg(target_os = "linux")]
    let host = if preferences.use_jack {
        cpal::host_from_id(cpal::HostId::Jack).unwrap_or_else(|_| cpal::default_host())
    } else {
        cpal::default_host()
    };

    #[cfg(not(target_os = "linux"))]
    let host = cpal::default_host();

    let host_name = host.id().name().to_string();

    let default_device_name = host
        .default_output_device()
        .and_then(|d| d.description().ok())
        .map(|desc| desc.name().to_string());

    let devices = match host.output_devices() {
        Ok(devices) => devices,
        Err(err) => {
            warn!("Unable to enumerate output devices: {err}");
            return AudioDevices {
                host_name,
                ..Default::default()
            };
        }
    };

    let mut proto_devices = Vec::new();

    for device in devices {
        let description = match device.description() {
            Ok(desc) => desc,
            Err(err) => {
                warn!("Skipping device with unreadable description: {err}");
                continue;
            }
        };

        let name = description.name().to_string();
        let is_default = default_device_name.as_deref() == Some(name.as_str());

        let mut supported_sample_rates: HashSet<u32> = HashSet::new();
        let mut supported_channel_counts: HashSet<u32> = HashSet::new();
        let mut min_buffer_size: u32 = 0;
        let mut max_buffer_size: u32 = 0;

        if let Ok(configs) = device.supported_output_configs() {
            for config in configs {
                let min_rate = config.min_sample_rate();
                let max_rate = config.max_sample_rate();

                // Collect the boundary sample rates; downstream code can interpolate.
                supported_sample_rates.insert(min_rate);
                supported_sample_rates.insert(max_rate);

                supported_channel_counts.insert(config.channels() as u32);

                match config.buffer_size() {
                    cpal::SupportedBufferSize::Range { min, max } => {
                        if min_buffer_size == 0 || *min < min_buffer_size {
                            min_buffer_size = *min;
                        }
                        if *max > max_buffer_size {
                            max_buffer_size = *max;
                        }
                    }
                    cpal::SupportedBufferSize::Unknown => {}
                }
            }
        }

        // Include common practical sample rates that fall within the supported range if
        // the device reports a wide continuous range (e.g. CoreAudio).
        let common_rates: &[u32] = &[44100, 48000, 88200, 96000, 176400, 192000];
        let range_lo = supported_sample_rates.iter().copied().min().unwrap_or(0);
        let range_hi = supported_sample_rates.iter().copied().max().unwrap_or(0);
        if range_hi > range_lo {
            for &rate in common_rates {
                if rate >= range_lo && rate <= range_hi {
                    supported_sample_rates.insert(rate);
                }
            }
        }

        let mut supported_sample_rates: Vec<u32> = supported_sample_rates.into_iter().collect();
        let mut supported_channel_counts: Vec<u32> = supported_channel_counts.into_iter().collect();
        supported_sample_rates.sort_unstable();
        supported_channel_counts.sort_unstable();

        proto_devices.push(AudioDevice {
            id: name.clone(),
            name,
            is_default,
            supported_sample_rates,
            supported_channel_counts,
            min_buffer_size,
            max_buffer_size,
            ..Default::default()
        });
    }

    AudioDevices {
        devices: proto_devices,
        host_name,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(not(feature = "audio_device_tests"), ignore = "requires audio_device_tests feature")]
    fn enumerate_output_devices_reports_at_least_one_device() {
        let devices = enumerate_output_devices(&AudioPreferences::default());
        assert!(
            !devices.devices.is_empty(),
            "Expected at least one output device but found none"
        );
    }

    #[test]
    fn enumerate_output_devices_does_not_panic() {
        // Always runs — just ensures enumeration completes without panicking.
        let _ = enumerate_output_devices(&AudioPreferences::default());
    }
}
