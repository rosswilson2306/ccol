use std::io;

use args::Args;
use clap::Parser;
use config::{get_config_dir, get_config_file};
use dotenv::dotenv;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};
use store::{AppState, CurrentScreen};
use ui::ui;

use crate::error::Result;
use crate::{config::parse_config, ui::traverse_config_tree};

mod app;
mod args;
mod config;
mod error;
mod store;
mod ui;

fn main() -> Result<()> {
    dotenv().ok();
    let _args = Args::parse();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = AppState::new();
    // RUN APP
    run_app(&mut terminal, &mut app)?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut AppState) -> Result<()> {
    let config_dir = get_config_dir()?;
    let config_file = get_config_file(config_dir);
    let config = parse_config(config_file)?;

    let tree_items = traverse_config_tree(config, "".to_string())?;

    loop {
        terminal.draw(|frame| ui(frame, app, &tree_items))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // skip events that are not KeyEventKind::Press
                continue;
            }

            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Editing;
                    }
                    KeyCode::Char('q') => break,
                    _ => {}
                },
                CurrentScreen::Editing => match key.code {
                    _ => {}
                },
            }
        }
    }

    Ok(())
}
