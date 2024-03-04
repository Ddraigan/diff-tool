use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub enum Message {
    RowUp,
    RowDown,
    BottomRow,
    Reset,
    Quit,
}
