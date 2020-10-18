use rand::seq::SliceRandom;

pub fn random_name() -> String {
    let mut rng = rand::thread_rng();
    return format!(
        "{} {}",
        ADJECTIVES.choose(&mut rng).unwrap(),
        NOUNS.choose(&mut rng).unwrap()
    );
}

const ADJECTIVES: &'static [&'static str] = &["Huge", "Weird", "Tight", "Amazing", "Mellow", "Soft", "Funky", "Heavy"];
const NOUNS: &'static [&'static str] = &["vibes", "beat", "tune", "symphony", "chops", "banger"];
