use crate::model::Model;
use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Message {
    PrevRow,
    NextRow,
    LastRow,
    FirstRow,
    Quit,
}

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::PrevRow => {
            model.previous_row();
        }
        Message::NextRow => {
            model.next_row();
        }
        Message::LastRow => {
            model.go_to_last_row();
        }
        Message::FirstRow => {
            model.go_to_first_row();
        }
        Message::Quit => {
            // Handle some exit stuff
            model.quit();
        }
    }

    None
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
