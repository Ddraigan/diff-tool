use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use anyhow::Result;

use super::key::Key;

pub enum InputEvent {
    /// An input event occurred.
    Input(Key),
    /// A tick event occurred.
    Tick,
    // A Terminal resize event occurred.
    // Resize,
}

/// A small event handler that wrap crossterm input and tick event. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct EventHandler {
    /// Event receiver channel
    receiver: Receiver<InputEvent>,
    /// Event sender channel
    sender: Sender<InputEvent>,
    /// Event handler thread
    handler: JoinHandle<()>,
}

impl EventHandler {
    /// Constructs an new instance of `Events` with the default config.
    /// mpsc setup
    pub fn new(tick_rate: Duration) -> Self {
        let (sender, receiver) = channel();

        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    // poll for tick rate duration, if no event, sent tick event.
                    if crossterm::event::poll(timeout).expect("No events available") {
                        if let crossterm::event::Event::Key(key) =
                            crossterm::event::read().expect("Undable to read event")
                        {
                            if key.kind == crossterm::event::KeyEventKind::Press {
                                let key = Key::from(key);
                                sender
                                    .send(InputEvent::Input(key))
                                    .expect("Failed to send terminal event");
                            }
                        }
                    }

                    // Solves ghost input after holding down
                    if last_tick.elapsed() >= tick_rate {
                        sender
                            .send(InputEvent::Tick)
                            .expect("failed to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };

        EventHandler {
            receiver,
            sender,
            handler,
        }
    }

    /// Attempts to read an event.
    /// This function will block the current thread.
    pub fn next(&self) -> Result<InputEvent> {
        Ok(self.receiver.recv()?)
    }
}
