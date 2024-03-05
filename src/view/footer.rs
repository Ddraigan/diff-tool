use std::collections::{HashMap, HashSet};

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Row, Table},
    Frame,
};

use crate::{model::App, update::message::Message};

pub(super) fn render_footer(app: &App, area: Rect, f: &mut Frame) {
    // Footer Layout (Console & Help)
    let [_left, right] = Layout::horizontal(Constraint::from_percentages([50, 50])).areas(area);

    // // Console Section
    // let console = draw_console(model);
    // f.render_widget(console, footer_chunks[0]);

    // Help Menu
    let help_menu = draw_help(app);

    f.render_widget(help_menu, right);
}

fn combine_keys_by_value(map: &HashMap<String, Message>) -> Vec<(Vec<String>, &Message)> {
    let mut result = Vec::new();
    let mut processed_messages = HashSet::new();

    for (_, message) in map.iter() {
        if !processed_messages.contains(message) {
            let keys_with_same_message: Vec<String> = map
                .iter()
                .filter(|(_, m)| *m == message)
                .map(|(k, _)| k.clone())
                .collect();
            result.push((keys_with_same_message, message));
            processed_messages.insert(message);
        }
    }

    result
}

// Draws the help menu component
fn draw_help(app: &App) -> Table {
    let key_style = Style::default().fg(Color::LightCyan);
    let message_style = Style::default().fg(Color::Gray);

    let keymaps = app.config().keymap();
    let keymaps = combine_keys_by_value(keymaps);

    let keymaps = keymaps.iter().map(|(keybinds, message)| {
        Row::new([
            Line::styled(keybinds.join(" | "), key_style),
            Line::styled(message.to_string(), message_style),
        ])
    });

    Table::new(keymaps, Constraint::from_percentages([20, 80])).block(
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
