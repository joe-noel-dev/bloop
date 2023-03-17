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
}

impl SampleConverter {
    pub fn new(complete_tx: mpsc::Sender<SampleConversionResult>) -> Self {
        Self { complete_tx }
    }

    pub fn convert(&self, sample_id: ID, sample_path: PathBuf) {
        let mut complete_tx = self.complete_tx.clone();

        std::thread::spawn(move || {
            let result = convert_sample(&sample_path);
            let _ = complete_tx.try_send(SampleConversionResult { sample_id, result });
        });
    }
}
