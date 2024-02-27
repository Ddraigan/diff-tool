use crate::git::Diff;

#[derive(Debug, Default)]
pub struct Model {
    line_count: u32,
    running_state: RunningState,
    diff: Diff,
}

impl Model {
    pub fn max_content(&self) -> bool {
        if self.line_count != self.diff.largest_line_number_len() {
            return false;
        }
        true
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
