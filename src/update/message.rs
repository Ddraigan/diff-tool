use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
