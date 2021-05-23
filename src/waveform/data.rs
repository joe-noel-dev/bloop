use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Algorithm {
    Min,
    Max,
    Rms,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Properties {
    pub length: i32,
    pub algorithm: Algorithm,
    pub channel: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WaveformData {
    sample_rate: i32,
    data: HashMap<Properties, Vec<f32>>,
}

impl WaveformData {
    pub fn new(sample_rate: i32) -> Self {
        Self {
            sample_rate,
            data: HashMap::new(),
        }
    }

    pub fn set_data(&mut self, properties: Properties, data: Vec<f32>) {
        self.data.insert(properties, data);
    }

    pub fn push(&mut self, properties: &Properties, value: f32) {
        match self.data.get_mut(properties) {
            None => {
                self.data.insert(properties.clone(), vec![value]);
            }
            Some(values) => values.push(value),
        }
    }

    pub fn sample_rate(&self) -> i32 {
        self.sample_rate
    }

    pub fn reset(&mut self) {
        self.sample_rate = 0;
        self.data.clear();
    }

    pub fn add(&mut self, other: Self) {
        if other == *self {
            return;
        }

        if other.sample_rate != self.sample_rate {
            return;
        }

        self.data.extend(other.data);
    }
}
