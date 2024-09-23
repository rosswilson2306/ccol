use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::store::AppState;

pub fn ui(frame: &mut Frame, app: &AppState) {
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

    let placeholder_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Gray));

    let placeholder = Paragraph::new(Text::styled("Menu", Style::default().fg(Color::Red)))
        .block(placeholder_block)
        .centered();

    frame.render_widget(placeholder, chunks[1]);
}
