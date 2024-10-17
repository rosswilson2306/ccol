use std::{fmt, io};

#[derive(Debug)]
pub enum CcolError {
    CorruptedConfig(serde_json::Error),
    MissingConfigDirectory,
    ParseConfigError,
    FileIO(io::Error),
}

impl From<io::Error> for CcolError {
    fn from(error: io::Error) -> Self {
        CcolError::FileIO(error)
    }
}

impl From<serde_json::Error> for CcolError {
    fn from(error: serde_json::Error) -> Self {
        CcolError::CorruptedConfig(error)
    }
}

impl fmt::Display for CcolError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CcolError::CorruptedConfig(_) => write!(formatter, "could not parse config file"),
            CcolError::ParseConfigError => write!(formatter, "could not parse config file"),
            CcolError::MissingConfigDirectory => {
                write!(formatter, "could not find config directory")
            }
            CcolError::FileIO(_) => write!(formatter, "error in file i/o"),
        }
    }
}

impl std::error::Error for CcolError {}

pub type Result<T> = std::result::Result<T, CcolError>;
