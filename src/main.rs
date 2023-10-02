mod app;

use app::{ui, App};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::{cell::RefCell, io, rc::Rc};
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let app = Rc::new(RefCell::new(App::new()));
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    loop {
        let app = app.borrow();

        terminal.draw(|rect| ui::draw(rect, &app))?;
    }

    terminal.clear()?;
    terminal.hide_cursor()?;
    disable_raw_mode()?;

    Ok(())
}
