use std::rc::Rc;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

use crate::{
    model::Model,
    services::git::{DiffKind, DiffLine},
};

pub(crate) fn draw_body(model: &mut Model, f: &mut Frame, area: Rc<[Rect]>) {
    if model.diff().is_some() {
        // Body Layout (Left Diff & Right Diff)
        let body_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area[1]);
        // Old/Left Diff
        render_diff_table(model, f, body_area[0], false);

        // Current/Right Diff
        return render_diff_table(model, f, body_area[1], true);
    }
    // TODO: Some sort of widget in place of a no diff
    unimplemented!()
}

/// Draws a diff table
fn render_diff_table(model: &Model, f: &mut Frame, area: Rect, is_current_diff: bool) {
    let diff = if is_current_diff {
        model.current_diff()
    } else {
        model.old_diff()
    };

    let diff_title = if is_current_diff { "New" } else { "Original" };

    let rows = parse_diff_rows(diff);

    // Dynamic column width
    let col_widths = [
        Constraint::Length(
            model
                .diff()
                .expect("Must have a diff to get this far")
                .largest_line_number_len(),
        ),
        Constraint::Percentage(2),
        Constraint::Percentage(97),
    ];

    let table = Table::new(rows, col_widths)
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
