use std::cell::RefCell;

use ratatui::widgets::TableState;

use crate::git::{Diff, DiffLine};

#[derive(Debug)]
pub struct Model {
    running_state: RunningState,
    diff: Diff,
    state: State,
    // Model could do with a colours / styling section that can load a config for theming
}

#[derive(Debug)]
pub struct State {
    old_diff: RefCell<TableState>,
    current_diff: RefCell<TableState>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            old_diff: RefCell::from(TableState::default().with_selected(0)),
            current_diff: RefCell::from(TableState::default().with_selected(0)),
        }
    }
}

impl Model {
    pub fn new(diff_string: &str) -> Self {
        let running_state = RunningState::default();
        let diff = Diff::parse_diff(diff_string);
        let state = State::default();

        Self {
            running_state,
            diff,
            state,
        }
    }

    pub fn old_diff_state(&self) -> &RefCell<TableState> {
        &self.state.old_diff
    }

    pub fn current_diff_state(&self) -> &RefCell<TableState> {
        &self.state.current_diff
    }

    pub fn old_diff(&self) -> &[DiffLine] {
        self.diff.old_diff()
    }

    pub fn current_diff(&self) -> &[DiffLine] {
        self.diff.current_diff()
    }

    pub fn diff(&self) -> Option<&Diff> {
        if self.old_diff().len() & self.current_diff().len() != 0 {
            return Some(&self.diff);
        }
        None
    }

    pub fn running_state(&self) -> &RunningState {
        &self.running_state
    }

    pub fn set_running(&mut self) {
        self.running_state = RunningState::Running
    }

    pub fn set_done(&mut self) {
        self.running_state = RunningState::Done
    }

    fn longest_diff_len(&self) -> usize {
        let old_diff = self.diff.old_diff().len();
        let current_diff = self.diff.current_diff().len();

        std::cmp::max(old_diff, current_diff) - 1
    }

    pub fn bottom_row(&self) {
        let last_row = self.longest_diff_len();
        self.state.old_diff.borrow_mut().select(Some(last_row));
        self.state.current_diff.borrow_mut().select(Some(last_row));
    }

    pub fn next_row(&self) {
        let i = match self.state.old_diff.borrow().selected() {
            Some(i) => {
                if i >= self.diff.old_diff().len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        let j = match self.state.current_diff.borrow().selected() {
            Some(j) => {
                if j >= self.diff.current_diff().len() - 1 {
                    0
                } else {
                    j + 1
                }
            }
            None => 0,
        };

        self.state.old_diff.borrow_mut().select(Some(i));
        self.state.current_diff.borrow_mut().select(Some(j));
    }

    pub fn previous_row(&self) {
        let i = match self.state.old_diff.borrow().selected() {
            Some(i) => {
                if i == 0 {
                    self.diff.old_diff().len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        let j = match self.state.current_diff.borrow().selected() {
            Some(j) => {
                if j == 0 {
                    self.diff.current_diff().len() - 1
                } else {
                    j - 1
                }
            }
            None => 0,
        };

        self.state.old_diff.borrow_mut().select(Some(i));
        self.state.current_diff.borrow_mut().select(Some(j));
    }

    pub fn reset_row_state(&self) {
        self.state.old_diff.borrow_mut().select(Some(0));
        self.state.current_diff.borrow_mut().select(Some(0));
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}
