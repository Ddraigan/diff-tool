pub mod event;

use crate::model::Model;

pub enum Message {
    Increment,
    Decrement,
    Reset,
    Nothing,
    Quit,
}

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::Increment => {
            model.line_count_incr();
            if model.max_content() {
                return Some(Message::Nothing);
            }
        }
        Message::Decrement => {
            model.line_count_decr();
            if model.line_count() == 0 {
                return Some(Message::Nothing);
            }
        }
        Message::Reset => model.line_count_reset(),
        Message::Nothing => model.line_count_nothing(),
        Message::Quit => {
            // Handle some exit stuff
            model.set_done()
        }
    }

    None
}
