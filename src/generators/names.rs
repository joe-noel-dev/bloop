use rand::seq::SliceRandom;

pub fn random_name() -> String {
    let mut rng = rand::thread_rng();
    return format!(
        "{} {}",
        ADJECTIVES.choose(&mut rng).unwrap(),
        NOUNS.choose(&mut rng).unwrap()
    );
}

const ADJECTIVES: &'static [&'static str] = &["Huge", "Weird", "Zany", "Amazing", "Mellow", "Soft"];
const NOUNS: &'static [&'static str] = &["vibes", "beat", "tune", "symphony", "chops"];
