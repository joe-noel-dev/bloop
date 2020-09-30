use super::names;
use crate::model::{song, state};
use rand::Rng;
use uuid::Uuid;

pub fn generate_song() -> song::Song {
    let mut rng = rand::thread_rng();
    song::Song {
        id: Uuid::new_v4(),
        state: state::State::Active,
        name: names::random_name(),
        tempo: song::Tempo {
            bpm: rng.gen_range(30.0, 300.0),
        },
        metronome: song::Metronome::Default,
        section_ids: vec![],
    }
}

pub fn generate_songs(num_songs: u32) -> Vec<song::Song> {
    return (0..num_songs).map(|_| generate_song()).collect();
}
