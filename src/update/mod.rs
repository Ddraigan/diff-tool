pub mod event;

use crate::model::Model;

pub enum Message {
    RowUp,
    RowDown,
    TopRow,
    BottomRow,
    Reset,
    Nothing,
    Quit,
}

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::RowUp => model.previous_row(),
        Message::RowDown => model.next_row(),
        Message::TopRow => return Some(Message::Reset),
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
