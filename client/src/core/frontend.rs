use futures::{Async, future, Future, Poll, Sink, Stream};
use futures::sync::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
use futures::sync::oneshot::{self, Receiver, Sender};

use xrl::{IntoStaticFuture, XiNotification, Client, FrontendBuilder, Frontend};

use crate::core::CoreEvent;

pub struct TuiService(UnboundedSender<CoreEvent>);

impl Frontend for TuiService {
    type NotificationResult = Result<(), ()>;

    fn handle_notification(&mut self, notification: XiNotification) -> Self::NotificationResult {
        self.0.start_send(CoreEvent::Notify(notification)).unwrap();
        self.0.poll_complete().unwrap();
        Ok(())
    }
}

pub struct NoErrorReceiver<T>(Receiver<T>);

impl<T> Future for NoErrorReceiver<T> {
    type Item = T;
    type Error = ();
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.0.poll().map_err(|_cancelled| ())
    }
}

pub struct TuiServiceBuilder(UnboundedSender<CoreEvent>);

impl TuiServiceBuilder {
    pub fn new() -> (Self, UnboundedReceiver<CoreEvent>) {
        let (tx, rx) = unbounded();
        (TuiServiceBuilder(tx), rx)
    }
}

impl FrontendBuilder for TuiServiceBuilder {
    type Frontend = TuiService;
    fn build(self, _client: Client) -> Self::Frontend {
        TuiService(self.0)
    }
}