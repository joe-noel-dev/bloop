use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    PreviousSong,
    NextSong,
    PreviousSection,
    NextSection,
    QueueSelected,
    ToggleLoop,
    TogglePlay,
}
