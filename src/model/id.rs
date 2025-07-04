use rand::Rng;

pub type ID = u64;

pub fn random_id() -> ID {
    rand::rng().random()
}

pub fn random_project_id() -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    const ID_LENGTH: usize = 15;

    let mut rng = rand::rng();
    (0..ID_LENGTH)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub const INVALID_ID: ID = 0;
