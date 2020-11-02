use rand::seq::SliceRandom;

pub fn random_name() -> String {
    let mut rng = rand::thread_rng();
    return format!(
        "{} {}",
        ADJECTIVES.choose(&mut rng).unwrap(),
        NOUNS.choose(&mut rng).unwrap()
    );
}

pub fn random_section_name() -> String {
    return format!("{}", SECTION_NAMES.choose(&mut rand::thread_rng()).unwrap());
}

const CHANNEL_NANES: &'static [&'static str] = &[
    "Bass", "Guitar", "Vox", "Drums", "Click", "Keys", "Synth", "Pad", "Shaker", "Perc",
];

pub fn random_channel_name() -> String {
    return format!("{}", CHANNEL_NANES.choose(&mut rand::thread_rng()).unwrap());
}

const SECTION_NAMES: &'static [&'static str] = &["Verse", "Chorus", "Intro", "Outro", "Break", "Middle", "Hook"];

const ADJECTIVES: &'static [&'static str] = &["Huge", "Weird", "Tight", "Amazing", "Mellow", "Soft", "Funky", "Heavy"];
const NOUNS: &'static [&'static str] = &["vibes", "beat", "tune", "symphony", "chops", "banger"];
