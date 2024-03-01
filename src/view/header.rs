use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

/// Draws the title component
pub(crate) fn draw_title<'a>(f: &mut Frame, area: Rect) {
    let title = Paragraph::new("Git Diff View")
        .style(Style::default().fg(Color::LightCyan))
        .centered()
        .block(
            Block::bordered()
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        );

    f.render_widget(title, area);
}
