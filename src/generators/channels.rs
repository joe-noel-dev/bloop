use crate::model::{channel, id};

pub fn generate_channels(num_channels: u32) -> Vec<channel::Channel> {
    return (0..num_channels).map(|_| generate_channel()).collect();
}

pub fn generate_channel() -> channel::Channel {
    channel::Channel::new().with_random_name()
}

pub fn get_channel_ids(channels: &[channel::Channel]) -> Vec<id::ID> {
    return channels.iter().map(|channel| channel.id).collect();
}
