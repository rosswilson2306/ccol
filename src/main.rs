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
use tui_tree_widget::{TreeItem, TreeState};
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

    let tree_state = TreeState::<String>::default();
    let mut app = AppState::new(tree_state);
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

    let tree_items = traverse_config_tree(config, "root".to_string())?;

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
                        if is_selected_item_a_leaf(&app.tree_state, &tree_items) {
                            // Add logic to copy command to shell
                            break;
                        }
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

pub fn is_selected_item_a_leaf(
    tree_state: &TreeState<String>,
    tree_items: &Vec<TreeItem<'_, String>>,
) -> bool {
    let selected = tree_state.selected();
    let last = match selected.last() {
        Some(identifier) => identifier,
        None => "",
    };
    let flattened_items = tree_state.flatten(&tree_items);
    let matched_item = flattened_items
        .iter()
        .find(|&flattened| flattened.item.identifier() == last)
        .unwrap();

    matched_item.item.children().is_empty()
}
