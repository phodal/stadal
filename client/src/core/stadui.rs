use failure::{AsFail, Error, Fail};
use futures::{Async, future, Future, Poll, Sink, Stream};
use futures::sync::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
use futures::sync::oneshot::{self, Receiver, Sender};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log::LevelFilter;
use tokio::io;
use xdg::BaseDirectories;

use xrl::{Client, ClientError, Frontend, FrontendBuilder, spawn, XiNotification};
use crate::core::Command;

pub enum CoreEvent {
    Notify(XiNotification),
}

pub struct Stadui {
    client: Client,
    exit: bool,
    core_events: UnboundedReceiver<CoreEvent>,
}

impl Stadui {
    /// Create a new Tui instance.
    pub fn new(client: Client, events: UnboundedReceiver<CoreEvent>) -> Result<Self, Error> {
        Ok(Stadui {
            exit: false,
            client,
            core_events: events,
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
                tokio::spawn(self.send_memory().map_err(|_| ()));
            }
        }
    }

    fn send_memory(&mut self) -> impl Future<Item=(), Error=ClientError> {
        let params = json!("");
        self.client.notify("send_memory", params).and_then(|_| Ok(()))
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
        self.poll_rpc();
        if self.exit {
            info!("exiting the TUI");
            return Ok(Async::Ready(()));
        }
        Ok(Async::NotReady)
    }
}
