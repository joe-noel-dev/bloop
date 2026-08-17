use std::{path::PathBuf, sync::mpsc as std_mpsc};

use anyhow::Result;
use futures_channel::mpsc;
use rawdio::OwnedAudioBuffer;

use crate::model::ID;

use super::convert::convert_sample;

pub struct SampleConversionResult {
    pub sample_id: ID,
    pub result: Result<OwnedAudioBuffer>,
}

struct SampleConversionJob {
    sample_id: ID,
    sample_path: PathBuf,
}

pub struct SampleConverter {
    job_tx: std_mpsc::Sender<SampleConversionJob>,
}

impl SampleConverter {
    pub fn new(complete_tx: mpsc::Sender<SampleConversionResult>, target_sample_rate: usize) -> Self {
        let (job_tx, job_rx) = std_mpsc::channel::<SampleConversionJob>();

        std::thread::spawn(move || {
            let mut complete_tx = complete_tx;
            while let Ok(job) = job_rx.recv() {
                let result = convert_sample(&job.sample_path, target_sample_rate);
                let _ = complete_tx.try_send(SampleConversionResult {
                    sample_id: job.sample_id,
                    result,
                });
            }
        });

        Self { job_tx }
    }

    pub fn convert(&self, sample_id: ID, sample_path: PathBuf) {
        let _ = self.job_tx.send(SampleConversionJob { sample_id, sample_path });
    }
}
