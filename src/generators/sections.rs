use crate::model::{id, section, song};
use rand::seq::SliceRandom;
use rand::Rng;
use uuid::Uuid;

pub fn generate_sections(
    num_sections_per_song: u32,
    songs: &mut [song::Song],
    channel_ids: &[id::ID],
) -> Vec<section::Section> {
    let mut sections: Vec<section::Section> = vec![];
    for song in songs.iter_mut() {
        let mut song_sections: Vec<section::Section> = (0..num_sections_per_song)
            .map(|_| generate_section(&channel_ids))
            .collect();
        song.section_ids = song_sections.iter().map(|section| section.id).collect();
        sections.append(&mut song_sections);
    }

    return sections;
}

pub fn generate_section(channel_ids: &[id::ID]) -> section::Section {
    let mut rng = rand::thread_rng();
    section::Section {
        id: Uuid::new_v4(),
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

pub fn get_sample_ids(sections: &[section::Section]) -> Vec<id::ID> {
    let mut sample_ids: Vec<id::ID> = vec![];
    for section in sections.iter() {
        sample_ids = [sample_ids, section.samples.iter().map(|pair| pair.sample_id).collect()].concat();
    }
    return sample_ids;
}

fn random_section_name() -> String {
    return format!("{}", SECTION_NAMES.choose(&mut rand::thread_rng()).unwrap());
}
const SECTION_NAMES: &'static [&'static str] = &["Verse", "Chorus", "Intro", "Outro", "Break", "Middle", "Hook"];
