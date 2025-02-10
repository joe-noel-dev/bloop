use rand::seq::IndexedRandom;

pub fn random_name() -> String {
    let mut rng = rand::rng();
    format!(
        "{} {}",
        ADJECTIVES.choose(&mut rng).unwrap(),
        NOUNS.choose(&mut rng).unwrap()
    )
}

pub fn random_section_name() -> &'static str {
    SECTION_NAMES.choose(&mut rand::rng()).unwrap()
}

const SECTION_NAMES: &[&str] = &["Verse", "Chorus", "Intro", "Outro", "Break", "Middle", "Hook"];

const ADJECTIVES: &[&str] = &["Huge", "Weird", "Tight", "Amazing", "Mellow", "Soft", "Funky", "Heavy"];
const NOUNS: &[&str] = &["vibes", "beat", "tune", "symphony", "chops", "banger"];
