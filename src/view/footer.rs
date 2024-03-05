use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Cell, Row, Table},
    Frame,
};

use crate::model::App;

pub fn render_footer(app: &App, area: Rect, f: &mut Frame) {
    // Footer Layout (Console & Help)
    let [_left, right] = Layout::horizontal(Constraint::from_percentages([50, 50])).areas(area);

    // // Console Section
    // let console = draw_console(model);
    // f.render_widget(console, footer_chunks[0]);

    // Help Menu
    let help_menu = draw_help(app);
    f.render_widget(help_menu, right);
}

/// Draws the help menu component
fn draw_help(app: &App) -> Table {
    let key_style = Style::default().fg(Color::LightCyan);
    let message_style = Style::default().fg(Color::Gray);

    let keys = app.config().keymap().iter().map(|(key, message)| {
        Row::new([
            Cell::from(Span::styled(key, key_style)),
            Cell::from(Span::styled(message.to_string(), message_style)),
        ])
    });

    Table::new(keys, Constraint::from_percentages([20, 80])).block(
        Block::bordered()
            .border_type(BorderType::Plain)
            .title("Help"),
    )
}

// fn draw_console(model: &Model) -> Table {
//     let lines: Vec<Row> = state
//         .console()
//         .unwrap_or(&vec![])
//         .iter()
//         .map(|item| Row::new(vec![Cell::from(Span::from(item.to_owned()))]))
//         .collect();
//
//     Table::new(lines)
//         .block(
//             Block::default()
//                 .borders(Borders::ALL)
//                 .style(Style::default().fg(Color::White))
//                 .border_type(BorderType::Plain),
//         )
//         .widths(&[Constraint::Length(11), Constraint::Min(20)])
//         .column_spacing(1)
// }
