use std::fmt::Display;

#[derive(Debug)]
pub enum ResolverError {
    ScopingError(String),
}

impl std::fmt::Display for ResolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolverError::ScopingError(message) => write!(f, "{}", message),
        }
    }
}
