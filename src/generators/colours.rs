use rand::seq::SliceRandom;

const DEFAULT_COLOURS: &'static [&'static str] = &[
    "#57C666", "#708FDA", "#70DADA", "#A0DA70", "#C570DA", "#DA7070", "#F49F0A", "#00A6A6", "#BBDEF0", "#FC60A8",
];

pub fn random_colour() -> String {
    let mut rng = rand::thread_rng();
    return format!("{}", DEFAULT_COLOURS.choose(&mut rng).unwrap());
}
