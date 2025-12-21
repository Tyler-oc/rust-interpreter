use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum LexError {
    NotFound(String),
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexError::NotFound(token) => write!(f, "invalid token: {}", token),
        }
    }
}

impl Error for LexError {}
