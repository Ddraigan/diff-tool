pub mod body;
pub mod footer;
pub mod header;

use ratatui::{
    layout::{Constraint, Layout, Rect},
    Frame,
};

use crate::app::App;

use self::{body::render_body, footer::render_footer, header::render_header};

/// Renders all the components
pub fn view(model: &mut App, f: &mut Frame) {
    // Term size
    let size = f.size();
    check_size(&size);

    let [header, body, footer] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(10),
        Constraint::Max(10),
    ])
    .areas(size);

    render_header(f, header);

    render_body(model, f, body);

    render_footer(model, footer, f);
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
