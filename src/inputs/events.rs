use std::{
    sync::mpsc::{channel, Receiver, RecvError, Sender},
    thread,
    time::Duration,
};

use super::key::Key;

pub enum InputEvent {
    /// An input event occurred.
    Input(Key),
    /// A tick event occurred.
    Tick,
}

/// A small event handler that wrap crossterm input and tick event. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: Receiver<InputEvent>,
    // Need to be kept around to prevent disposing the sender side.
    _tx: Sender<InputEvent>,
}

impl Events {
    /// Constructs an new instance of `Events` with the default config.
    /// mpsc setup
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = channel();

        let event_tx = tx.clone();
        thread::spawn(move || loop {
            // poll for tick rate duration, if no event, sent tick event.
            if crossterm::event::poll(tick_rate).unwrap() {
                if let crossterm::event::Event::Key(key) = crossterm::event::read().unwrap() {
                    if key.kind == crossterm::event::KeyEventKind::Press {
                        let key = Key::from(key);
                        event_tx.send(InputEvent::Input(key)).unwrap();
                    }
                }
            }
            match event_tx.send(InputEvent::Tick) {
                Ok(_) => {}
                Err(err) => log::error!("Send error: {err}"),
            };
        });

        Events { rx, _tx: tx }
    }

    /// Attempts to read an event.
    /// This function will block the current thread.
    pub fn next(&self) -> Result<InputEvent, RecvError> {
        self.rx.recv()
    }
}
