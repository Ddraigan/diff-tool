use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::App;

pub fn draw<B>(rect: &mut Frame<B>, _app: &App)
where
    B: Backend,
{
    let size = rect.size();
    check_size(&size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3)].as_ref())
        .split(size);

    let title = draw_title();
    rect.render_widget(title, chunks[0]);
}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("Quick Title To Test")
        .style(Style::default().fg(tui::style::Color::LightCyan))
        .alignment(tui::layout::Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(tui::style::Color::White))
                .border_type(tui::widgets::BorderType::Plain),
        )
}

fn check_size(rect: &Rect) {
    if rect.width < 52 {
        panic!("Require width >= 52, (got {})", rect.width);
    }
    if rect.height < 28 {
        panic!("Require height >= 28, (got {})", rect.height);
    }
}
