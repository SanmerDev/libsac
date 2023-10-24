use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
#[non_exhaustive]
pub enum SacError {
    Unsupported(String),
    IO(String),
}

impl Display for SacError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SacError::Unsupported(msg) => write!(f, "{}", msg),
            SacError::IO(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for SacError {}
