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
    let help_menu = build_help_table(app);
    f.render_widget(help_menu, right);
}

fn combine_keys_by_value(map: &HashMap<String, Message>) -> Vec<(String, &Message)> {
    let mut result = Vec::new();
    let mut processed_messages = HashSet::new();

    for (_, message) in map.iter() {
        if !processed_messages.contains(message) {
            let combined_keys = map
                .into_iter()
                .filter(|(_, m)| *m == message)
                .enumerate()
                .map(|(i, (k, _))| {
                    if i == 0 {
                        k.to_owned().to_uppercase()
                    } else {
                        format!(" | {}", k.to_uppercase())
                    }
                })
                .collect::<String>();
            result.push((combined_keys, message));
            processed_messages.insert(message);
        }
    }

    result
}

fn longest_combined_keymap(combined_keymaps: &Vec<(String, &Message)>) -> u16 {
    combined_keymaps
        .iter()
        .map(|(keys, _)| keys.len().try_into().unwrap())
        .max()
        .unwrap()
}

// Draws the help menu component
fn build_help_table(app: &App) -> Table {
    let key_style = Style::default().fg(Color::LightCyan);
    let message_style = Style::default().fg(Color::Gray);

    let keymaps = app.config().keymap();
    let combined_keymaps = combine_keys_by_value(keymaps);

    let longest_string = longest_combined_keymap(&combined_keymaps);
    let widths = [Constraint::Length(longest_string), Constraint::Min(10)];

    let keymap_rows = combined_keymaps.iter().map(|(keybinds, message)| {
        Row::new([
            Line::styled(keybinds.to_owned(), key_style),
            Line::styled(message.to_string(), message_style),
        ])
    });

    Table::new(keymap_rows, widths).block(
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
