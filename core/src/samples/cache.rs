use super::sample::Sample;
use crate::bloop::AudioFileFormat;
use crate::{model::ID, types::extension_for_format};
use anyhow::{anyhow, Context};
use log::{debug, warn};
use std::{
    collections::HashMap,
    ffi::OsStr,
    path::{Path, PathBuf},
};
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

pub struct SamplesCache {
    root_directory: PathBuf,
    samples: HashMap<ID, Sample>,
    generation: u64,
}

pub struct SampleMetadata {
    pub name: String,
    pub sample_rate: u32,
    pub sample_count: u32,
    pub num_channels: u32,
    pub detected_tempo: Option<f64>,
}

impl SamplesCache {
    pub fn new(root_directory: &Path) -> Self {
        if !root_directory.exists() {
            std::fs::create_dir_all(root_directory)
                .unwrap_or_else(|_| panic!("Couldn't create directory: {}", root_directory.to_str().unwrap()));
        }

        Self {
            root_directory: PathBuf::from(root_directory),
            samples: HashMap::new(),
            generation: 0,
        }
    }

    pub fn begin_upload(&mut self, id: ID, format: AudioFileFormat, filename: &str) {
        self.samples.remove(&id);
        let mut sample = self.new_sample(filename);
        let path = self.path_for_sample(id, format);
        let tmp_path = self.tmp_path_for_sample(id, format);
        // Remove any leftover incomplete upload from a previous interrupted attempt.
        if tmp_path.is_file() {
            if let Err(e) = std::fs::remove_file(&tmp_path) {
                warn!(
                    "Failed to remove stale temp sample file ({}): {}",
                    tmp_path.display(),
                    e
                );
            }
        }
        sample.set_cache_location(&path);
        sample.set_tmp_location(&tmp_path);
        self.samples.insert(id, sample);
    }

    pub async fn upload(&mut self, id: ID, data: &[u8]) -> anyhow::Result<()> {
        debug!("Received bytes for upload id={}, length={}", id, data.len());
        let sample = self.samples.get(&id).ok_or_else(|| anyhow!("Sample not found: {id}"))?;
        let tmp_path = sample.get_tmp_path().to_path_buf();
        self.write_to_file(data, &tmp_path).await?;
        Ok(())
    }

    pub fn complete_upload(&mut self, id: ID) -> anyhow::Result<()> {
        let sample = self
            .samples
            .get_mut(&id)
            .ok_or_else(|| anyhow!("Sample not found: {id}"))?;

        let tmp_path = sample.get_tmp_path().to_path_buf();
        let final_path = sample.get_path().to_path_buf();

        if !tmp_path.is_file() {
            return Err(anyhow!("Temp sample file doesn't exist on disk: {id}"));
        }

        std::fs::rename(&tmp_path, &final_path)
            .with_context(|| format!("Failed to move temp sample to cache: {id}"))?;

        sample.set_cached(true);
        Ok(())
    }

    pub fn invalidate_sample(&mut self, id: ID, generation: u64) {
        if self
            .samples
            .get(&id)
            .is_some_and(|sample| sample.generation() == generation)
        {
            self.samples.remove(&id);
        }
    }

    fn new_sample(&mut self, name: &str) -> Sample {
        self.generation = self.generation.checked_add(1).expect("Sample generation overflow");
        Sample::new(name, self.generation)
    }

    fn detect_tempo(filename: &str) -> Option<f64> {
        let re = regex::Regex::new(r"([0-9]{2,3}(?:[\.,][0-9]+)?)").unwrap();

        let midrange = 120.0;

        re.captures_iter(filename)
            .filter_map(|captures| captures[1].parse::<f64>().ok())
            .filter(|value| 30.0 <= *value && *value <= 300.0)
            .reduce(|a, b| {
                if (a - midrange).abs() < (b - midrange).abs() {
                    a
                } else {
                    b
                }
            })
    }

    pub fn get_sample_metadata(&self, id: ID) -> anyhow::Result<SampleMetadata> {
        let sample = self.samples.get(&id).ok_or_else(|| anyhow!("Sample not found: {id}"))?;

        let path = sample.get_path();
        if !sample.is_cached() || !path.is_file() {
            return Err(anyhow!("Sample doesn't exist on disk: {id}"));
        }

        let wav_reader = hound::WavReader::open(path).with_context(|| format!("Couldn't read audio file: {id}"))?;

        Ok(SampleMetadata {
            name: String::from(sample.get_name()),
            sample_rate: wav_reader.spec().sample_rate,
            sample_count: wav_reader.duration(),
            num_channels: u32::from(wav_reader.spec().channels),
            detected_tempo: Self::detect_tempo(sample.get_name()),
        })
    }

