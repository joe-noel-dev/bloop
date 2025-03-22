use crate::bloop::AudioFileFormat;

pub fn extension_for_format(format: AudioFileFormat) -> &'static str {
    match format {
        AudioFileFormat::WAV => "wav",
    }
}
