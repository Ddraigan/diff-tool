pub mod body;
pub mod footer;
pub mod header;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::model::Model;

use self::{body::draw_diff_table, header::draw_title};

/// Renders all the components
pub fn view(model: &mut Model, f: &mut Frame) {
    // Term size
    let size = f.size();
    check_size(&size);

    // Vertical Layout
    let area = Layout::default()
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
    f.render_widget(title, area[0]);

    // Body Layout (Left Diff & Right Diff)
    let body_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area[1]);

    // Left Diff
    draw_diff_table(model, f, body_area[0], "Original", false);

    // Right Diff
    draw_diff_table(model, f, body_area[1], "New", true);

    // draw_footer(chunks, f, model)
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

// fn draw_footer(chunks: std::rc::Rc<[Rect]>, f: &mut Frame, model: &Model) {
//     // Footer Layout (Console & Help)
//     let footer_chunks = Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
//         .split(chunks[2]);
//
//     // Console Section
//     let console = draw_console(model);
//     f.render_widget(console, footer_chunks[0]);
//
//     // Help Menu
//     let help_menu = draw_help(model);
//     f.render_widget(help_menu, footer_chunks[1]);
// }
