use std::error::Error;
use std::fmt;

use crate::errors::lexError::LexError;
use crate::errors::parseError::ParseError;

#[derive(Debug)]
pub enum InterpreterError {
    LexError(LexError),
    ParseError(ParseError),
}

impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterpreterError::LexError(e) => write!(f, "{}", e),
            InterpreterError::ParseError(e) => write!(f, "{}", e),
        }
    }
}

impl Error for InterpreterError {}
