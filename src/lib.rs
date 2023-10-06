pub mod app;
pub mod cli;
pub mod git;
pub mod inputs;

use anyhow::Result;
use app::{
    app::{App, AppReturn},
    ui::ui::{self, parse_diff_rows},
};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use inputs::events::{Events, InputEvent};
use std::{cell::RefCell, io, rc::Rc, time::Duration};
use tui::{backend::CrosstermBackend, widgets::TableState, Terminal};

pub fn start_tui(app: Rc<RefCell<App>>) -> Result<()> {
    enable_raw_mode()?;
    let stdout = io::stdout();
    // execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let tick_rate = Duration::from_millis(200);
    let events = Events::new(tick_rate);

    let mut table_state = TableState::default();
    table_state.select(Some(0));

    let mut app = app.borrow_mut();
    // todo Want this whole app clone gone
    let app_clone = app.clone();
    let diff_one_rows = parse_diff_rows(app_clone.state().diff().unwrap().diff_one());
    let diff_two_rows = parse_diff_rows(app_clone.state().diff().unwrap().diff_two());

    loop {
        // Render ui
        terminal
            .draw(|rect| ui::draw(rect, &app, &mut table_state, &diff_one_rows, &diff_two_rows))?;

        // Handle inputs
        let input_result = match events.next()? {
            InputEvent::Input(key) => app.do_action(key),
            InputEvent::Tick => app.update_on_tick(),
        };

        // Check if we should exit
        if input_result == AppReturn::Exit {
            break;
        }
    }

    // Restore the terminal and close application
    terminal.clear()?;
    terminal.hide_cursor()?;
    disable_raw_mode()?;

    Ok(())
}
