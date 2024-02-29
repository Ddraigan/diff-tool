use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::model::Model;
use crate::update::Message;

pub fn handle_event(_: &Model) -> Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
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
        KeyCode::Char('k') => Some(Message::RowUp),
        KeyCode::Char('j') => Some(Message::RowDown),
        KeyCode::Char('g') => Some(Message::Reset),
        KeyCode::Char('G') => Some(Message::BottomRow),
        KeyCode::Char('q') => Some(Message::Quit),
        _ => None,
    }
}
