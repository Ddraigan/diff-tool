use log::error;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Row, Table},
    Frame,
};

use crate::{
    model::Model,
    services::git::{DiffKind, DiffLine},
};

pub(crate) fn draw_body(model: &mut Model, f: &mut Frame, area: Rect) {
    if model.diff().is_some() {
        // Body Layout (Left Diff & Right Diff)
        let [left_side, right_side] =
            Layout::horizontal(Constraint::from_percentages([50, 50])).areas(area);

        // Old/Left Diff
        render_diff_table(model, f, left_side, false);

        // Current/Right Diff
        return render_diff_table(model, f, right_side, true);
    }
    // This should be a widget rather than an error
    error!("No Diff was able to be drawn")
}

/// Draws a diff table
fn render_diff_table(model: &Model, f: &mut Frame, area: Rect, is_current_diff: bool) {
    let diff = if is_current_diff {
        model.diff().expect("Diff to exist").current_diff()
    } else {
        model.diff().expect("Diff to exist").old_diff()
    };

    let diff_title = if is_current_diff { "New" } else { "Original" };

    let rows = parse_diff_rows(diff);

    // Dynamic column width
    let widths = [
        // Line Number col depends on the largest line number
        Constraint::Length(
            model
                .diff()
                .expect("Diff to exist")
                .largest_line_number_char_len(),
        ),
        Constraint::Percentage(2),
        Constraint::Percentage(97),
    ];

    let table = Table::new(rows, widths)
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
        .highlight_symbol(">>");

    let mut state = if is_current_diff {
        model.current_diff_state().borrow_mut()
    } else {
        model.old_diff_state().borrow_mut()
    };

    f.render_stateful_widget(table, area, &mut state);
}

fn parse_diff_rows(diff_content: &[DiffLine]) -> Vec<Row> {
    diff_content.iter().map(parse_diff_line).collect()
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
