#[derive(Debug, Default)]
pub struct Model {
    line_count: i32,
    running_state: RunningState,
}

impl Model {
    pub fn line_count(&self) -> i32 {
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
