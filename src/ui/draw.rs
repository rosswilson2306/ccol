use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tui_tree_widget::Tree;

use crate::store::{AppState, CurrentScreen};

use super::popup::Popup;

pub fn draw(frame: &mut Frame, app: &mut AppState) {
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

    let tree_menu = Tree::new(&app.tree_items)
        .expect("all item identifiers are unique")
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Gray))
        .block(menu_block);

    frame.render_stateful_widget(tree_menu, chunks[1], &mut app.tree_state);

    let current_mode_text = vec![match app.current_screen {
        CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
        CurrentScreen::Editing => Span::styled("Editing Mode", Style::default().fg(Color::Yellow)),
    }
    .to_owned()];

    let mode_footer =
        Paragraph::new(Line::from(current_mode_text)).block(Block::default().borders(Borders::ALL));

    let keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => vec![
                Span::styled("(q) quit", Style::default().fg(Color::Blue)),
                Span::styled(" | ", Style::default().fg(Color::Red)),
                Span::styled("(e) edit node", Style::default().fg(Color::Blue)),
                Span::styled(" | ", Style::default().fg(Color::Red)),
                Span::styled("(Enter) toggle / open", Style::default().fg(Color::Blue)),
            ],
            CurrentScreen::Editing => {
                vec![
                    Span::styled("(Esc) normal", Style::default().fg(Color::Blue)),
                    Span::styled(" | ", Style::default().fg(Color::Red)),
                    Span::styled("(Enter) save change", Style::default().fg(Color::Blue)),
                ]
            }
        }
    };

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(chunks[2]);

    let keys_hint_block =
        Paragraph::new(Line::from(keys_hint)).block(Block::default().borders(Borders::ALL));

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(keys_hint_block, footer_chunks[1]);

    if let CurrentScreen::Editing = app.current_screen {
        let area = frame.area();
        let popup_area = Rect {
            x: area.width / 4,
            y: area.height / 3,
            width: area.width / 2,
            height: area.height / 3,
        };

        let popup = Popup::default()
            .content("Edit command")
            .style(Style::new().yellow())
            .title("Edit command")
            .title_style(Style::new().white().bold())
            .border_style(Style::new().red());

        frame.render_widget(popup, popup_area);
    }
}
