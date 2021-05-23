use std::{collections::HashSet, thread::spawn};

use tokio::sync::broadcast;

use crate::{
    api::response::{Response, WaveformResponse},
    model::id::ID,
    samples::{cache::SamplesCache, sample::CacheState},
    waveform::{
        data::Algorithm,
        generate::{generate_waveform_from_file, Options},
    },
};

pub struct WaveformStore {
    response_tx: broadcast::Sender<Response>,
}

impl WaveformStore {
    pub fn new(response_tx: broadcast::Sender<Response>) -> Self {
        Self { response_tx }
    }

    pub async fn run(&mut self) {}

    pub fn get_waveform(&self, sample_id: &ID, samples_cache: &SamplesCache) -> Result<(), String> {
        let sample = match samples_cache.get_sample(sample_id) {
            Some(sample) => sample,
            None => return Err(format!("Couldn't find sample with ID: {}", sample_id)),
        };

        if *sample.get_cache_state() != CacheState::Cached {
            return Err(format!("Sample is not cached: {}", sample_id));
        }

        let tx = self.response_tx.clone();
        let sample_id = *sample_id;
        let sample_path = sample.get_path().to_path_buf();

        println!("Generating waveform for sample: {}", sample_id);

        spawn(move || {
            let mut lengths = HashSet::new();
            lengths.insert(128);
            lengths.insert(512);
            lengths.insert(2048);
            lengths.insert(8192);

            let mut algorithms = HashSet::new();
            algorithms.insert(Algorithm::Min);
            algorithms.insert(Algorithm::Max);
            algorithms.insert(Algorithm::Rms);

            let options = Options {
                lengths,
                algorithms,
                num_channels: 2,
            };

            let response = match generate_waveform_from_file(&sample_path, options) {
                Ok(waveform_data) => Response::new().with_waveform(WaveformResponse {
                    sample_id,
                    waveform_data,
                }),
                Err(error) => Response::new().with_error(&error),
            };

            tx.send(response).unwrap();
        });

        Ok(())
    }
}
