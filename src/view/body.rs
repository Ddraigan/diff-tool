use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Row, Table},
    Frame,
};

use crate::{
    model::App,
    services::git::{DiffKind, DiffLine},
};

pub(super) fn render_body(model: &mut App, f: &mut Frame, area: Rect) {
    if model.diff().is_none() {
        // This should be a widget rather than an error
        panic!("No Diff was able to be drawn")
    }

    // Body Layout (Left Diff & Right Diff)
    let [left_side, right_side] =
        Layout::horizontal(Constraint::from_percentages([50, 50])).areas(area);

    let line_number_char_len = model.diff().unwrap().largest_line_number_char_len();

    // Old/Left Diff
    let old_diff = model.diff().unwrap().old_diff();
    let old_diff_table = build_diff_table(old_diff, false, line_number_char_len);
    let mut old_diff_state = model.diff_state().old_diff().borrow_mut();

    // Current/Right Diff
    let current_diff = model.diff().unwrap().current_diff();
    let current_diff_table = build_diff_table(current_diff, true, line_number_char_len);
    let mut current_diff_state = model.diff_state().current_diff().borrow_mut();

    f.render_stateful_widget(old_diff_table, left_side, &mut old_diff_state);
    f.render_stateful_widget(current_diff_table, right_side, &mut current_diff_state)
}

/// Draws a diff table
fn build_diff_table(diff: &[DiffLine], is_current_diff: bool, line_number_char_len: u16) -> Table {
    let diff_title = if is_current_diff { "New" } else { "Original" };

    let rows = diff.iter().map(parse_diff_line);

    // Dynamic column width
    let widths = [
        // Line Number col depends on the largest line number
        Constraint::Length(line_number_char_len),
        Constraint::Percentage(2),
        Constraint::Percentage(97),
    ];

    Table::new(rows, widths)
        .block(
            Block::bordered()
                .title(Span::styled(
                    diff_title,
                    Style::default().fg(Color::LightCyan),
                ))
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
        .highlight_style(if is_current_diff {
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        })
        .highlight_symbol(">>")
}

fn parse_diff_line(line: &DiffLine) -> Row {
    // TODO: The styling should be a property of the model
    let line_number_style = Style::default().fg(Color::Gray);

    let (prefix_style, content_style) = match line.kind() {
        DiffKind::Addition => (
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
            Style::default()
                .bg(Color::Rgb(131, 242, 140))
                .fg(Color::Black),
        ),
        DiffKind::Removal => (
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            Style::default().bg(Color::LightRed).fg(Color::Black),
        ),
        DiffKind::Neutral => (Style::default(), Style::default()),
        DiffKind::Blank => (Style::default(), Style::default().bg(Color::DarkGray)),
    };

    let line_number = match line.line_number() {
        Some(x) => x.to_string(),
        None => " ".to_string(),
    };

    let prefix = line.kind().value();
    let content = line.content();

    Row::new([
        Line::styled(line_number, line_number_style).right_aligned(),
        Line::styled(prefix, prefix_style).centered(),
        Line::styled(content, content_style),
    ])
}