    pub async fn add_sample_from_file(
        &mut self,
        id: ID,
        format: AudioFileFormat,
        from_path: &Path,
    ) -> anyhow::Result<()> {
        self.samples.remove(&id);
        let mut sample = self.new_sample("");
        let path = self.path_for_sample(id, format);

        if path.is_file() {
            tokio::fs::remove_file(path.as_path())
                .await
                .with_context(|| format!("Failed to remove existing file: {}", path.display()))?
        }

        tokio::fs::copy(from_path, path.as_path())
            .await
            .with_context(|| format!("Error copying sample into cache: {}", from_path.display()))?;

        sample.set_cache_location(&path);
        sample.set_cached(true);

        self.samples.insert(id, sample);

        Ok(())
    }

    pub async fn scan(&mut self) -> anyhow::Result<()> {
        let mut dir_entries = tokio::fs::read_dir(&self.root_directory)
            .await
            .with_context(|| format!("Failed to read directory: {}", self.root_directory.display()))?;

        while let Some(entry) = dir_entries.next_entry().await? {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            if let Some(filename) = path.file_name().and_then(OsStr::to_str) {
                // Remove leftover temp files from interrupted downloads.
                if filename.ends_with(".tmp") {
                    warn!("Removing incomplete download temp file: {}", path.display());
                    if let Err(e) = tokio::fs::remove_file(&path).await {
                        warn!("Failed to remove temp file ({}): {}", path.display(), e);
                    }
                    continue;
                }

                if let Some((id_str, _ext)) = filename.split_once('.') {
                    if let Ok(id) = id_str.parse::<ID>() {
                        let validation_path = path.clone();
                        let valid = tokio::task::spawn_blocking(move || Self::is_valid_wav(&validation_path))
                            .await
                            .context("Failed to validate cached sample")?;
                        if !valid {
                            warn!("Removing invalid or truncated cached sample: {}", path.display());
                            if let Err(e) = tokio::fs::remove_file(&path).await {
                                warn!("Failed to remove invalid sample ({}): {}", path.display(), e);
                            }
                            continue;
                        }

                        debug!("Scan found sample: {id}");
                        let mut sample = self.new_sample(filename);
                        sample.set_cache_location(&path);
                        sample.set_cached(true);
                        self.samples.insert(id, sample);
                    }
                }
            }
        }
        Ok(())
    }

    fn is_valid_wav(path: &Path) -> bool {
        match hound::WavReader::open(path) {
            Ok(reader) => {
                let expected_bytes = reader.duration() as u64
                    * u64::from(reader.spec().channels)
                    * u64::from(reader.spec().bits_per_sample / 8);
                let file_size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
                // A valid WAV has at least a 44-byte header plus the PCM data.
                file_size >= 44 + expected_bytes
            }
            Err(_) => false,
        }
    }

    pub fn get_sample(&self, id: ID) -> Option<&Sample> {
        self.samples.get(&id)
    }

    fn path_for_sample(&self, id: ID, format: AudioFileFormat) -> PathBuf {
        let mut path = self.root_directory.clone();
        let filename = id.to_string() + "." + extension_for_format(format);
        path.push(filename);
        path
    }

    fn tmp_path_for_sample(&self, id: ID, format: AudioFileFormat) -> PathBuf {
        let mut path = self.root_directory.clone();
        let filename = id.to_string() + "." + extension_for_format(format) + ".tmp";
        path.push(filename);
        path
    }

