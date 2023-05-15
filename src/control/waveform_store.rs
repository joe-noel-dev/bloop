use crate::{
    api::{Response, WaveformResponse},
    model::ID,
    samples::SamplesCache,
    waveform::{generate_waveform_from_file, Algorithm, Options},
};
use anyhow::anyhow;
use std::{collections::HashSet, thread::spawn};
use tokio::sync::broadcast;

pub struct WaveformStore {
    response_tx: broadcast::Sender<Response>,
    samples_being_generated: HashSet<ID>,
    sample_rate: usize,
}

impl WaveformStore {
    pub fn new(response_tx: broadcast::Sender<Response>) -> Self {
        Self {
            response_tx,
            samples_being_generated: HashSet::new(),
            sample_rate: 44_100,
        }
    }

    pub fn get_waveform(&mut self, sample_id: &ID, samples_cache: &SamplesCache) -> anyhow::Result<()> {
        if self.samples_being_generated.contains(sample_id) {
            return Ok(());
        }

        let sample = match samples_cache.get_sample(sample_id) {
            Some(sample) => sample,
            None => return Err(anyhow!("Couldn't find sample with ID: {}", sample_id)),
        };

        if !sample.is_cached() {
            return Err(anyhow!("Sample is not cached: {}", sample_id));
        }

        self.samples_being_generated.insert(*sample_id);

        let tx = self.response_tx.clone();
        let sample_id = *sample_id;
        let sample_path = sample.get_path().to_path_buf();

        println!("Generating waveform for sample: {sample_id}");

        let sample_rate = self.sample_rate;

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
                sample_rate,
            };

            let response = match generate_waveform_from_file(&sample_path, options) {
                Ok(waveform_data) => Response::default().with_waveform(WaveformResponse {
                    sample_id,
                    waveform_data,
                }),
                Err(error) => Response::default().with_error(&error.to_string()),
            };

            tx.send(response).unwrap();
        });

        Ok(())
    }
}
