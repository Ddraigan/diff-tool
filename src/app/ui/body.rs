use tui::{
    layout::Constraint,
    style::Style,
    text::Span,
    widgets::{Block, Borders, Row, Table},
};

use crate::git::git::{DiffKind, DiffLine};

/// Draws the body components
pub(crate) fn draw_body<'a>(diff: &'a Vec<DiffLine>) -> Table<'a> {
    /* let text = Spans::from(vec![
        Span::raw("First"),
        Span::styled("line", Style::default().add_modifier(Modifier::ITALIC)),
        Span::raw("."),
    ]); */

    let lines: Vec<Row> = diff.iter().map(parse_diff_line).collect();

    Table::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(tui::style::Color::White))
                .border_type(tui::widgets::BorderType::Plain),
        )
        .widths(&[Constraint::Percentage(100)])
        .column_spacing(1)
}

fn parse_diff_line(line: &DiffLine) -> Row {
    let style = match line.kind() {
        DiffKind::Addition => Style::default().bg(tui::style::Color::LightGreen),
        DiffKind::Removal => Style::default().bg(tui::style::Color::LightRed),
        DiffKind::Neutral => Style::default(),
        DiffKind::Blank => Style::default().bg(tui::style::Color::DarkGray),
    };

    Row::new(vec![Span::from(line.content())]).style(style)
}
