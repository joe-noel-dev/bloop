use rand::seq::SliceRandom;

pub fn random_name() -> String {
    let mut rng = rand::thread_rng();
    format!(
        "{} {}",
        ADJECTIVES.choose(&mut rng).unwrap(),
        NOUNS.choose(&mut rng).unwrap()
    )
}

pub fn random_section_name() -> &'static str {
    return SECTION_NAMES.choose(&mut rand::thread_rng()).unwrap();
}

const CHANNEL_NANES: &[&str] = &[
    "Bass", "Guitar", "Vox", "Drums", "Click", "Keys", "Synth", "Pad", "Shaker", "Perc",
];

pub fn random_channel_name() -> &'static str {
    return CHANNEL_NANES.choose(&mut rand::thread_rng()).unwrap();
}

const SECTION_NAMES: &[&str] = &["Verse", "Chorus", "Intro", "Outro", "Break", "Middle", "Hook"];

const ADJECTIVES: &[&str] = &["Huge", "Weird", "Tight", "Amazing", "Mellow", "Soft", "Funky", "Heavy"];
const NOUNS: &[&str] = &["vibes", "beat", "tune", "symphony", "chops", "banger"];
