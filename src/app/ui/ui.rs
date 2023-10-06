use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Cell, Row, TableState},
    Frame,
};

use crate::{
    app::app::App,
    git::git::{DiffKind, DiffLine},
};

use super::{
    body::draw_body,
    footer::{draw_console, draw_help},
    header::draw_title,
};

/// Draws all the components
pub fn draw<B>(
    rect: &mut Frame<B>,
    app: &App,
    mut diff_two_state: &mut TableState,
    diff_one_rows: &Vec<Row>,
    diff_two_rows: &Vec<Row>,
) where
    B: Backend,
{
    // Term size
    let size = rect.size();
    check_size(&size);

    // Vertical Layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Max(10),
            ]
            .as_ref(),
        )
        .split(size);

    // Title at top
    let title = draw_title();
    rect.render_widget(title, chunks[0]);

    // Body Layout (Left Diff & Right Diff)
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    // Left Diff
    let body_left = draw_body(&diff_one_rows, "Original");
    rect.render_widget(body_left, body_chunks[0]);

    // Right Diff
    let body_right = draw_body(&diff_two_rows, "New");
    rect.render_stateful_widget(body_right, body_chunks[1], &mut diff_two_state);

    // Footer Layout (Console & Help)
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(chunks[2]);

    // Console Section
    let console = draw_console(app.state());
    rect.render_widget(console, footer_chunks[0]);

    // Help Menu
    let help_menu = draw_help(app.actions());
    rect.render_widget(help_menu, footer_chunks[1]);
}

/// Checks terminal size is large enough
fn check_size(rect: &Rect) {
    if rect.width < 52 {
        panic!("Require width >= 52, (got {})", rect.width);
    }
    if rect.height < 28 {
        panic!("Require height >= 28, (got {})", rect.height);
    }
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
