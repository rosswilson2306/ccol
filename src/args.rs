use std::path::PathBuf;

use clap::{self, Parser};

use crate::config::get_config_dir;

pub fn about() -> String {
    let author = env!("CARGO_PKG_AUTHORS");

    let config_dir = get_config_dir()
        .unwrap_or_else(|_| PathBuf::from("$HOME/.config/ccol/"))
        .to_string_lossy()
        .into_owned();

    format!(
        "Authors: {author}

Config directory: {config_dir}"
    )
}

#[derive(Debug, Parser)]
#[command(version, about = about())]
pub struct Args;
