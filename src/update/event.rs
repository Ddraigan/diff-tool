use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

use crate::{model::Model, update::message::Message};

pub fn handle_event(model: &Model) -> Result<Option<Message>> {
    if event::poll(*model.tick_rate())? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                return Ok(handle_key(key));
            }
        }
    }
    Ok(None)
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('k') => Some(Message::PrevRow),
        KeyCode::Char('j') => Some(Message::NextRow),
        KeyCode::Char('g') => Some(Message::FirstRow),
        KeyCode::Char('G') => Some(Message::LastRow),
        KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Some(Message::Quit),
        KeyCode::Esc => Some(Message::Quit),
        _ => None,
    }
}
