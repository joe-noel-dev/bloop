use crate::model::{channel, id, state};
use rand::seq::SliceRandom;
use uuid::Uuid;

pub fn generate_channels(num_channels: u32) -> Vec<channel::Channel> {
    return (0..num_channels).map(|_| generate_channel()).collect();
}

pub fn generate_channel() -> channel::Channel {
    channel::Channel {
        id: Uuid::new_v4(),
        state: state::State::Active,
        name: random_channel_name(),
        mute: false,
        solo: false,
        colour: random_colour(),
    }
}

const CHANNEL_NANES: &'static [&'static str] = &[
    "Bass", "Guitar", "Vox", "Drums", "Click", "Keys", "Synth", "Pad", "Shaker", "Perc",
];

const CHANNEL_COLOURS: &'static [&'static str] = &[
    "#57C666", "#708FDA", "#70DADA", "#A0DA70", "#C570DA", "#DA7070", "#F49F0A", "#00A6A6", "#BBDEF0", "#FC60A8",
];

fn random_channel_name() -> String {
    return format!("{}", CHANNEL_NANES.choose(&mut rand::thread_rng()).unwrap());
}

fn random_colour() -> String {
    let mut rng = rand::thread_rng();
    return format!("{}", CHANNEL_COLOURS.choose(&mut rng).unwrap());
}

pub fn get_channel_ids(channels: &[channel::Channel]) -> Vec<id::ID> {
    return channels.iter().map(|channel| channel.id).collect();
}
