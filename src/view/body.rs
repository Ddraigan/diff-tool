use ratatui::{
    layout::Constraint,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Row, Table},
};

use crate::{
    git::{DiffKind, DiffLine},
    model::Model,
};

/// Draws a diff table
pub(crate) fn draw_diff_table<'a>(
    model: &Model,
    table_rows: &[Row],
    diff_title: &'a str,
    is_current_diff: bool,
) -> Table<'a> {
    // Dynamic column width
    let col_width = model.diff().largest_line_number_len();

    let col_widths = [
        Constraint::Length(col_width),
        Constraint::Percentage(2),
        Constraint::Percentage(97),
    ];

    Table::new(table_rows, col_widths)
        .block(
            Block::default()
                .title(Span::styled(
                    diff_title,
                    Style::default().fg(Color::LightCyan),
                ))
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(ratatui::widgets::BorderType::Plain),
        )
        .column_spacing(1)
        .highlight_style(if is_current_diff {
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        })
        .highlight_symbol(">>")
}

pub fn parse_diff_rows<'a>(diff_content: &'a [DiffLine]) -> Vec<Row<'a>> {
    diff_content.iter().map(parse_diff_line).collect()
}

fn parse_diff_line(line: &DiffLine) -> Row {
    let prefix_style = match line.kind() {
        DiffKind::Addition => Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
        DiffKind::Removal => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        DiffKind::Neutral => Style::default(),
        DiffKind::Blank => Style::default(),
    };

    let content_style = match line.kind() {
        DiffKind::Addition => Style::default()
            .bg(Color::Rgb(131, 242, 140))
            .fg(Color::Black),
        DiffKind::Removal => Style::default().bg(Color::LightRed).fg(Color::Black),
        DiffKind::Neutral => Style::default(),
        DiffKind::Blank => Style::default().bg(Color::DarkGray),
    };

    let line_number = match line.line_number() {
        Some(x) => x.to_string(),
        None => " ".to_string(),
    };

    let prefix = line.kind().value();
    let content = line.content();

    Row::new([
        Cell::from(Line::from(line_number).alignment(ratatui::prelude::Alignment::Right))
            .style(Style::default().fg(Color::Gray)),
        Cell::from(prefix).style(prefix_style),
        Cell::from(content).style(content_style),
    ])
}
