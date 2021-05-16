use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AudioFileFormat {
    Wav,
}

pub fn extension_for_format(format: &AudioFileFormat) -> &'static str {
    match format {
        AudioFileFormat::Wav => "wav",
    }
}
