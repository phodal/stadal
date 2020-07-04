use futures::Future;
use serde_json::Value;

use crate::srl::errors::ClientError;
use crate::srl::protocol;

/// A client to send notifications and request to xi-core.
#[derive(Clone)]
pub struct Client(pub protocol::Client);

impl Client {
    /// Send a notification to the core. Most (if not all) notifications
    /// supported by the core are already implemented, so this method
    /// should not be necessary in most cases.
    pub fn notify(
        &self,
        method: &str,
        params: Value,
    ) -> impl Future<Item=(), Error=ClientError> {
        info!(">>> notification: method={}, params={}", method, &params);
        self.0
            .notify(method, params)
            .map_err(|_| ClientError::NotifyFailed)
    }
}
