use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Copy)]
#[serde(rename_all = "camelCase")]
pub struct Progress {
    pub song_progress: f64,
    pub section_progress: f64,
}
