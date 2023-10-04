use tui::{
    style::Style,
    text::Spans,
    widgets::{Block, Borders, Paragraph},
};

use crate::git::git::DiffLine;

/// Draws the body components
pub(crate) fn draw_body<'a>(diff: &'a Vec<DiffLine>) -> Paragraph<'a> {
    /* let text = Spans::from(vec![
        Span::raw("First"),
        Span::styled("line", Style::default().add_modifier(Modifier::ITALIC)),
        Span::raw("."),
    ]); */

    let text: Vec<Spans> = diff.iter().map(parse_diff_line).collect();

    Paragraph::new(text)
        .style(Style::default().fg(tui::style::Color::LightGreen))
        .alignment(tui::layout::Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(tui::style::Color::White))
                .border_type(tui::widgets::BorderType::Plain),
        )
}

fn parse_diff_line(line: &DiffLine) -> Spans {
    Spans::from(line.content().to_string())
}
