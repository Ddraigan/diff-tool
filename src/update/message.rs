use std::fmt::{self, Display};

use serde::Deserialize;

use crate::services::config::AppConfig;

use super::keys::Key;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum Message {
    PrevRow,
    NextRow,
    LastRow,
    FirstRow,
    Quit,
}

pub fn handle_key(key: Key, config: &AppConfig) -> Option<Message> {
    let key_string = key.to_string();
    let key = config.keymap.get(&key_string);
    key.cloned()
}

/// Display a user friendly short description of action
impl Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Message::PrevRow => "Scroll up one row",
            Message::NextRow => "Scroll down one row",
            Message::LastRow => "Jump to bottom row",
            Message::FirstRow => "Jump to top row",
            Message::Quit => "Quit application",
        };
        write!(f, "{}", str)
    }
}
