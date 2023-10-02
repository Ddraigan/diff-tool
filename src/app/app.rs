use super::{
    actions::{Action, Actions},
    state::AppState,
};
use crate::inputs::key::Key;

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

/// The main application, containing the state
pub struct App {
    /// State
    state: AppState,
    /// Contextual Actions
    actions: Actions,
}

impl App {
    pub fn new() -> Self {
        let state = AppState::default();
        let actions = vec![Action::Quit].into();

        Self { state, actions }
    }

    pub fn do_action(&mut self, key: Key) -> AppReturn {
        if let Some(action) = self.actions.find(key) {
            println!("Run action [{:?}]", action);
            match action {
                Action::Quit => AppReturn::Exit,
            }
        } else {
            println!("No action associated to {}", key);
            AppReturn::Continue
        }
    }

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
