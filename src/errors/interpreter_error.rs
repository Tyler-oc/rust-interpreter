use std::error::Error;
use std::fmt;

use crate::errors::lex_error::LexError;
use crate::errors::parse_error::ParseError;
use crate::errors::runtime_error::RunTimeError;

#[derive(Debug)]
pub enum InterpreterError {
    LexError(LexError),
    ParseError(ParseError),
    RunTimeError(RunTimeError),
}

impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterpreterError::LexError(e) => write!(f, "{}", e),
            InterpreterError::ParseError(e) => write!(f, "{}", e),
            InterpreterError::RunTimeError(e) => write!(f, "{}", e),
        }
    }
}

impl Error for InterpreterError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            InterpreterError::LexError(e) => Some(e),
            InterpreterError::ParseError(e) => Some(e),
            InterpreterError::RunTimeError(e) => Some(e),
        }
    }
}

impl From<LexError> for InterpreterError {
    fn from(value: LexError) -> Self {
        InterpreterError::LexError(value)
    }
}

impl From<ParseError> for InterpreterError {
    fn from(value: ParseError) -> Self {
        InterpreterError::ParseError(value)
    }
}

impl From<RunTimeError> for InterpreterError {
    fn from(value: RunTimeError) -> Self {
        InterpreterError::RunTimeError(value)
    }
}
