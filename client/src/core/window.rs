use xrl::{Client, ClientError};
use futures::{Future, Poll, Async};
use crate::core::CoreEvent;

pub struct Window {
    pub delayed_events: Vec<CoreEvent>,
    pub client: Client,
}

impl Window {
    pub fn new(client: Client) -> Window {
        Window {
            delayed_events: Vec::new(),
            client
        }
    }
}

impl Future for Window {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        debug!("polling the editor");

        debug!("handling delayed events");
        if !self.delayed_events.is_empty() {
            let delayed_events: Vec<CoreEvent> = self.delayed_events.drain(..).collect();
            for event in delayed_events {
                self.handle_core_event(event);
            }
        }

        Ok(Async::NotReady)
    }
}

impl Window {
    pub fn handle_core_event(&mut self, event: CoreEvent) {
        match event {
            CoreEvent::Notify(notification) => match notification {
                _ => info!("ignoring Xi core notification: {:?}", notification),
            },
        }
    }

    pub(crate) fn send_memory(&mut self) -> impl Future<Item=(), Error=ClientError> {
        let params = json!("");
        self.client.notify("send_memory", params).and_then(|_| Ok(()))
    }
}