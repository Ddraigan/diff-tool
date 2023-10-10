use ratatui::widgets::{Row, TableState};

use super::{
    actions::{Action, Actions},
    state::AppState,
};
use crate::{git::git::Diff, inputs::key::Key};

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

/// The main application, containing the state
#[derive(Clone)]
pub struct App {
    /// State
    state: AppState,
    /// Contextual Actions
    actions: Actions,
}

impl App {
    pub fn new() -> Self {
        let state = AppState::Init;
        let actions = vec![Action::Quit, Action::Enter, Action::Up, Action::Down].into();

        Self { state, actions }
    }

    pub fn set_diff(&mut self, diff: Diff) {
        self.state = AppState::initialized(diff)
    }

    /// Handle a user action
    pub fn do_action(
        &mut self,
        key: Key,
        state_one: &mut TableState,
        state_two: &mut TableState,
        diff_one_rows: &Vec<Row>,
        diff_two_rows: &Vec<Row>,
    ) -> AppReturn {
        if let Some(action) = self.actions.find(key) {
            self.state.send_to_console(format!("Run action: {action}"));
            match action {
                Action::Quit => AppReturn::Exit,
                Action::Enter => AppReturn::Continue,
                Action::Up => {
                    previous_row(state_one, diff_one_rows);
                    previous_row(state_two, diff_two_rows);
                    AppReturn::Continue
                }
                Action::Down => {
                    next_row(state_one, diff_one_rows);
                    next_row(state_two, diff_two_rows);
                    AppReturn::Continue
                }
            }
        } else {
            self.state
                .send_to_console(format!("No action associated to {}", key));
            AppReturn::Continue
        }
    }

    /// Update the app or dispatch events on tick
    pub fn update_on_tick(&mut self) -> AppReturn {
        // Increments counter
        self.state.incr_tick();
        AppReturn::Continue
    }

    pub fn actions(&self) -> &Actions {
        &self.actions
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }
}

fn next_row(state: &mut TableState, rows: &Vec<Row>) {
    let i = match state.selected() {
        Some(i) => {
            if i >= rows.len() - 1 {
                0
            } else {
                i + 1
            }
        }
        None => 0,
    };

    state.select(Some(i));
}

fn previous_row(state: &mut TableState, rows: &Vec<Row>) {
    let i = match state.selected() {
        Some(i) => {
            if i == 0 {
                rows.len() - 1
            } else {
                i - 1
            }
        }
        None => 0,
    };

    state.select(Some(i));
}
