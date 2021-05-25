use super::sample::Sample;
use crate::{
    model::id::ID,
    samples::sample::SampleMetadata,
    types::audio_file_format::{extension_for_format, AudioFileFormat},
};
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
        let path = self.path_for_sample(id, &format);
        sample.set_cache_location(&path);
        self.samples.insert(*id, sample);
    }

    pub async fn upload(&mut self, id: &ID, data: &[u8]) -> Result<(), String> {
        let sample = self.samples.get(id).ok_or(format!("Sample not found: {}", id))?;
        let path = sample.get_path();
        self.write_to_file(data, path).await?;
        Ok(())
    }

    pub fn complete_upload(&mut self, id: &ID) -> Result<(), String> {
        let sample = self.samples.get_mut(id).ok_or(format!("Sample not found: {}", id))?;

        let path = sample.get_path();
        if !path.is_file() {
            return Err(format!("Sample doesn't exist on disk: {}", id));
        }

        let path = sample.get_path();
        let wav_reader =
            hound::WavReader::open(path).map_err(|error| format!("Couldn't read audio file: {}", error))?;

        sample.set_metadata(SampleMetadata {
            sample_rate: wav_reader.spec().sample_rate,
            sample_count: wav_reader.duration(),
            num_channels: u32::from(wav_reader.spec().channels),
        });

        Ok(())
    }

    pub async fn add_sample_from_file(
        &mut self,
        id: &ID,
        format: &AudioFileFormat,
        from_path: &Path,
    ) -> Result<(), String> {
        let mut sample = Sample::new("");
        let path = self.path_for_sample(id, &format);

        if path.is_file() {
            match tokio::fs::remove_file(path.as_path()).await {
                Ok(_) => (),
                Err(error) => {
                    return Err(format!(
                        "Failed to remove existing file ({}): {}",
                        path.display(),
                        error
                    ))
                }
            }
        }

        match tokio::fs::copy(from_path, path.as_path()).await {
            Ok(_) => (),
            Err(error) => {
                return Err(format!(
                    "Error copying sample into cache ({}): {}",
                    from_path.display(),
                    error
                ))
            }
        };

        sample.set_cache_location(&path);

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
        let filename = id.to_string() + "." + extension_for_format(&format);
        path.push(filename);
        path
    }

    async fn write_to_file(&self, data: &[u8], path: &Path) -> Result<(), String> {
        let mut position = 0;

        let mut file = match OpenOptions::new().append(true).create(true).open(path).await {
            Ok(file) => file,
            Err(error) => {
                return Err(format!(
                    "Failed to open audio file ({}): {}",
                    path.display(),
                    error.to_string()
                ))
            }
        };

        while position < data.len() {
            let bytes_written = match file.write(&data[position..]).await {
                Ok(bytes_written) => bytes_written,
                Err(error) => return Err(format!("Error writing audio file: {}", error)),
            };

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
