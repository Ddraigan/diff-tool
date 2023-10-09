pub mod app;
pub mod cli;
pub mod git;
pub mod inputs;

use anyhow::{Ok, Result};
use app::{
    app::{App, AppReturn},
    ui::{self, body::parse_diff_rows},
};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use inputs::events::{Events, InputEvent};
use std::{
    cell::RefCell,
    io::{self, Stdout},
    rc::Rc,
    time::Duration,
};
use tui::{backend::CrosstermBackend, widgets::TableState, Terminal};

pub fn start_tui(app: Rc<RefCell<App>>) -> Result<()> {
    let mut terminal = term_setup()?;

    let tick_rate = Duration::from_millis(200);
    let events = Events::new(tick_rate);

    let mut diff_one_state = TableState::default();
    let mut diff_two_state = TableState::default();

    diff_one_state.select(Some(0));
    diff_two_state.select(Some(0));

    let mut app = app.borrow_mut();
    // todo Want this whole app clone gone
    let app_clone = app.clone();
    let diff_one_rows = parse_diff_rows(app_clone.state().diff().unwrap().diff_one());
    let diff_two_rows = parse_diff_rows(app_clone.state().diff().unwrap().diff_two());

    loop {
        // Render ui
        terminal.draw(|rect| {
            ui::ui::draw(
                rect,
                &app,
                &mut diff_one_state,
                &mut diff_two_state,
                &diff_one_rows,
                &diff_two_rows,
            )
        })?;

        // Handle inputs
        let input_result = match events.next()? {
            InputEvent::Input(key) => app.do_action(
                key,
                &mut diff_one_state,
                &mut diff_two_state,
                &diff_one_rows,
                &diff_two_rows,
            ),
            InputEvent::Tick => app.update_on_tick(),
        };

        // Check if we should exit
        if input_result == AppReturn::Exit {
            break;
        }
    }

    term_restore(&mut terminal)?;

    Ok(())
}

fn term_setup() -> Result<Terminal<CrosstermBackend<Stdout>>, anyhow::Error> {
    enable_raw_mode()?;
    let stdout = io::stdout();
    // execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;
    Ok(terminal)
}

fn term_restore(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    // Restore the terminal and close application
    terminal.clear()?;
    terminal.hide_cursor()?;
    disable_raw_mode()?;
    Ok(())
}
