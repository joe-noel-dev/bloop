use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use std::fs;
use std::io::prelude::*;

use crate::{
    model::id::ID,
    types::audio_file_format::{extension_for_format, AudioFileFormat},
};

use super::sample::Sample;

pub struct SamplesCache {
    root_directory: PathBuf,
    samples: HashMap<ID, Sample>,
}

pub struct SampleMetadata {
    pub sample_rate: u32,
    pub sample_count: u32,
    pub num_channels: u32,
}

impl SamplesCache {
    pub fn new(root_directory: &Path) -> Self {
        if !root_directory.exists() {
            fs::create_dir_all(root_directory).expect(&format!(
                "Couldn't create directory: {}",
                root_directory.to_str().unwrap()
            ));
        }

        Self {
            root_directory: PathBuf::from(root_directory),
            samples: HashMap::new(),
        }
    }

    pub fn add_sample_from_data(
        &mut self,
        id: &ID,
        format: &AudioFileFormat,
        data: &[u8],
    ) -> Result<SampleMetadata, String> {
        let mut sample = Sample::new(id);
        let path = self.path_for_sample(id, &format);

        self.write_file(data, &path)?;

        sample.set_cache_location(&path);

        self.samples.insert(*id, sample);

        let wav_reader = match hound::WavReader::open(path) {
            Ok(reader) => reader,
            Err(error) => return Err(format!("Failed to read audio file: {}", error)),
        };

        Ok(SampleMetadata {
            sample_rate: wav_reader.spec().sample_rate,
            sample_count: wav_reader.duration(),
            num_channels: u32::from(wav_reader.spec().channels),
        })
    }

    pub fn add_sample_from_file(&mut self, id: &ID, format: &AudioFileFormat, from_path: &Path) -> Result<(), String> {
        let mut sample = Sample::new(id);
        let path = self.path_for_sample(id, &format);

        if path.is_file() {
            match fs::remove_file(path.as_path()) {
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

        match fs::copy(from_path, path.as_path()) {
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

    fn write_file(&self, data: &[u8], path: &Path) -> Result<(), String> {
        let mut position = 0;
        let mut file = match fs::File::create(path) {
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
            let bytes_written = match file.write(&data[position..]) {
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
