use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Progress {
    pub song_progress: f64,
    pub section_progress: f64,
    pub section_beat: f64,
}
