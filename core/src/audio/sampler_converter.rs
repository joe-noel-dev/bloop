use std::{path::PathBuf, sync::mpsc as std_mpsc};

use anyhow::Result;
use futures_channel::mpsc;
use rawdio::OwnedAudioBuffer;

use crate::model::ID;

use super::convert::convert_sample;

pub struct SampleConversionResult {
    pub sample_id: ID,
    pub generation: u64,
    pub result: Result<OwnedAudioBuffer>,
}

struct SampleConversionJob {
    sample_id: ID,
    generation: u64,
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
                    generation: job.generation,
                    result,
                });
            }
        });

        Self { job_tx }
    }

    pub fn convert(&self, sample_id: ID, generation: u64, sample_path: PathBuf) {
        let _ = self.job_tx.send(SampleConversionJob {
            sample_id,
            generation,
            sample_path,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;

    #[tokio::test]
    async fn failed_conversion_returns_cache_generation() {
        let dir = tempfile::tempdir().unwrap();
        let (tx, mut rx) = mpsc::channel(1);
        let converter = SampleConverter::new(tx, 44100);
        converter.convert(1, 42, dir.path().join("missing.wav"));

        let result = rx.next().await.unwrap();
        assert_eq!(result.sample_id, 1);
        assert_eq!(result.generation, 42);
        assert!(result.result.is_err());
    }
}
