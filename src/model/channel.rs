use super::id::ID;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

const DEFAULT_COLOURS: &'static [&'static str] = &[
    "#57C666", "#708FDA", "#70DADA", "#A0DA70", "#C570DA", "#DA7070", "#F49F0A", "#00A6A6", "#BBDEF0", "#FC60A8",
];

fn random_colour() -> String {
    let mut rng = rand::thread_rng();
    return format!("{}", DEFAULT_COLOURS.choose(&mut rng).unwrap());
}

const CHANNEL_NANES: &'static [&'static str] = &[
    "Bass", "Guitar", "Vox", "Drums", "Click", "Keys", "Synth", "Pad", "Shaker", "Perc",
];

fn random_channel_name() -> String {
    return format!("{}", CHANNEL_NANES.choose(&mut rand::thread_rng()).unwrap());
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub id: ID,
    pub name: String,
    pub mute: bool,
    pub solo: bool,
    pub colour: String,
}

impl Channel {
    pub fn new() -> Self {
        Self {
            id: ID::new_v4(),
            name: "Channel".to_string(),
            mute: false,
            solo: false,
            colour: random_colour(),
        }
    }
    pub fn with_random_name(mut self) -> Self {
        self.name = random_channel_name();
        self
    }

    pub fn _with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
}
