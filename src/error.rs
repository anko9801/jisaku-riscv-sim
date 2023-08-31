use std::{error::Error, fmt, io};

#[derive(Debug)]
pub enum SimError {
    ParseError(String),
    IoError(io::Error),
    SimError(String),
}

impl SimError {
    pub fn from(e: String) -> SimError {
        SimError::SimError(e)
    }
}

impl fmt::Display for SimError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SimError::ParseError(e) => write!(f, "parse error: {}", e),
            SimError::IoError(v) => write!(f, "{}", v),
            SimError::SimError(e) => write!(f, "error: {}", e),
        }
    }
}

impl Error for SimError {}

pub type SimResult<T> = Result<T, SimError>;
