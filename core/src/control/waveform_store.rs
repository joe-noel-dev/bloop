use crate::{
    bloop::{Response, WaveformAlgorithm, WaveformResponse},
    model::ID,
    samples::SamplesCache,
    waveform::{generate_waveform_from_file, Options},
};
use anyhow::anyhow;
use log::info;
use std::{collections::HashSet, sync::mpsc, thread::spawn};
use tokio::sync::broadcast;

pub struct WaveformStore {
    response_tx: broadcast::Sender<Response>,
    samples_being_generated: HashSet<ID>,
    sample_rate: usize,
    complete_channel_rx: mpsc::Receiver<ID>,
    complete_channel_tx: mpsc::Sender<ID>,
}

impl WaveformStore {
    pub fn new(response_tx: broadcast::Sender<Response>) -> Self {
        let (complete_channel_tx, complete_channel_rx) = mpsc::channel();

        Self {
            response_tx,
            samples_being_generated: HashSet::new(),
            sample_rate: 44_100,
            complete_channel_rx,
            complete_channel_tx,
        }
    }

    pub fn get_waveform(&mut self, sample_id: ID, samples_cache: &SamplesCache) -> anyhow::Result<()> {
        while let Ok(completed_id) = self.complete_channel_rx.try_recv() {
            self.samples_being_generated.remove(&completed_id);
        }

        if self.samples_being_generated.contains(&sample_id) {
            return Ok(());
        }

        let sample = match samples_cache.get_sample(sample_id) {
            Some(sample) => sample,
            None => return Err(anyhow!("Couldn't find sample with ID: {sample_id}")),
        };

        if !sample.is_cached() {
            return Err(anyhow!("Sample is not cached: {sample_id}"));
        }

        self.samples_being_generated.insert(sample_id);

        let tx = self.response_tx.clone();
        let sample_path = sample.get_path().to_path_buf();

        info!("Generating waveform for sample: {sample_id}");

        let sample_rate = self.sample_rate;

        let complete_tx = self.complete_channel_tx.clone();

        spawn(move || {
            let mut lengths = HashSet::new();
            lengths.insert(512);
            lengths.insert(8192);

            let mut algorithms = HashSet::new();
            algorithms.insert(WaveformAlgorithm::MIN);
            algorithms.insert(WaveformAlgorithm::MAX);

            let options = Options {
                lengths,
                algorithms,
                num_channels: 0,
                sample_rate,
            };

            let response = match generate_waveform_from_file(&sample_path, options) {
                Ok(waveform_data) => Response::default().with_waveform(&WaveformResponse {
                    sample_id,
                    waveform_data: Some(waveform_data).into(),
                    ..Default::default()
                }),
                Err(error) => Response::default().with_error(&error.to_string()),
            };

            tx.send(response).unwrap();

            let _ = complete_tx.send(sample_id);
        });

        Ok(())
    }
}
