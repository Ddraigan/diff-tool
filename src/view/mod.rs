pub mod ui;

use ratatui::{widgets::Paragraph, Frame};

use crate::model::Model;

pub fn view(model: &mut Model, f: &mut Frame) {
    f.render_widget(
        Paragraph::new(format!("Counter: {}", model.line_count())),
        f.size(),
    );
}
