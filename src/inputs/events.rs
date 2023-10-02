use std::{
    sync::mpsc::{channel, Receiver, RecvError, Sender},
    thread,
    time::Duration,
};

use super::key::Key;

pub enum InputEvent {
    /// An input event occurred.
    Input(Key),
    /// An tick event occurred.
    Tick,
}

pub struct Events {
    rx: Receiver<InputEvent>,
    _tx: Sender<InputEvent>,
}

impl Events {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = channel();

        let event_tx = tx.clone();
        thread::spawn(move || loop {
            if crossterm::event::poll(tick_rate).unwrap() {
                if let crossterm::event::Event::Key(key) = crossterm::event::read().unwrap() {
                    let key = Key::from(key);
                    event_tx.send(InputEvent::Input(key)).unwrap();
                }
            }
            event_tx.send(InputEvent::Tick).unwrap();
        });

        Events { rx, _tx: tx }
    }

    pub fn next(&self) -> Result<InputEvent, RecvError> {
        self.rx.recv()
    }
}
