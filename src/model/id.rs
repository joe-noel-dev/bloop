pub type ID = u64;

pub fn random_id() -> ID {
    use rand::Rng;
    rand::rng().random()
}

pub const INVALID_ID: ID = 0;
