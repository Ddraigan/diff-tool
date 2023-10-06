use tui::{
    layout::Constraint,
    style::Style,
    text::Span,
    widgets::{Block, Borders, Row, Table},
};

/// Draws the body components
pub(crate) fn draw_body<'a>(table_rows: &Vec<Row<'a>>, diff_title: &'a str) -> Table<'a> {
    /* let largest_line_number = diff
        .iter()
        .map(|x| x.line_number().unwrap_or(0))
        .max()
        .unwrap_or(0);
    let length = std::cmp::min(largest_line_number.to_string().len(), u16::MAX.into());
    let length = length as u16; */

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
        .widths(&[
            Constraint::Length(4),
            Constraint::Percentage(2),
            // Constraint::Percentage(1),
            Constraint::Percentage(97),
        ])
        .column_spacing(1)
        .highlight_style(
            tui::style::Style::default()
                .fg(tui::style::Color::Magenta)
                .add_modifier(tui::style::Modifier::BOLD),
        )
        .highlight_symbol(">>")
}
