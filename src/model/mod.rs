use ratatui::widgets::TableState;

use crate::git::{get_diff, get_raw_diff, Diff};

#[derive(Debug)]
pub struct Model {
    line_count: u32,
    running_state: RunningState,
    diff: Diff,
    state: State,
}

#[derive(Debug)]
pub struct State {
    old_diff: TableState,
    current_diff: TableState,
}

impl State {
    fn new() -> Self {
        let mut old_diff = TableState::default();
        let mut current_diff = TableState::default();

        old_diff.select(Some(0));
        current_diff.select(Some(0));

        Self {
            old_diff,
            current_diff,
        }
    }
}

impl Model {
    pub fn new(diff: Diff) -> Self {
        let line_count = u32::default();
        let running_state = RunningState::default();
        let diff = diff;
        let state = State::new();

        Self {
            line_count,
            running_state,
            diff,
            state,
        }
    }

    pub fn max_content(&self) -> bool {
        if self.line_count != self.diff.largest_line_number_len() {
            return false;
        }
        true
    }

    pub fn old_diff_state_mut(&mut self) -> &mut TableState {
        &mut self.state.old_diff
    }

    pub fn current_diff_state_mut(&mut self) -> &mut TableState {
        &mut self.state.current_diff
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
