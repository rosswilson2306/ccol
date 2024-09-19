use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use serde::Deserialize;

use crate::error::{Result, CcolError};

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Tree {
    Leaf(String),
    Branch(HashMap<String, Tree>),
}

pub fn parse_config() -> Result<Tree> {
    let config_path = get_config_file()?.display().to_string();
    dbg!(&config_path);

    let file = File::open(config_path)?;
    let reader = BufReader::new(file);

    let contents: Tree = serde_json::from_reader(reader)?;

    dbg!(&contents);

    Ok(contents)
}

pub fn get_config_dir() -> Result<PathBuf> {
    let directory = if let Ok(v) = std::env::var("CCOL_CONFIG_PATH") {
        PathBuf::from(v)
    } else if let Some(project_dirs) = ProjectDirs::from("io", "rosswilson", "ccol") {
        project_dirs.config_local_dir().to_path_buf()
    } else {
        return Err(CcolError::MissingConfigDirectory(
            "Unable to find config directory for ccol".to_string(),
        ));
    };
    Ok(directory)
}

pub fn get_config_file() -> Result<PathBuf> {
    let config_dir = get_config_dir()?.display().to_string();
    let config_file_str = format!("{config_dir}/ccol.json");

    let path_buf = Path::new(&config_file_str).to_path_buf();
    Ok(path_buf)
}
