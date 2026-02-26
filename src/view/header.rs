use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};

pub(super) fn render_header(f: &mut Frame, area: Rect) {
    let title = draw_title();
    f.render_widget(title, area);
}

/// Draws the title component
fn draw_title<'a>() -> Paragraph<'a> {
    // TODO: instead of Git Diff View, this should be the path and file we are viewing
    Paragraph::new("Git Diff View")
        .style(Style::default().fg(Color::LightCyan))
        .centered()
        .block(
            Block::bordered()
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}
