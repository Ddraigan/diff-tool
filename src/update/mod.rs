use crate::model::Model;

pub enum Message {
    RowUp,
    RowDown,
    BottomRow,
    Reset,
    Quit,
}

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
            model.set_done();
        }
    }

    None
}
