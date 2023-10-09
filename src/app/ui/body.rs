use tui::{
    layout::Constraint,
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Cell, Row, Table},
};

use crate::git::git::{DiffKind, DiffLine};

/// Draws a diff table
pub(crate) fn draw_diff_table<'a>(
    table_rows: &Vec<Row<'a>>,
    diff_title: &'a str,
    is_diff_two: bool,
    col_widths: &'a [Constraint],
) -> Table<'a> {
    Table::new(table_rows.to_owned())
        .block(
            Block::default()
                .title(Span::styled(
                    diff_title,
                    Style::default().fg(tui::style::Color::LightCyan),
                ))
                .borders(Borders::ALL)
                .style(Style::default().fg(tui::style::Color::White))
                .border_type(tui::widgets::BorderType::Plain),
        )
        .widths(&col_widths)
        .column_spacing(1)
        .highlight_style(if is_diff_two {
            Style::default()
                .fg(tui::style::Color::Magenta)
                .add_modifier(tui::style::Modifier::BOLD)
        } else {
            Style::default()
        })
        .highlight_symbol(">>")
}

pub(crate) fn parse_diff_rows<'a>(diff_content: &'a Vec<DiffLine>) -> Vec<Row<'a>> {
    diff_content.iter().map(parse_diff_line).collect()
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
        // Cell::from("").style(content_style),
        Cell::from(content).style(content_style),
    ])
}
