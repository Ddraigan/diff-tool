use std::fmt::{self, Display};

use super::keys::Key;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Message {
    PrevRow,
    NextRow,
    LastRow,
    FirstRow,
    Quit,
}

pub fn handle_key(key: Key) -> Option<Message> {
    match key {
        Key::Char('k') => Some(Message::PrevRow),
        Key::Char('j') => Some(Message::NextRow),
        Key::Char('g') => Some(Message::FirstRow),
        Key::Char('G') => Some(Message::LastRow),
        Key::Char('q') => Some(Message::Quit),
        Key::Ctrl('c') => Some(Message::Quit),
        Key::Esc => Some(Message::Quit),
        _ => None,
    }
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
