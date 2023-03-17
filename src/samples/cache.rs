use super::sample::Sample;
use crate::{
    model::ID,
    types::{extension_for_format, AudioFileFormat},
};
use anyhow::{anyhow, Context};
use std::fs;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

pub struct SamplesCache {
    root_directory: PathBuf,
    samples: HashMap<ID, Sample>,
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
            fs::create_dir_all(root_directory)
                .unwrap_or_else(|_| panic!("Couldn't create directory: {}", root_directory.to_str().unwrap()));
        }

        Self {
            root_directory: PathBuf::from(root_directory),
            samples: HashMap::new(),
        }
    }

    pub fn begin_upload(&mut self, id: &ID, format: &AudioFileFormat, filename: &str) {
        let mut sample = Sample::new(filename);
        let path = self.path_for_sample(id, format);
        sample.set_cache_location(&path);
        self.samples.insert(*id, sample);
    }

    pub async fn upload(&mut self, id: &ID, data: &[u8]) -> anyhow::Result<()> {
        let sample = self
            .samples
            .get(id)
            .ok_or_else(|| anyhow!("Sample not found: {}", id))?;
        let path = sample.get_path();
        self.write_to_file(data, path).await?;
        Ok(())
    }

    pub fn complete_upload(&mut self, id: &ID) -> anyhow::Result<()> {
        let sample = self
            .samples
            .get_mut(id)
            .ok_or_else(|| anyhow!("Sample not found: {}", id))?;

        let path = sample.get_path();
        if !path.is_file() {
            return Err(anyhow!("Sample doesn't exist on disk: {}", id));
        }

        sample.set_cached(true);
        Ok(())
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

    pub fn get_sample_metadata(&self, id: &ID) -> anyhow::Result<SampleMetadata> {
        let sample = self
            .samples
            .get(id)
            .ok_or_else(|| anyhow!("Sample not found: {}", id))?;

        let path = sample.get_path();
        if !sample.is_cached() || !path.is_file() {
            return Err(anyhow!("Sample doesn't exist on disk: {}", id));
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
        id: &ID,
        format: &AudioFileFormat,
        from_path: &Path,
    ) -> anyhow::Result<()> {
        let mut sample = Sample::new("");
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

        self.samples.insert(*id, sample);

        Ok(())
    }

    pub fn clear(&mut self) {
        self.samples.clear();
    }

    pub fn get_sample(&self, id: &ID) -> Option<&Sample> {
        self.samples.get(id)
    }

    fn path_for_sample(&self, id: &ID, format: &AudioFileFormat) -> PathBuf {
        let mut path = self.root_directory.clone();
        let filename = id.to_string() + "." + extension_for_format(format);
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

        Ok(())
    }
}

impl Drop for SamplesCache {
    fn drop(&mut self) {
        if let Err(error) = fs::remove_dir_all(&self.root_directory) {
            println!(
                "Failed to remove directory ({}): {}",
                self.root_directory.display(),
                error
            );
        }
    }
}
