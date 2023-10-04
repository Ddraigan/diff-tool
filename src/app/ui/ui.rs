use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::app::app::App;

use super::{
    body::draw_body,
    footer::{draw_console, draw_help},
    header::draw_title,
};

/// Draws all the components
pub fn draw<B>(rect: &mut Frame<B>, app: &App)
where
    B: Backend,
{
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
    let body_left = draw_body(app.state().diff().unwrap().diff_one());
    rect.render_widget(body_left, body_chunks[0]);

    // Right Diff
    let body_right = draw_body(app.state().diff().unwrap().diff_two());
    rect.render_widget(body_right, body_chunks[1]);

    // Footer Layout (Logs & Help)
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(chunks[2]);

    let logs = draw_console(app.state());
    rect.render_widget(logs, footer_chunks[0]);

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
