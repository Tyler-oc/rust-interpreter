use std::error::Error;
use std::fmt;

use crate::errors::environment_error::EnvironmentError;
use crate::interpreting::value::Value;
#[derive(Debug)]
pub enum RunTimeError {
    CouldNotEval(String),
    EnvironmentError(EnvironmentError),
    CallableError(String),
    Return(Value),
}

impl fmt::Display for RunTimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RunTimeError::CouldNotEval(val) => write!(f, "Could not evaluate: {}", val),
            RunTimeError::EnvironmentError(err) => write!(f, "env error: {}", err),
            RunTimeError::CallableError(c) => write!(f, "{}", c), //more in depth later
            RunTimeError::Return(v) => write!(f, "returning {}", v),
        }
    }
}

impl Error for RunTimeError {}

impl From<EnvironmentError> for RunTimeError {
    fn from(value: EnvironmentError) -> Self {
        RunTimeError::EnvironmentError(value)
    }
}
