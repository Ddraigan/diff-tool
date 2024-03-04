pub mod message;

use crate::model::Model;

use self::message::Message;

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::RowUp => {
            model.previous_row();
        }
        Message::RowDown => {
            model.next_row();
        }
        Message::BottomRow => {
            model.bottom_row();
        }
        Message::Reset => {
            model.reset_row_state();
        }
        Message::Quit => {
            // Handle some exit stuff
            model.quit();
        }
    }

    None
}
