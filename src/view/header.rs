use ratatui::{
    prelude::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

/// Draws the title component
pub(crate) fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("Git Diff View")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}
