use std::error::Error;
use std::fmt;
#[derive(Debug)]
pub enum RunTimeError {
    CouldNotEval(String),
}

impl fmt::Display for RunTimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RunTimeError::CouldNotEval(val) => write!(f, "Could not evaluate: {}", val),
        }
    }
}

impl Error for RunTimeError {}
