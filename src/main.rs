use std::io;

use anyhow::{Context, Result};
use args::Args;
use clap::Parser;
use colored::Colorize;
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
use store::{AppState, CurrentScreen};
use ui::{
    draw::draw,
    json::{get_selected_item, is_selected_item_a_leaf},
};

use crate::config::find_command_in_json;

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

    let mut app = AppState::new()?;

    let output = run_app(&mut terminal, &mut app)?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    let ctx = ClipboardContext::new();

    if let Some(identifier) = output {
        let (key, command) = find_command_in_json(identifier, &app)
            .context("Unablde to find command from identifier")?;

        let message = ctx
            .and_then(|mut clipboard| clipboard.set_contents(command.to_owned()))
            .map(|_| "Command copied to clipboard:")
            .unwrap_or_else(|_| "Unable to copy command to clipboard:");

        println!(
            "\n\n{}\n\n{}\n\n{}\n\n",
            message.bold().blue(),
            key.bold().magenta(),
            command.green()
        );
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut AppState) -> Result<Option<String>> {
    loop {
        terminal.draw(|frame| draw(frame, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // skip events that are not KeyEventKind::Press
                continue;
            }

            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                        if let Some(selected_item) =
                            get_selected_item(&app.tree_state, &app.tree_items)
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
                        if let Some(selected_item) =
                            get_selected_item(&app.tree_state, &app.tree_items)
                        {
                            if is_selected_item_a_leaf(selected_item) {
                                // TODO Add logic to copy command to shell
                                return Ok(Some(selected_item.identifier().clone()));
                            }
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => break,
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
