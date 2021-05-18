use cpal::traits::{DeviceTrait, HostTrait};

pub struct Process {}

impl Process {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("No output device available");

        let stream_config = device.default_output_config().expect("error while querying configs");

        let _stream = device.build_output_stream(
            &stream_config.config(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for sample in data.iter_mut() {
                    *sample = 0.0;
                }
            },
            move |_err| {
                // react to errors here.
            },
        );

        Self {}
    }
}
