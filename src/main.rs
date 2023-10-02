mod app;
mod inputs;

use app::{
    app::{App, AppReturn},
    ui,
};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use eyre::Result;
use inputs::events::{Events, InputEvent};
use std::{cell::RefCell, io, rc::Rc, time::Duration};
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<()> {
    enable_raw_mode()?;
    let app = Rc::new(RefCell::new(App::new()));
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let tick_rate = Duration::from_millis(200);
    let events = Events::new(tick_rate);

    loop {
        let mut app = app.borrow_mut();

        // Render
        terminal.draw(|rect| ui::draw(rect))?;

        // Handle inputs
        let result = match events.next()? {
            InputEvent::Input(key) => app.do_action(key),
            InputEvent::Tick => app.update_on_tick(),
        };

        // Check if we should exit
        if result == AppReturn::Exit {
            break;
        }
    }

    // Restore the terminal and close application
    terminal.clear()?;
    terminal.hide_cursor()?;
    disable_raw_mode()?;

    Ok(())
}
