use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "camelCase")]
pub enum Algorithm {
    Min,
    Max,
    Rms,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    pub length: i32,
    pub algorithm: Algorithm,
    pub channel: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WaveformGroup {
    pub properties: Properties,
    pub values: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WaveformData {
    sample_rate: i32,
    peaks: Vec<WaveformGroup>,
}

impl WaveformData {
    pub fn new(sample_rate: i32) -> Self {
        Self {
            sample_rate,
            peaks: vec![],
        }
    }

    fn get_group_mut(&mut self, properties: &Properties) -> Option<&mut WaveformGroup> {
        self.peaks.iter_mut().find(|group| (*group).properties == *properties)
    }

    pub fn sort(&mut self) {
        self.peaks.sort_by(|a, b| a.properties.cmp(&b.properties))
    }

    pub fn push(&mut self, properties: &Properties, value: f32) {
        match self.get_group_mut(properties) {
            None => {
                self.peaks.push(WaveformGroup {
                    properties: properties.clone(),
                    values: vec![value],
                });
            }
            Some(group) => group.values.push(value),
        }
    }

    pub fn add(&mut self, mut other: Self) {
        if other == *self {
            return;
        }

        if other.sample_rate != self.sample_rate {
            return;
        }

        self.peaks.append(&mut other.peaks);
    }
}
