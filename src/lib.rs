pub mod app;
pub mod cli;
pub mod git;
pub mod inputs;
pub mod ui;

use anyhow::Result;
use app::{App, AppReturn};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use inputs::events::{Events, InputEvent};
use ratatui::{backend::CrosstermBackend, widgets::TableState, Terminal};
use std::{
    io::{self, Stdout},
    time::Duration,
};
use ui::{body::parse_diff_rows, draw};

pub fn start_tui(app: App) -> Result<()> {
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

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, mut app: App) -> Result<()> {
    let tick_rate = Duration::from_millis(200);
    let events = Events::new(tick_rate);

    let mut diff_one_state = create_diff_state();
    let mut diff_two_state = create_diff_state();

    // todo Want this whole app clone gone
    // let app_clone = app.clone();
    let diff_one = app.state().diff().unwrap().diff_one().clone();
    let diff_two = app.state().diff().unwrap().diff_two().clone();
    let diff_one_rows = parse_diff_rows(&diff_one);
    let diff_two_rows = parse_diff_rows(&diff_two);

    loop {
        // Render ui
        terminal.draw(|rect| {
            draw(
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
