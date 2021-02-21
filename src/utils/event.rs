use std::io;
use std::time::Duration;

use termion::{event::Key, input::TermRead};

use tokio::{sync::mpsc, task, time};

#[derive(Debug)]
pub enum Event<I> {
    Input(I),
    Tick,
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
    _input_handle: task::JoinHandle<()>,
    _tick_handle: task::JoinHandle<()>,
}

impl Events {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(2);
        let _input_handle = {
            let tx = tx.clone();
            tokio::spawn(async move {
                let stdin = io::stdin();

                for i in stdin.keys() {
                    if let Ok(key) = i {
                        if let Err(err) = tx.send(Event::Input(key)).await {
                            eprintln!("{}", err);
                            break;
                        }
                    }
                }
            })
        };

        let _tick_handle = {
            let tx = tx.clone();
            tokio::spawn(async move {
                loop {
                    if tx.send(Event::Tick).await.is_err() {
                        break;
                    }
                    time::sleep(Duration::from_millis(1000)).await;
                }
            })
        };

        Events {
            rx,
            _input_handle,
            _tick_handle,
        }
    }

    pub async fn next(&mut self) -> Option<Event<Key>> {
        self.rx.recv().await
    }

    pub fn close(&mut self) {
        self._input_handle.abort();
        self._tick_handle.abort();
        self.rx.close();
    }
}
