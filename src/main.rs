use std::{error::Error, io};

use args::Args;
use clap::Parser;
use dotenv::dotenv;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{DisableMouseCapture, EnableMouseCapture},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

use crate::config::{get_config_file, parse_config};

mod app;
mod args;
mod config;
mod store;
mod error;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let _args = Args::parse();

    let config_file = get_config_file();
    dbg!(&config_file);

    parse_config()?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // RUN APP
    run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) {
    //
}
