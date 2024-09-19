use clap::{self, Parser};

use crate::config::get_config_dir;

pub fn about() -> String {
    let author = env!("CARGO_PKG_AUTHORS");

    let config_dir = get_config_dir().unwrap().display().to_string();

    format!(
        "Authors: {author}

Config directory: {config_dir}"
    )
}

#[derive(Debug, Parser)]
#[command(version, about = about())]
pub struct Args;