    async fn write_to_file(&self, data: &[u8], path: &Path) -> anyhow::Result<()> {
        let mut position = 0;

        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .await
            .with_context(|| format!("Failed to open audio file: {}", path.display()))?;

        while position < data.len() {
            let bytes_written = file
                .write(&data[position..])
                .await
                .with_context(|| format!("Error writing audio file: {}", path.display()))?;

            position += bytes_written;
        }

        file.flush()
            .await
            .with_context(|| format!("Failed to flush audio file: {}", path.display()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn uploaded_bytes_are_readable_before_completion() {
        let dir = tempfile::tempdir().unwrap();
        let mut cache = SamplesCache::new(dir.path());
        cache.begin_upload(1, AudioFileFormat::WAV, "upload");
        let tmp_path = cache.get_sample(1).unwrap().get_tmp_path().to_path_buf();
        let mut expected = Vec::new();

        for _ in 0..100 {
            cache.upload(1, b"chunk").await.unwrap();
            expected.extend_from_slice(b"chunk");
            assert_eq!(std::fs::read(&tmp_path).unwrap(), expected);
        }
    }

    #[tokio::test]
    async fn stale_conversion_failure_preserves_replacement_upload() {
        let dir = tempfile::tempdir().unwrap();
        let mut cache = SamplesCache::new(dir.path());
        cache.begin_upload(1, AudioFileFormat::WAV, "original");
        cache.upload(1, b"original").await.unwrap();
        cache.complete_upload(1).unwrap();
        let old_generation = cache.get_sample(1).unwrap().generation();

        cache.begin_upload(1, AudioFileFormat::WAV, "replacement");
        cache.upload(1, b"replacement").await.unwrap();
        let sample = cache.get_sample(1).unwrap();
        let generation = sample.generation();
        let tmp_path = sample.get_tmp_path().to_path_buf();
        assert_ne!(generation, old_generation);

        cache.invalidate_sample(1, old_generation);
        assert_eq!(tokio::fs::read(&tmp_path).await.unwrap(), b"replacement");
        cache.complete_upload(1).unwrap();
        let path = cache.get_sample(1).unwrap().get_path().to_path_buf();

        cache.invalidate_sample(1, old_generation);
        assert_eq!(cache.get_sample(1).unwrap().generation(), generation);
        assert_eq!(tokio::fs::read(&path).await.unwrap(), b"replacement");

        cache.invalidate_sample(1, generation);
        assert!(cache.get_sample(1).is_none());
        assert!(!path.exists());
    }

    #[tokio::test]
    async fn invalidated_sample_can_be_added_again() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("source");
        tokio::fs::write(&source, b"sample").await.unwrap();
        let mut cache = SamplesCache::new(dir.path());

        cache
            .add_sample_from_file(1, AudioFileFormat::WAV, &source)
            .await
            .unwrap();
        let path = cache.get_sample(1).unwrap().get_path().to_path_buf();
        cache.invalidate_sample(1, cache.get_sample(1).unwrap().generation());
        assert!(cache.get_sample(1).is_none());
        assert!(!path.exists());

        cache
            .add_sample_from_file(1, AudioFileFormat::WAV, &source)
            .await
            .unwrap();
        assert!(cache.get_sample(1).unwrap().is_cached());
        cache
            .add_sample_from_file(1, AudioFileFormat::WAV, &source)
            .await
            .unwrap();
        assert_eq!(tokio::fs::read(path).await.unwrap(), b"sample");
    }

    #[tokio::test]
    async fn interrupted_uploads_are_cleaned_up_and_can_be_retried() {
        let dir = tempfile::tempdir().unwrap();
        let mut cache = SamplesCache::new(dir.path());
        cache.begin_upload(1, AudioFileFormat::WAV, "first");
        cache.upload(1, b"partial").await.unwrap();
        let tmp_path = cache.get_sample(1).unwrap().get_tmp_path().to_path_buf();
        assert!(tmp_path.exists());

        cache.begin_upload(1, AudioFileFormat::WAV, "retry");
        assert!(!tmp_path.exists());
        cache.upload(1, b"complete").await.unwrap();
        cache.complete_upload(1).unwrap();
        let path = cache.get_sample(1).unwrap().get_path().to_path_buf();
        assert_eq!(tokio::fs::read(&path).await.unwrap(), b"complete");
        assert!(!tmp_path.exists());

        cache.begin_upload(2, AudioFileFormat::WAV, "interrupted");
        cache.upload(2, b"partial").await.unwrap();
        let interrupted_path = cache.get_sample(2).unwrap().get_tmp_path().to_path_buf();
        cache.invalidate_sample(2, cache.get_sample(2).unwrap().generation());
        assert!(!interrupted_path.exists());

        cache.begin_upload(2, AudioFileFormat::WAV, "interrupted again");
        cache.upload(2, b"partial").await.unwrap();
        drop(cache);
        assert!(!interrupted_path.exists());
    }

    #[tokio::test]
    async fn scan_keeps_valid_wavs_and_removes_truncated_and_temp_files() {
        let dir = tempfile::tempdir().unwrap();
        let valid_path = dir.path().join("1.wav");
        let truncated_path = dir.path().join("2.wav");
        let tmp_path = dir.path().join("3.wav.tmp");
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(&valid_path, spec).unwrap();
        for _ in 0..100 {
            writer.write_sample(0i16).unwrap();
        }
        writer.finalize().unwrap();
        let bytes = tokio::fs::read(&valid_path).await.unwrap();
        tokio::fs::write(&truncated_path, &bytes[..bytes.len() - 10])
            .await
            .unwrap();
        tokio::fs::write(&tmp_path, b"partial").await.unwrap();

        let mut cache = SamplesCache::new(dir.path());
        cache.scan().await.unwrap();
        assert!(cache.get_sample(1).unwrap().is_cached());
        assert!(valid_path.exists());
        assert!(cache.get_sample(2).is_none());
        assert!(!truncated_path.exists());
        assert!(!tmp_path.exists());
    }
}
