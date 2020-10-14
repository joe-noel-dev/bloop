use crate::model::{id, sample};
use rand::Rng;

pub fn generate_samples(sample_ids: &[id::ID]) -> Vec<sample::Sample> {
    return sample_ids.iter().map(|sample_id| generate_sample(sample_id)).collect();
}

pub fn generate_sample(id: &id::ID) -> sample::Sample {
    let mut rng = rand::thread_rng();
    sample::Sample {
        id: id.clone(),
        path: "/path/to/sample.wav".to_string(),
        tempo: rng.gen_range(30.0, 300.0),
    }
}
