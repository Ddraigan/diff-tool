use crate::model::Model;

pub enum Message {
    IncrementLine,
    DecrementLine,
    Reset,
    Quit,
}

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::IncrementLine => {
            model.line_count_incr();
            if model.line_count() > 50 {
                return Some(Message::Reset);
            }
        }
        Message::DecrementLine => {
            model.line_count_decr();
            if model.line_count() < -50 {
                return Some(Message::Reset);
            }
        }
        Message::Reset => model.line_count_reset(),
        Message::Quit => model.set_done(),
    }

    None
}
