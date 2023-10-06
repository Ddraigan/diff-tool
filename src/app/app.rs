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

    pub fn diff(self, diff: Diff) -> Self {
        let state = AppState::initialized(diff);

        Self {
            state,
            actions: self.actions,
        }
    }

    /// Handle a user action
    pub fn do_action(&mut self, key: Key) -> AppReturn {
        if let Some(action) = self.actions.find(key) {
            self.state
                .send_to_console(format!("Run action [{:?}]", action));
            match action {
                Action::Quit => AppReturn::Exit,
                Action::Enter => AppReturn::Continue,
                Action::Up => todo!(),
                Action::Down => todo!(),
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
