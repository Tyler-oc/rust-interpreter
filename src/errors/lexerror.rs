use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum LexError {
    NotFound(String),
    ValueError(String),
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexError::NotFound(token) => write!(f, "invalid token: {}", token),
            LexError::ValueError(token_type) => write!(f, "no value found for {}", token_type),
        }
    }
}

impl Error for LexError {}
