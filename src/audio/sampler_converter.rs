use std::path::PathBuf;

use anyhow::Result;
use futures_channel::mpsc;
use rawdio::OwnedAudioBuffer;

use crate::model::ID;

use super::convert::convert_sample;

pub struct SampleConversionResult {
    pub sample_id: ID,
    pub result: Result<OwnedAudioBuffer>,
}

pub struct SampleConverter {
    complete_tx: mpsc::Sender<SampleConversionResult>,
    target_sample_rate: usize,
}

impl SampleConverter {
    pub fn new(complete_tx: mpsc::Sender<SampleConversionResult>, target_sample_rate: usize) -> Self {
        Self {
            complete_tx,
            target_sample_rate,
        }
    }

    pub fn convert(&self, sample_id: ID, sample_path: PathBuf) {
        let mut complete_tx = self.complete_tx.clone();
        let sample_rate = self.target_sample_rate;

        std::thread::spawn(move || {
            let result = convert_sample(&sample_path, sample_rate);
            let _ = complete_tx.try_send(SampleConversionResult { sample_id, result });
        });
    }
}
