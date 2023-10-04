use tui::{
    style::Style,
    widgets::{Block, Borders, Paragraph},
};

/// Draws the title component
pub(crate) fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("Git Diff View")
        .style(Style::default().fg(tui::style::Color::LightCyan))
        .alignment(tui::layout::Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(tui::style::Color::White))
                .border_type(tui::widgets::BorderType::Plain),
        )
}
