use std::{io, path::PathBuf};

use clap::{self, Parser};
use directories::ProjectDirs;

fn get_config_dir() -> Result<PathBuf, io::Error> {
    let directory = if let Ok(v) = std::env::var("CCOL_CONFIG_PATH") {
        PathBuf::from(v)
    } else if let Some(project_dirs) = ProjectDirs::from("io", "rosswilson", "ccol") {
        project_dirs.config_local_dir().to_path_buf()
    } else {
        // TODO: custom error
        return Err(io::Error::new(io::ErrorKind::Other, "Unable to find config directory for ccol"));
    };
    Ok(directory)
}

pub fn about() -> String {
    let author = env!("CARGO_PKG_AUTHORS");

    let config_dir = get_config_dir().unwrap().display().to_string();

    format!("Authors: {author}

Config directory: {config_dir}")
}

#[derive(Debug, Parser)]
#[command(version, about = about())]
pub struct Args;

