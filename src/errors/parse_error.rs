use std::error::Error;
use std::fmt;

use crate::lexing::token::Token;
use crate::parsing::ast::Expr;

#[derive(Debug)]
pub enum ParseError {
    InvalidConversion(String),
    IndexOutOfBounds,
    InvalidGrouping(String),
    MissingValue { val: String, line: usize },
    InvalidDeclaration(String),
    AssignmentError(Token),
    SyntaxError(String),
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
            ParseError::InvalidDeclaration(e) => {
                write!(f, "Invalid declaration: {}", e)
            }
            ParseError::AssignmentError(t) => {
                write!(f, "Assignment error {} on line {}", t.lexeme, t.line)
            }
            ParseError::SyntaxError(s) => write!(f, "{}", s),
        }
    }
}

impl Error for ParseError {}
