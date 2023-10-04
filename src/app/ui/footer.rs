use tui::{
    layout::Constraint,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Cell, Row, Table},
};

use crate::app::{actions::Actions, state::AppState};

pub(crate) fn draw_console(state: &AppState) -> Table {
    let lines: Vec<Row> = state
        .console()
        .unwrap_or(&vec![])
        .iter()
        .map(|item| Row::new(vec![Cell::from(Span::from(item.to_owned()))]))
        .collect();

    Table::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(tui::widgets::BorderType::Plain),
        )
        .widths(&[Constraint::Length(11), Constraint::Min(20)])
        .column_spacing(1)
}

/// Draws the help menu component
pub(crate) fn draw_help(actions: &Actions) -> Table {
    let key_style = Style::default().fg(Color::LightCyan);
    let help_style = Style::default().fg(Color::Gray);

    let mut lines = vec![];
    for action in actions.actions().iter() {
        let mut first = true;
        for key in action.keys() {
            let help = if first {
                first = false;
                action.to_string()
            } else {
                String::from("")
            };
            let row = Row::new(vec![
                Cell::from(Span::styled(key.to_string(), key_style)),
                Cell::from(Span::styled(help, help_style)),
            ]);
            lines.push(row);
        }
    }

    Table::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(tui::widgets::BorderType::Plain)
                .title("Help"),
        )
        .widths(&[Constraint::Length(11), Constraint::Min(20)])
        .column_spacing(1)
}
