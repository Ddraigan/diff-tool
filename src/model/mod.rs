use std::cell::RefCell;

use ratatui::widgets::TableState;

use crate::git::{Diff, DiffLine};

#[derive(Debug)]
pub struct Model {
    line_count: u32,
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
        let line_count = u32::default();
        let running_state = RunningState::default();
        let diff = Diff::parse_diff(diff_string);
        let state = State::default();

        Self {
            line_count,
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

    pub fn max_content(&self) -> bool {
        if self.line_count != self.diff.largest_line_number_len().into() {
            return false;
        }
        true
    }

    pub fn old_diff(&self) -> &[DiffLine] {
        self.diff.old_diff()
    }

    pub fn current_diff(&self) -> &[DiffLine] {
        self.diff.current_diff()
    }

    pub fn diff(&self) -> &Diff {
        &self.diff
    }

    pub fn line_count(&self) -> u32 {
        self.line_count
    }

    pub fn line_count_incr(&mut self) {
        self.line_count += 1
    }

    pub fn line_count_decr(&mut self) {
        self.line_count -= 1
    }

    pub fn line_count_reset(&mut self) {
        self.line_count = 0
    }

    pub fn line_count_nothing(&mut self) {
        self.line_count = self.line_count
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
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}
