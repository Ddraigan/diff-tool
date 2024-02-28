pub mod body;
pub mod footer;
pub mod header;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Row,
    Frame,
};

use crate::model::Model;

use self::{body::draw_diff_table, header::draw_title};

/// Renders all the components
pub fn view<B>(
    f: &mut Frame,
    model: &mut Model,
    diff_one_rows: &Vec<Row>,
    diff_two_rows: &Vec<Row>,
) {
    // Term size
    let size = f.size();
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
    f.render_widget(title, chunks[0]);

    // Body Layout (Left Diff & Right Diff)
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    // Left Diff
    let body_left = draw_diff_table(&model, &diff_one_rows, "Original", false);
    let old_diff_state = model.old_diff_state_mut();
    f.render_stateful_widget(body_left, body_chunks[0], old_diff_state);

    // Right Diff
    let body_right = draw_diff_table(&model, &diff_two_rows, "New", true);
    let current_diff_state = model.current_diff_state_mut();
    f.render_stateful_widget(body_right, body_chunks[1], current_diff_state);

    draw_footer(chunks, f, model)
}

/// Checks terminal size is large enough
fn check_size(f: &Rect) {
    if f.width < 52 {
        panic!("Require width >= 52, (got {})", f.width);
    }
    if f.height < 28 {
        panic!("Require height >= 28, (got {})", f.height);
    }
}

fn draw_footer(chunks: std::rc::Rc<[Rect]>, f: &mut Frame, model: &Model) {
    // Footer Layout (Console & Help)
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(chunks[2]);

    // Console Section
    let console = draw_console(model);
    f.render_widget(console, footer_chunks[0]);

    // Help Menu
    let help_menu = draw_help(model);
    f.render_widget(help_menu, footer_chunks[1]);
}
