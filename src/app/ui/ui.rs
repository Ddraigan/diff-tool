use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Row, TableState},
    Frame,
};

use crate::{app::app::App, git::git::Diff};

use super::{
    body::draw_diff_table,
    footer::{draw_console, draw_help},
    header::draw_title,
};

/// Draws all the components
pub fn draw<B>(
    rect: &mut Frame<B>,
    app: &App,
    mut body_left_state: &mut TableState,
    mut body_right_state: &mut TableState,
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

    // Dynamic column width
    let col_width = app
        .state()
        .diff()
        .unwrap_or(&Diff::default())
        .largest_line_number_len();

    let col_widths = [
        Constraint::Length(col_width),
        Constraint::Percentage(2),
        // Constraint::Percentage(1),
        Constraint::Percentage(97),
    ];

    // Left Diff
    let body_left = draw_diff_table(&diff_one_rows, "Original", false, &col_widths);
    rect.render_stateful_widget(body_left, body_chunks[0], &mut body_left_state);

    // Right Diff
    let body_right = draw_diff_table(&diff_two_rows, "New", true, &col_widths);
    rect.render_stateful_widget(body_right, body_chunks[1], &mut body_right_state);

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
