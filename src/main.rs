use std::io;

use args::Args;
use clap::Parser;
use colored::Colorize;
use config::{get_config_dir, get_config_file, parse_config};
use copypasta::{ClipboardContext, ClipboardProvider};
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
use serde_json::Value;
use store::{AppState, CurrentScreen};
use ui::{
    draw::draw,
    json::{get_selected_item, is_selected_item_a_leaf, tree_items},
};

use crate::{
    config::find_command_in_json,
    error::{CcolError, Result},
};

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

    let output = run_app(&mut terminal, &mut app)?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    let mut ctx = ClipboardContext::new().unwrap(); // TODO;

    if let Some(identifier) = output {
        let (key, command) =
            find_command_in_json(identifier, &app).ok_or(CcolError::ParseConfigError)?;

        ctx.set_contents(command.to_owned()).unwrap(); // TODO
        println!(
            "{} {} {}",
            "Command copied to clipboard:".bold().blue(),
            key.bold().magenta(),
            command.green()
        );
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut AppState) -> Result<Option<String>> {
    let config_dir = get_config_dir()?;
    let config_file = get_config_file(config_dir);
    let config = parse_config(config_file)?;
    let tree_items = tree_items(&config)?;

    match config.clone() {
        Value::Object(o) => app.config = Some(o),
        _ => return Err(CcolError::ParseConfigError),
    }

    loop {
        terminal.draw(|frame| draw(frame, app, &tree_items))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // skip events that are not KeyEventKind::Press
                continue;
            }

            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                        if let Some(selected_item) = get_selected_item(&app.tree_state, &tree_items)
                        {
                            if is_selected_item_a_leaf(selected_item) {
                                let _json_result =
                                    find_command_in_json(selected_item.identifier().clone(), app);
                            }
                        }

                        app.current_screen = CurrentScreen::Editing;
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        app.tree_state.key_up();
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        app.tree_state.key_down();
                    }
                    KeyCode::Char('l') => {
                        app.tree_state.key_right();
                    }
                    KeyCode::Char('h') => {
                        app.tree_state.key_left();
                    }
                    KeyCode::Enter => {
                        app.tree_state.toggle_selected();
                        if let Some(selected_item) = get_selected_item(&app.tree_state, &tree_items)
                        {
                            if is_selected_item_a_leaf(selected_item) {
                                // TODO Add logic to copy command to shell
                                return Ok(Some(selected_item.identifier().clone()));
                            }
                        }
                    }
                    KeyCode::Char('q') => break,
                    _ => {}
                },
                CurrentScreen::Editing => match key.code {
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                },
            }
        }
    }

    Ok(None)
}
