use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Draws all the components
pub fn draw<B>(rect: &mut Frame<B>)
where
    B: Backend,
{
    let size = rect.size();
    check_size(&size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(size);

    let title = draw_title();
    rect.render_widget(title, chunks[0]);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(20), Constraint::Length(50)].as_ref())
        .split(chunks[1]);

    let body_left = draw_body();
    rect.render_widget(body_left, body_chunks[0]);

    let body_right = draw_body();
    rect.render_widget(body_right, body_chunks[1]);
}

fn check_size(rect: &Rect) {
    if rect.width < 52 {
        panic!("Require width >= 52, (got {})", rect.width);
    }
    if rect.height < 28 {
        panic!("Require height >= 28, (got {})", rect.height);
    }
}

fn draw_title<'a>() -> Paragraph<'a> {
    Paragraph::new("Git Diff View")
        .style(Style::default().fg(tui::style::Color::LightCyan))
        .alignment(tui::layout::Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(tui::style::Color::White))
                .border_type(tui::widgets::BorderType::Plain),
        )
}

fn draw_body<'a>() -> Paragraph<'a> {
    let text = Spans::from(vec![
        Span::raw("First"),
        Span::styled("line", Style::default().add_modifier(Modifier::ITALIC)),
        Span::raw("."),
    ]);

    let many_text = vec![text; 20];

    Paragraph::new(many_text)
        .style(Style::default().fg(tui::style::Color::LightGreen))
        .alignment(tui::layout::Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(tui::style::Color::White))
                .border_type(tui::widgets::BorderType::Plain),
        )
}
