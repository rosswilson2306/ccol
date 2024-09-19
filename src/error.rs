use std::io;

#[derive(Debug)]
pub enum CcolError {
    CorruptedConfig(String),
    MissingConfigDirectory(String),
    FileIO(String),
}

impl From<io::Error> for CcolError {
    fn from(_error: io::Error) -> Self {
        CcolError::FileIO("error in file i/o".to_string())
    }
}

impl From<serde_json::Error> for CcolError {
    fn from(_error: serde_json::Error) -> Self {
        CcolError::CorruptedConfig("could not parse config file".to_string())
    }
}

pub type Result<T> = std::result::Result<T, CcolError>;
