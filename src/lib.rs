pub mod app;
pub mod cli;
pub mod git;
pub mod inputs;

use anyhow::Result;
use app::{
    app::{App, AppReturn},
    ui::{self, body::parse_diff_rows},
};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use inputs::events::{Events, InputEvent};
use ratatui::{backend::CrosstermBackend, widgets::TableState, Terminal};
use std::{
    cell::RefCell,
    io::{self, Stdout},
    rc::Rc,
    time::Duration,
};

pub fn start_tui(app: Rc<RefCell<App>>) -> Result<()> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    term_startup(&mut terminal)?;
    let status = run(&mut terminal, app);
    term_shutdown(&mut terminal)?;
    status?;

    Ok(())
}

fn term_startup(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    enable_raw_mode()?;
    crossterm::execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen)?;
    terminal.clear()?;
    terminal.hide_cursor()?;
    Ok(())
}

fn term_shutdown(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    // Restore the terminal and close application
    crossterm::execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;
    terminal.clear()?;
    terminal.hide_cursor()?;
    disable_raw_mode()?;
    Ok(())
}

fn create_diff_state() -> TableState {
    let mut diff_state = TableState::default();
    diff_state.select(Some(0));

    diff_state
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: Rc<RefCell<App>>) -> Result<()> {
    let tick_rate = Duration::from_millis(200);
    let events = Events::new(tick_rate);

    let mut diff_one_state = create_diff_state();
    let mut diff_two_state = create_diff_state();

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

    Ok(())
}
