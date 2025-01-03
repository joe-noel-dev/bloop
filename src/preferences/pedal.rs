use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PedalPreferences {
    #[serde(default)]
    pub serial_path: Option<String>,
}
