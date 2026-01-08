use std::error::Error;
use std::fmt;
#[derive(Debug)]
pub enum LexError {
    NotFound(String),
    ValueError(String),
    UnexpectedCharacter { char: char, line: usize },
    UnterminatedString { line: usize },
    NumberParsingError { num_str: String, line: usize },
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexError::NotFound(token) => write!(f, "invalid token: {}", token),
            LexError::ValueError(token_type) => write!(f, "no value found for {}", token_type),
            LexError::UnexpectedCharacter { char, line } => {
                write!(f, "unexpected character {} on line {}", char, line)
            }
            LexError::UnterminatedString { line } => {
                write!(f, "unterminated string at line: {}", line)
            }
            LexError::NumberParsingError { num_str, line } => {
                write!(f, "failed to parse double: {} on line {}", num_str, line)
            }
        }
    }
}

impl Error for LexError {}
