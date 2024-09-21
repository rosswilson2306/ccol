use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Write},
    path::PathBuf,
};

use directories::ProjectDirs;
use serde::Deserialize;
use serde_json::json;

use crate::error::{CcolError, Result};

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Tree {
    Leaf(String),
    Branch(HashMap<String, Tree>),
}

pub fn parse_config(config_path: PathBuf) -> Result<Tree> {
    let file = match File::open(&config_path) {
        Ok(f) => f,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let mut new_config_file = File::create(&config_path)?;
            let default_content = json!({});

            new_config_file.write_all(default_content.to_string().as_bytes())?;
            new_config_file.sync_all()?;
            File::open(&config_path)?
        }
        Err(_) => return Err(CcolError::FileIO),
    };
    let reader = BufReader::new(file);

    let contents: Tree = serde_json::from_reader(reader)?;

    Ok(contents)
}

pub fn get_config_dir() -> Result<PathBuf> {
    let directory = if let Ok(v) = std::env::var("CCOL_CONFIG_PATH") {
        PathBuf::from(v)
    } else if let Some(project_dirs) = ProjectDirs::from("io", "rosswilson", "ccol") {
        project_dirs.config_local_dir().to_path_buf()
    } else {
        return Err(CcolError::MissingConfigDirectory);
    };
    Ok(directory)
}

pub fn get_config_file(mut config_dir: PathBuf) -> PathBuf {
    config_dir.push("ccol.json");
    config_dir
}

#[cfg(test)]
mod tests {
    use std::{env, error::Error};

    use super::*;

    #[test]
    fn get_config_dir_with_env() -> std::result::Result<(), Box<dyn Error>> {
        let temp_dir = tempfile::tempdir()?;
        env::set_var("CCOL_CONFIG_PATH", temp_dir.path());

        let config_dir = get_config_dir()?;

        assert_eq!(config_dir, temp_dir.path());

        env::remove_var("CCOL_CONFIG_PATH");
        Ok(())
    }

    #[test]
    fn config_file_name() -> std::result::Result<(), Box<dyn Error>> {
        let mut temp_dir = tempfile::tempdir()?.path().to_path_buf();
        let config_file = get_config_file(temp_dir.clone());
        temp_dir.push("ccol.json");

        assert_eq!(config_file, temp_dir);
        Ok(())
    }
}
