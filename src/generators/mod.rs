use crate::model::{channel, id, project, sample, section, song, state};
use rand::seq::SliceRandom;
use rand::Rng;
use uuid::Uuid;

const ADJECTIVES: &'static [&'static str] =
    &["Huge", "Weird", "Zany", "Amazing", "Mellow", "Soft"];
const NOUNS: &'static [&'static str] =
    &["vibes", "beat", "tune", "symphony", "chops"];

const CHANNEL_COLOURS: &'static [&'static str] = &[
    "#57C666", "#708FDA", "#70DADA", "#A0DA70", "#C570DA", "#DA7070",
    "#F49F0A", "#00A6A6", "#BBDEF0", "#FC60A8",
];

const CHANNEL_NANES: &'static [&'static str] = &[
    "Bass", "Guitar", "Vox", "Drums", "Click", "Keys", "Synth", "Pad",
    "Shaker", "Perc",
];

const SECTION_NAMES: &'static [&'static str] = &[
    "Verse", "Chorus", "Intro", "Outro", "Break", "Middle", "Hook",
];

fn random_name() -> String {
    let mut rng = rand::thread_rng();
    return format!(
        "{} {}",
        ADJECTIVES.choose(&mut rng).unwrap(),
        NOUNS.choose(&mut rng).unwrap()
    );
}

fn random_channel_name() -> String {
    return format!(
        "{}",
        CHANNEL_NANES.choose(&mut rand::thread_rng()).unwrap()
    );
}

fn random_section_name() -> String {
    return format!(
        "{}",
        SECTION_NAMES.choose(&mut rand::thread_rng()).unwrap()
    );
}

fn random_colour() -> String {
    let mut rng = rand::thread_rng();
    return format!("{}", CHANNEL_COLOURS.choose(&mut rng).unwrap());
}

fn generate_channels(num_channels: u32) -> Vec<channel::Channel> {
    return (0..num_channels).map(|_| generate_channel()).collect();
}

fn get_channel_ids(channels: &[channel::Channel]) -> Vec<id::ID> {
    return channels.iter().map(|channel| channel.id).collect();
}

fn generate_songs(num_songs: u32) -> Vec<song::Song> {
    return (0..num_songs).map(|_| generate_song()).collect();
}

fn generate_sections(
    num_sections_per_song: u32,
    songs: &mut [song::Song],
    channel_ids: &[id::ID],
) -> Vec<section::Section> {
    let mut sections: Vec<section::Section> = vec![];
    for song in songs.iter_mut() {
        let mut song_sections: Vec<section::Section> = (0
            ..num_sections_per_song)
            .map(|_| generate_section(&channel_ids))
            .collect();
        song.section_ids =
            song_sections.iter().map(|section| section.id).collect();
        sections.append(&mut song_sections);
    }

    return sections;
}

fn get_sample_ids(sections: &[section::Section]) -> Vec<id::ID> {
    let mut sample_ids: Vec<id::ID> = vec![];
    for section in sections.iter() {
        sample_ids = [
            sample_ids,
            section.samples.iter().map(|pair| pair.sample_id).collect(),
        ]
        .concat();
    }
    return sample_ids;
}

fn generate_samples(sample_ids: &[id::ID]) -> Vec<sample::Sample> {
    return sample_ids
        .iter()
        .map(|sample_id| generate_sample(sample_id))
        .collect();
}

pub fn generate_project(
    num_channels: u32,
    num_songs: u32,
    num_sections_per_song: u32,
) -> project::Project {
    let channels = generate_channels(num_channels);
    let channel_ids = get_channel_ids(&channels);
    let mut songs = generate_songs(num_songs);
    let sections =
        generate_sections(num_sections_per_song, &mut songs, &channel_ids);
    let sample_ids = get_sample_ids(&sections);
    let samples = generate_samples(&sample_ids);

    project::Project {
        id: Uuid::new_v4(),
        state: state::State::Active,
        info: project::ProjectInfo {
            name: random_name(),
            version: "0".to_string(),
        },
        songs,
        sections,
        channels,
        samples,
    }
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

pub fn generate_song() -> song::Song {
    let mut rng = rand::thread_rng();
    song::Song {
        id: Uuid::new_v4(),
        state: state::State::Active,
        name: random_name(),
        tempo: song::Tempo {
            bpm: rng.gen_range(30.0, 300.0),
        },
        metronome: song::Metronome::Default,
        section_ids: vec![],
    }
}

pub fn generate_section(channel_ids: &[id::ID]) -> section::Section {
    let mut rng = rand::thread_rng();
    section::Section {
        id: Uuid::new_v4(),
        state: state::State::Active,
        name: random_section_name(),
        beat_length: rng.gen_range(4.0, 24.0),
        loop_properties: section::LoopProperties {
            mode: section::LoopMode::Fixed,
            count: 1,
        },
        samples: channel_ids
            .into_iter()
            .map(|channel_id| section::ChannelSamplePair {
                channel_id: channel_id.clone(),
                sample_id: Uuid::new_v4(),
            })
            .collect(),
    }
}

pub fn generate_sample(id: &id::ID) -> sample::Sample {
    let mut rng = rand::thread_rng();
    sample::Sample {
        id: id.clone(),
        state: state::State::Active,
        path: "/path/to/sample.wav".to_string(),
        tempo: rng.gen_range(30.0, 300.0),
    }
}
