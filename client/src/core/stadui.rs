use failure::Error;
use futures::{Async, Future, Poll, Stream};
use futures::sync::mpsc::{UnboundedReceiver};
use tokio::io;

use xrl::{Client, ClientError, XiNotification};
use crate::core::Command;
use crate::core::window::Window;

pub enum CoreEvent {
    Notify(XiNotification),
}

pub struct Stadui {
    exit: bool,
    window: Window,
    core_events: UnboundedReceiver<CoreEvent>,
}

impl Stadui {
    pub fn new(client: Client, events: UnboundedReceiver<CoreEvent>) -> Result<Self, Error> {
        Ok(Stadui {
            exit: false,
            core_events: events,
            window: Window::new(client),
        })
    }

    fn handle_core_event(&mut self, event: CoreEvent) {
        match event {
            CoreEvent::Notify(notification) => match notification {
                _ => info!("ignoring Xi core notification: {:?}", notification),
            }
        };
    }

    pub fn run_command(&mut self, cmd: Command) {
        match cmd {
            Command::SendMemory => {
                tokio::spawn(self.window.send_memory().map_err(|_| ()));
            }
        }
    }

    fn poll_rpc(&mut self) {
        debug!("polling for RPC messages");
        loop {
            match self.core_events.poll() {
                Ok(Async::Ready(Some(event))) => self.handle_core_event(event),
                Ok(Async::Ready(None)) => {
                    info!("The RPC endpoint exited normally. Shutting down the TUI");
                    self.exit = true;
                    return;
                }
                Ok(Async::NotReady) => {
                    debug!("no more RPC event, done polling");
                    return;
                }
                Err(e) => {
                    error!("The RPC endpoint exited with an error: {:?}", e);
                    error!("Shutting down the TUI");
                    self.exit = true;
                    return;
                }
            }
        }
    }
}

impl Future for Stadui {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        debug!("polling the TUI");
        self.poll_rpc();
        if self.exit {
            info!("exiting the TUI");
            return Ok(Async::Ready(()));
        }
        Ok(Async::NotReady)
    }
}
