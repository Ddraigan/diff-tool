use tui::{
    layout::Constraint,
    style::{Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
};

use crate::git::git::{DiffKind, DiffLine};

/// Draws the body components
pub(crate) fn draw_body<'a>(diff: &'a Vec<DiffLine>) -> Table<'a> {
    /* let largest_line_number = diff
        .iter()
        .map(|x| x.line_number().unwrap_or(0))
        .max()
        .unwrap_or(0);
    let length = cmp::min(largest_line_number.to_string().len(), u16::MAX.into());
    let length = length as u16; */

    let lines: Vec<Row> = diff.iter().map(parse_diff_line).collect();

    Table::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(tui::style::Color::White))
                .border_type(tui::widgets::BorderType::Plain),
        )
        .widths(&[
            Constraint::Length(4),
            Constraint::Percentage(2),
            Constraint::Percentage(1),
            Constraint::Percentage(97),
        ])
        .column_spacing(0)
}

fn parse_diff_line(line: &DiffLine) -> Row {
    let prefix_style = match line.kind() {
        DiffKind::Addition => Style::default()
            .fg(tui::style::Color::Green)
            .add_modifier(Modifier::BOLD),
        DiffKind::Removal => Style::default()
            .fg(tui::style::Color::Red)
            .add_modifier(Modifier::BOLD),
        DiffKind::Neutral => Style::default(),
        DiffKind::Blank => Style::default(),
    };

    let content_style = match line.kind() {
        DiffKind::Addition => Style::default()
            .bg(tui::style::Color::Rgb(131, 242, 140))
            .fg(tui::style::Color::Black),
        DiffKind::Removal => Style::default()
            .bg(tui::style::Color::LightRed)
            .fg(tui::style::Color::Black),
        DiffKind::Neutral => Style::default(),
        DiffKind::Blank => Style::default().bg(tui::style::Color::DarkGray),
    };

    let line_number = match line.line_number() {
        Some(x) => x.to_string(),
        None => " ".to_string(),
    };

    let prefix = line.kind().value();
    let content = line.content();

    Row::new(vec![
        Cell::from(line_number).style(Style::default().fg(tui::style::Color::Gray)),
        Cell::from(prefix).style(prefix_style),
        Cell::from("").style(content_style),
        Cell::from(content).style(content_style),
    ])
}
