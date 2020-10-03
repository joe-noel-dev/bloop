use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct NetworkError {
    details: String,
}

impl NetworkError {
    pub fn new(msg: &str) -> Self {
        NetworkError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for NetworkError {
    fn description(&self) -> &str {
        &self.details
    }
}
