pub mod state;

use std::{cell::RefCell, time::Duration};

use ratatui::widgets::TableState;

use crate::services::git::Diff;

use self::state::{RunningState, State};

#[derive(Debug)]
pub struct Model {
    running_state: RunningState,
    diff: Diff,
    state: State,
    /// Default value is 250 millis
    tick_rate: Duration,
    // TODO: Model could do with a colours / styling section that can load a config for theming
}

impl Default for Model {
    fn default() -> Self {
        Self {
            running_state: Default::default(),
            diff: Default::default(),
            state: Default::default(),
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl Model {
    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn tick_rate(&self) -> &Duration {
        &self.tick_rate
    }

    pub fn set_tick_rate(&mut self, tick_rate: Duration) {
        self.tick_rate = tick_rate
    }

    pub fn old_diff_state(&self) -> &RefCell<TableState> {
        &self.state.old_diff()
    }

    pub fn current_diff_state(&self) -> &RefCell<TableState> {
        &self.state.current_diff()
    }

    pub fn diff(&self) -> Option<&Diff> {
        if self.diff.old_diff().len() & self.diff.current_diff().len() != 0 {
            return Some(&self.diff);
        }
        None
    }

    pub fn set_diff(&mut self, diff_string: &str) {
        self.diff = Diff::parse_diff(diff_string)
    }

    pub fn running_state(&self) -> &RunningState {
        &self.running_state
    }

    pub fn quit(&mut self) {
        self.running_state = RunningState::Done
    }

    pub fn go_to_last_row(&self) {
        let last_row = self.diff.longest_diff_len();
        self.state.old_diff().borrow_mut().select(Some(last_row));
        self.state
            .current_diff()
            .borrow_mut()
            .select(Some(last_row));
    }

    pub fn next_row(&self) {
        let old_diff_row_index = match self.state.old_diff().borrow().selected() {
            Some(i) => {
                if i >= self.diff.old_diff().len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        let current_diff_row_index = match self.state.current_diff().borrow().selected() {
            Some(j) => {
                if j >= self.diff.current_diff().len() - 1 {
                    0
                } else {
                    j + 1
                }
            }
            None => 0,
        };

        self.state
            .old_diff()
            .borrow_mut()
            .select(Some(old_diff_row_index));
        self.state
            .current_diff()
            .borrow_mut()
            .select(Some(current_diff_row_index));
    }

    pub fn previous_row(&self) {
        let old_diff_row_index = match self.state.old_diff().borrow().selected() {
            Some(i) => {
                if i == 0 {
                    self.diff.old_diff().len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        let current_diff_row_index = match self.state.current_diff().borrow().selected() {
            Some(j) => {
                if j == 0 {
                    self.diff.current_diff().len() - 1
                } else {
                    j - 1
                }
            }
            None => 0,
        };

        self.state
            .old_diff()
            .borrow_mut()
            .select(Some(old_diff_row_index));
        self.state
            .current_diff()
            .borrow_mut()
            .select(Some(current_diff_row_index));
    }
}
