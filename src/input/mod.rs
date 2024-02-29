pub mod event;

use std::fmt::{self, Display};

use crate::model::Model;

pub enum Message {
    RowUp,
    RowDown,
    BottomRow,
    Reset,
    Nothing,
    Quit,
}

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::RowUp => model.previous_row(),
        Message::RowDown => model.next_row(),
        Message::BottomRow => model.bottom_row(),
        Message::Reset => model.reset_row_state(),
        Message::Nothing => {}
        Message::Quit => {
            // Handle some exit stuff
            model.set_done()
        }
    }

    None
}

impl Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Message::RowUp => "Move One Row Up",
            Message::RowDown => "Move One Row Down",
            Message::Reset => "Go To Top Row",
            Message::BottomRow => "Go To Bottom Row",
            Message::Nothing => "",
            Message::Quit => "Quit",
        };
        write!(f, "{}", str)
    }
}
