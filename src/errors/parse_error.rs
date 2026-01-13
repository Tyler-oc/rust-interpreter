use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    InvalidConversion(String),
    IndexOutOfBounds,
    InvalidGrouping(String),
    MissingValue { val: String, line: usize },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidConversion(invalid_obj) => {
                write!(f, "ParseError: Conversion Error: {}", invalid_obj)
            }
            ParseError::IndexOutOfBounds => write!(f, "Index out of bounds"),
            ParseError::InvalidGrouping(message) => write!(f, "{}", message),
            ParseError::MissingValue { val, line } => {
                write!(f, "Missing value: {} at line {}", val, line)
            }
        }
    }
}

impl Error for ParseError {}
