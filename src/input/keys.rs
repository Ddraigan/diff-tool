#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Key {
    /// Both Enter (or Return) and numpad Enter
    Enter,
    Tab,
    Backspace,
    Esc,
    /// Left arrow
    Left,
    /// Right arrow
    Right,
    /// Up arrow
    Up,
    /// Down arrow
    Down,
    Ins,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    F0,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Char(char),
    Ctrl(char),
    Alt(char),
    Unknown,
}

impl Key {
    pub fn from_f(n: u8) -> Key {
        match n {
            0 => Key::F0,
            1 => Key::F1,
            2 => Key::F2,
            3 => Key::F3,
            4 => Key::F4,
            5 => Key::F5,
            6 => Key::F6,
            7 => Key::F7,
            8 => Key::F8,
            9 => Key::F9,
            10 => Key::F10,
            11 => Key::F11,
            12 => Key::F12,
            _ => {
                log::warn!("unknown function key: F{}", n);
                Key::Unknown
            }
        }
    }
}
