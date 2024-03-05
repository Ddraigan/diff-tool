use std::fmt::{self, Display};

use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd)]
pub enum Message {
    PrevRow,
    NextRow,
    LastRow,
    FirstRow,
    Quit,
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
