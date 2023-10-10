use crate::git::Diff;

#[derive(Clone)]
pub struct AppState {
    counter_tick: u64,
    console: Vec<String>,
    diff: Diff,
}

impl AppState {
    pub fn new(diff: Diff) -> Self {
        let counter_tick = 0;

        Self {
            counter_tick,
            console: vec![],
            diff,
        }
    }

    pub fn incr_tick(&mut self) {
        self.counter_tick += 1
    }

    pub fn count_tick(&self) -> Option<u64> {
        Some(self.counter_tick)
    }

    pub fn console(&self) -> Option<&Vec<String>> {
        Some(&self.console)
    }

    pub fn diff(&self) -> Option<&Diff> {
        Some(&self.diff)
    }

    pub fn send_to_console(&mut self, content: String) {
        self.console.push(content)
    }
}
