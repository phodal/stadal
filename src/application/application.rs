use std::sync::{Mutex, Arc, Weak};
use xi_rpc::{Handler, RpcCtx, RemoteError};

use serde::de::{self, Deserialize, Deserializer};
use serde::ser::{self, Serialize, Serializer};
use serde_json::{self, Value};

pub enum CoreNotification {
    SetTheme { theme_name: String },
    TracingConfig { enabled: bool },
}

#[allow(dead_code)]
pub struct CoreState {}

impl CoreState {
    pub(crate) fn client_notification(&mut self, cmd: CoreNotification) {
        use self::CoreNotification::*;
    }

    fn do_new_view(&mut self) {

    }
}

pub enum Stadal {
    // TODO: profile startup, and determine what things (such as theme loading)
    // we should be doing before client_init.
    Waiting,
    Running(Arc<Mutex<CoreState>>),
}

/// A weak reference to the main state. This is passed to plugin threads.
#[derive(Clone)]
pub struct WeakStadal(Weak<Mutex<CoreState>>);

#[allow(dead_code)]
impl Stadal {
    pub fn new() -> Self {
        Stadal::Waiting
    }

    /// Returns `true` if the `client_started` has not been received.
    fn is_waiting(&self) -> bool {
        match *self {
            Stadal::Waiting => true,
            _ => false,
        }
    }
}

impl Handler for Stadal {
    type Notification = ();
    type Request = ();

    fn handle_notification(&mut self, ctx: &RpcCtx, rpc: Self::Notification) {
        unimplemented!()
    }

    fn handle_request(&mut self, ctx: &RpcCtx, rpc: Self::Request) -> Result<Value, RemoteError> {
        unimplemented!()
    }
}