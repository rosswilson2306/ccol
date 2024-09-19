use std::{fmt, io};

#[derive(Debug)]
pub enum CcolError {
    CorruptedConfig,
    MissingConfigDirectory,
    FileIO,
}

impl From<io::Error> for CcolError {
    fn from(_error: io::Error) -> Self {
        CcolError::FileIO
    }
}

impl From<serde_json::Error> for CcolError {
    fn from(_error: serde_json::Error) -> Self {
        CcolError::CorruptedConfig
    }
}

impl fmt::Display for CcolError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CcolError::CorruptedConfig => write!(formatter, "could not parse config file"),
            CcolError::MissingConfigDirectory => {
                write!(formatter, "could not find config directory")
            }
            CcolError::FileIO => write!(formatter, "error in file i/o"),
        }
    }
}

impl std::error::Error for CcolError {}

pub type Result<T> = std::result::Result<T, CcolError>;
