use std::{io, panic};

use anyhow::Result;
use crossterm::{
    event::DisableMouseCapture,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::widgets::{Row, TableState};

use crate::{app::App, inputs::events::EventHandler, ui::render};

pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>;

pub struct Tui {
    /// Interface to the terminal
    terminal: CrosstermTerminal,
    /// Terminal event handler
    events: EventHandler,
}

impl Tui {
    pub fn new(terminal: CrosstermTerminal, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    /// Initialise the terminal interface
    pub fn enter(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stdout(), EnterAlternateScreen, DisableMouseCapture)?;

        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("Failed to reset terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    /// Draw the terminal interface by rendering the widgets
    pub fn draw<'a>(
        &mut self,
        app: &mut App,
        body_left_state: &mut TableState,
        body_right_state: &mut TableState,
        diff_one_rows: &Vec<Row<'a>>,
        diff_two_rows: &Vec<Row<'a>>,
    ) -> Result<()> {
        self.terminal.draw(|rect| {
            render(
                rect,
                app,
                body_left_state,
                body_right_state,
                diff_one_rows,
                diff_two_rows,
            )
        })?;
        Ok(())
    }

    /// Resets the terminal interface.
    ///
    /// This function is also used for the panic hook to revert
    /// the terminal properties if unexpected errors occur.
    fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;
        Ok(())
    }

    /// Exits the terminal interface.
    ///
    /// It disables the raw mode and reverts back the terminal properties.
    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
