use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use super::{actions::Actions, app::App};

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
    let body_left = draw_body();
    rect.render_widget(body_left, body_chunks[0]);

    // Right Diff
    let body_right = draw_body();
    rect.render_widget(body_right, body_chunks[1]);

    // Footer Layout (Logs & Help)
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(chunks[2]);

    /* let logs = draw_logs();
    rect.render_widget(logs, footer_chunks[0]); */

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

/// Draws the title component
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

/// Draws the body components
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

/// Draws the log component
/* fn draw_logs<'a>() -> TuiLoggerWidget<'a> {
    TuiLoggerWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Gray))
        .style_info(Style::default().fg(Color::Cyan))
        .block(
            Block::default()
                .title("Logs")
                .border_style(
                    Style::default()
                        .fg(tui::style::Color::White)
                        .bg(tui::style::Color::Black),
                )
                .borders(Borders::ALL),
        )
        .style(
            Style::default()
                .fg(tui::style::Color::White)
                .bg(tui::style::Color::Black),
        )
} */

/// Draws the help menu component
fn draw_help(actions: &Actions) -> Table {
    let key_style = Style::default().fg(tui::style::Color::LightCyan);
    let help_style = Style::default().fg(tui::style::Color::Gray);

    let mut rows = vec![];
    for action in actions.actions().iter() {
        let mut first = true;
        for key in action.keys() {
            let help = if first {
                first = false;
                action.to_string()
            } else {
                String::from("")
            };
            let row = Row::new(vec![
                Cell::from(Span::styled(key.to_string(), key_style)),
                Cell::from(Span::styled(help, help_style)),
            ]);
            rows.push(row);
        }
    }

    Table::new(rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(tui::widgets::BorderType::Plain)
                .title("Help"),
        )
        .widths(&[Constraint::Length(11), Constraint::Min(20)])
        .column_spacing(1)
}
