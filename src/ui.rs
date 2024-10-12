use std::collections::HashMap;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tui_tree_widget::{Tree, TreeItem};

use crate::error::Result;
use crate::{config::CollectionTree, store::AppState};

pub fn ui(
    frame: &mut Frame,
    app: &mut AppState,
    items: &[TreeItem<String>],
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Gray));

    let title = Paragraph::new(Text::styled(
        "Command Collection",
        Style::default().fg(Color::Blue),
    ))
    .block(title_block)
    .centered();

    frame.render_widget(title, chunks[0]);

    let menu_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Gray));

    let tree_menu = Tree::new(items)
        .expect("all item identifiers are unique")
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Gray))
        .block(menu_block);

    frame.render_stateful_widget(tree_menu, chunks[1], &mut app.tree_state);
}

pub fn traverse_config_tree<'a>(
    config: HashMap<String, CollectionTree>,
    path: String,
) -> Result<Vec<TreeItem<'a, String>>> {
    let mut items = Vec::new();

    for (key, subtree) in config {
        let new_path = format!("{}/{}", path, key);

        let tree_item = match subtree {
            CollectionTree::Leaf(value) => TreeItem::new_leaf(new_path.clone(), format!("{}:  {}", key, value)),
            CollectionTree::Branch(subtree_map) => {
                let children = traverse_config_tree(subtree_map, new_path.clone())?;

                TreeItem::new(new_path.clone(), key, children)?
            }
        };

        items.push(tree_item);
    }

    Ok(items)
}
