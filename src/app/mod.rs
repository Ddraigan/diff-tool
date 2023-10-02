pub mod state;
pub mod ui;

use self::state::AppState;

pub struct App {
    state: AppState,
}

impl App {
    pub fn new() -> Self {
        let state = AppState::default();

        Self { state }
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }
}
