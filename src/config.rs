use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufReader, Write},
    path::PathBuf,
};

use directories::ProjectDirs;
use serde::Deserialize;
use serde_json::json;

use crate::error::{CcolError, Result};

#[derive(Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum CollectionTree {
    Leaf(String),
    Branch(HashMap<String, CollectionTree>),
}

pub fn parse_config(config_path: PathBuf) -> Result<HashMap<String, CollectionTree>> {
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

    let contents: HashMap<String, CollectionTree> = serde_json::from_reader(reader)?;

    Ok(contents)
}

pub fn get_config_dir() -> Result<PathBuf> {
    let directory = if let Ok(v) = std::env::var("CCOL_CONFIG_PATH") {
        PathBuf::from(v).join("ccol")
    } else if let Some(project_dirs) = ProjectDirs::from("io", "rosswilson", "ccol") {
        project_dirs.config_local_dir().to_path_buf()
    } else {
        return Err(CcolError::MissingConfigDirectory);
    };

    if !directory.exists() {
        fs::create_dir_all(&directory).map_err(|_| CcolError::FileIO)?;
    }

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

        let temp_confi_dir = temp_dir.path().join("ccol");

        assert_eq!(config_dir, temp_confi_dir);

        env::remove_var("CCOL_CONFIG_PATH");
        Ok(())
    }

    #[test]
    fn config_file_exists() -> std::result::Result<(), Box<dyn Error>> {
        let temp_dir = tempfile::tempdir()?;
        env::set_var("CCOL_CONFIG_PATH", temp_dir.path());

        let config_dir = get_config_dir()?;

        assert!(config_dir.exists());
        assert!(config_dir.is_dir());

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

    #[test]
    fn parse_config_creates_file_with_default_content() -> std::result::Result<(), Box<dyn Error>> {
        let temp_dir = tempfile::tempdir()?;
        let temp_path = temp_dir.path().join("ccol.json");

        let result = parse_config(temp_path.clone());

        assert!(result.is_ok());
        assert!(temp_path.exists());

        let file = File::open(temp_path)?;
        let reader = BufReader::new(file);
        let contents: CollectionTree = serde_json::from_reader(reader)?;
        let expected: CollectionTree = serde_json::from_value(json!({}))?;

        assert_eq!(contents, expected);

        Ok(())
    }
}
