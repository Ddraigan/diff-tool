pub mod app;
pub mod cli;
pub mod git;
pub mod inputs;

use anyhow::Result;
use app::{
    app::{App, AppReturn},
    ui::ui,
};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use inputs::events::{Events, InputEvent};
use std::{cell::RefCell, io, rc::Rc, time::Duration};
use tui::{backend::CrosstermBackend, Terminal};

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

    loop {
        let mut app = app.borrow_mut();

        // Render ui
        terminal.draw(|rect| ui::draw(rect, &app))?;

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
