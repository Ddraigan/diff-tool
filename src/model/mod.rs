pub mod state;

use anyhow::Result;
use crossterm::event::{self, Event, KeyEventKind};
use std::time::Duration;

use crate::{
    services::{config::Config, git::Diff},
    update::{keys::Key, message::Message},
};

use self::state::{RunningState, State};

#[derive(Debug)]
pub struct App {
    running_state: RunningState,
    // TODO: Model could do with a colours / styling section that can load a config for theming
    config: Config,
    diff: Diff,
    state: State,
    /// Default value is 250 millis
    tick_rate: Duration,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running_state: Default::default(),
            // TODO: This should be handled
            config: Config::new().expect("Unable to get Conifguration"),
            diff: Default::default(),
            state: Default::default(),
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl App {
    pub fn update(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::PrevRow => {
                self.previous_row();
            }
            Message::NextRow => {
                self.next_row();
            }
            Message::LastRow => {
                self.go_to_last_row();
            }
            Message::FirstRow => {
                self.state().reset_row_state();
            }
            Message::Quit => {
                // Handle some exit stuff
                self.quit();
            }
        }

        None
    }

    pub fn handle_event(&self) -> Result<Option<Message>> {
        if event::poll(self.tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    // Converts Crossterm::Event::Key into our update::Key
                    return Ok(self.handle_key(key.into()));
                }
            }
        }
        Ok(None)
    }

    fn handle_key(&self, key: Key) -> Option<Message> {
        let key_string = key.to_string();
        let key = self.config.keymap().get(&key_string);
        key.cloned()
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn state(&self) -> &State {
        &self.state
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

    fn quit(&mut self) {
        self.running_state = RunningState::Done
    }

    fn go_to_last_row(&self) {
        let last_row = self.diff.longest_diff_len();
        self.state.old_diff().borrow_mut().select(Some(last_row));
        self.state
            .current_diff()
            .borrow_mut()
            .select(Some(last_row));
    }

    fn next_row(&self) {
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

    fn previous_row(&self) {
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
