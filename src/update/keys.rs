use std::fmt::{self, Display, Formatter};

use crossterm::event;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
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
    Shift(char),
    Unknown,
}

impl Key {
    /// Returns the function key corresponding to the given number
    ///
    /// 1 -> F1, etc...
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

impl From<event::KeyEvent> for Key {
    fn from(key_event: event::KeyEvent) -> Self {
        match key_event {
            event::KeyEvent {
                code: event::KeyCode::Esc,
                ..
            } => Key::Esc,
            event::KeyEvent {
                code: event::KeyCode::Backspace,
                ..
            } => Key::Backspace,
            event::KeyEvent {
                code: event::KeyCode::Left,
                ..
            } => Key::Left,
            event::KeyEvent {
                code: event::KeyCode::Right,
                ..
            } => Key::Right,
            event::KeyEvent {
                code: event::KeyCode::Up,
                ..
            } => Key::Up,
            event::KeyEvent {
                code: event::KeyCode::Down,
                ..
            } => Key::Down,
            event::KeyEvent {
                code: event::KeyCode::Home,
                ..
            } => Key::Home,
            event::KeyEvent {
                code: event::KeyCode::End,
                ..
            } => Key::End,
            event::KeyEvent {
                code: event::KeyCode::PageUp,
                ..
            } => Key::PageUp,
            event::KeyEvent {
                code: event::KeyCode::PageDown,
                ..
            } => Key::PageDown,
            event::KeyEvent {
                code: event::KeyCode::Delete,
                ..
            } => Key::Delete,
            event::KeyEvent {
                code: event::KeyCode::Insert,
                ..
            } => Key::Ins,
            event::KeyEvent {
                code: event::KeyCode::F(n),
                ..
            } => Key::from_f(n),
            event::KeyEvent {
                code: event::KeyCode::Enter,
                ..
            } => Key::Enter,
            event::KeyEvent {
                code: event::KeyCode::Tab,
                ..
            } => Key::Tab,

            // First check for char + modifier
            event::KeyEvent {
                code: event::KeyCode::Char(c),
                modifiers: event::KeyModifiers::ALT,
                kind: _,
                state: _,
            } => Key::Alt(c),

            event::KeyEvent {
                code: event::KeyCode::Char(c),
                modifiers: event::KeyModifiers::CONTROL,
                kind: _,
                state: _,
            } => Key::Ctrl(c),

            event::KeyEvent {
                code: event::KeyCode::Char(c),
                modifiers: event::KeyModifiers::SHIFT,
                kind: _,
                state: _,
            } => Key::Shift(c),

            event::KeyEvent {
                code: event::KeyCode::Char(c),
                ..
            } => Key::Char(c),

            _ => Key::Unknown,
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Key::Alt(' ') => write!(f, "alt+space"),
            Key::Ctrl(' ') => write!(f, "ctrl+space"),
            Key::Char(' ') => write!(f, "space"),
            Key::Alt(c) => write!(f, "alt+{}", c),
            Key::Ctrl(c) => write!(f, "ctrl+{}", c),
            Key::Shift(c) => write!(f, "shift+{}", c.to_lowercase()),
            Key::Char(c) => write!(f, "{}", c),
            _ => write!(f, "{:?}", self),
        }
    }
}
