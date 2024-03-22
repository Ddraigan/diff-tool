pub mod state;

use anyhow::Result;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::widgets::ListState;
use std::{
    cell::RefCell,
    cmp,
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::{
    services::{config::Config, git::Diff},
    update::{keys::Key, message::Message},
};

use self::state::{DiffState, RunningState};

#[derive(Debug)]
pub struct App {
    running_state: RunningState,
    // TODO: Model could do with a colours / styling section that can load a config for theming
    config: Config,
    diff: Diff,
    diff_state: DiffState,
    logs: Arc<Mutex<Vec<String>>>,
    console_state: RefCell<ListState>,
    /// Default value is 250 millis
    tick_rate: Duration,
}

impl App {
    pub fn new(logs: Arc<Mutex<Vec<String>>>) -> Self {
        let mut new = Self {
            running_state: Default::default(),
            // TODO: This should be handled with a default config probably
            config: Config::new().expect("Unable to get Configuration"),
            diff: Default::default(),
            diff_state: Default::default(),
            logs,
            console_state: Default::default(),
            tick_rate: Duration::from_millis(250),
        };

        new.handle_console();

        return new;
    }

    pub fn console_state(&self) -> &RefCell<ListState> {
        &self.console_state
    }

    pub fn handle_console(&mut self) {
        let console_length = self.console().len();
        let console_state_index = cmp::max(1, console_length) - 1;

        self.console_state
            .borrow_mut()
            .select(Some(console_state_index));
    }

    pub fn update(&mut self, msg: Message) -> Option<Message> {
        log::info!("{}", msg);
        self.handle_console();

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
                self.diff_state().reset_row_state();
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

    pub fn console(&self) -> Vec<String> {
        return self.logs.lock().unwrap().clone();
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn diff_state(&self) -> &DiffState {
        &self.diff_state
    }

    pub fn diff(&self) -> Option<&Diff> {
        if self.diff.old_diff().len() != 0 && self.diff.current_diff().len() != 0 {
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
        self.diff_state
            .old_diff()
            .borrow_mut()
            .select(Some(last_row));
        self.diff_state
            .current_diff()
            .borrow_mut()
            .select(Some(last_row));
    }

    fn next_row(&self) {
        let old_diff_row_index = match self.diff_state.old_diff().borrow().selected() {
            Some(i) => {
                if i >= self.diff.old_diff().len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        let current_diff_row_index = match self.diff_state.current_diff().borrow().selected() {
            Some(j) => {
                if j >= self.diff.current_diff().len() - 1 {
                    0
                } else {
                    j + 1
                }
            }
            None => 0,
        };

        self.diff_state
            .old_diff()
            .borrow_mut()
            .select(Some(old_diff_row_index));
        self.diff_state
            .current_diff()
            .borrow_mut()
            .select(Some(current_diff_row_index));
    }

    fn previous_row(&self) {
        let old_diff_row_index = match self.diff_state.old_diff().borrow().selected() {
            Some(i) => {
                if i == 0 {
                    self.diff.old_diff().len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        let current_diff_row_index = match self.diff_state.current_diff().borrow().selected() {
            Some(j) => {
                if j == 0 {
                    self.diff.current_diff().len() - 1
                } else {
                    j - 1
                }
            }
            None => 0,
        };

        self.diff_state
            .old_diff()
            .borrow_mut()
            .select(Some(old_diff_row_index));
        self.diff_state
            .current_diff()
            .borrow_mut()
            .select(Some(current_diff_row_index));
    }
}
