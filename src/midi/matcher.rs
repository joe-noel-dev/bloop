pub trait Matcher {
    fn matches(&self, message: &[u8]) -> bool;
}

pub struct ExactMatcher {
    reference: Vec<u8>,
}

impl Matcher for ExactMatcher {
    fn matches(&self, message: &[u8]) -> bool {
        self.reference.len() == message.len() && self.reference.iter().zip(message.iter()).all(|(a, b)| a == b)
    }
}

impl ExactMatcher {
    pub fn new(reference: &[u8]) -> Self {
        Self {
            reference: Vec::from(reference),
        }
    }
}
