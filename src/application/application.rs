use std::sync::{Arc, Mutex, MutexGuard, Weak};

use log::{error, info, warn};
use serde::de::{self, Deserialize, Deserializer};
use serde::ser::{self, Serialize, Serializer};
use serde_json::{self, Value};

use xi_rpc::{Handler, RemoteError, RpcCtx, RpcPeer};
use std::path::PathBuf;
use crate::application::application::CoreNotification::TracingConfig;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "method", content = "params")]
pub enum CoreNotification {
    TracingConfig { enabled: bool },
    SetTheme { theme_name: String },
    /// Notifies `xi-core` that the client has started.
    ClientStarted {
        #[serde(default)]
        config_dir: Option<PathBuf>,
        /// Path to additional plugins, included by the client.
        #[serde(default)]
        client_extras_dir: Option<PathBuf>,
    },
}

pub struct Client(RpcPeer);

impl Client {
    pub fn new(peer: RpcPeer) -> Self {
        Client(peer)
    }
}

#[allow(dead_code)]
pub struct CoreState {
    peer: Client,
}

impl CoreState {
    pub(crate) fn new(peer: &RpcPeer) -> Self {
        CoreState {
            peer: Client::new(peer.clone())
        }
    }
    pub(crate) fn client_notification(&mut self, cmd: CoreNotification) {
        use self::CoreNotification::*;
        match cmd {
            ClientStarted { .. } => (),
            _ => {
                 // self.not_command(view_id, language_id);
            }
        }
    }

    pub(crate) fn finish_setup(&mut self, self_ref: WeakStadalCore) {

    }

    pub(crate) fn handle_idle(&mut self, token: usize) {
        match token {
            _ => {
                info!("token: {}", token);
            }
        }
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
pub struct WeakStadalCore(Weak<Mutex<CoreState>>);

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

    /// Returns a guard to the core state. A convenience around `Mutex::lock`.
    ///
    /// # Panics
    ///
    /// Panics if core has not yet received the `client_started` message.
    pub fn inner(&self) -> MutexGuard<CoreState> {
        match self {
            Stadal::Running(ref inner) => inner.lock().unwrap(),
            Stadal::Waiting => panic!(
                "core does not start until client_started \
                 RPC is received"
            ),
        }
    }

    /// Returns a new reference to the core state, if core is running.
    fn weak_self(&self) -> Option<WeakStadalCore> {
        match self {
            Stadal::Running(ref inner) => Some(WeakStadalCore(Arc::downgrade(inner))),
            Stadal::Waiting => None,
        }
    }
}

impl Handler for Stadal {
    type Notification = CoreNotification;
    type Request = ();

    fn handle_notification(&mut self, ctx: &RpcCtx, rpc: Self::Notification) {
        // We allow tracing to be enabled before event `client_started`
        if let TracingConfig { enabled } = rpc {
            match enabled {
                // true => xi_trace::enable_tracing(),
                // false => xi_trace::disable_tracing(),
            }
            info!("tracing in core = {:?}", enabled);
            if self.is_waiting() {
                return;
            }
        }


        let state = CoreState::new(ctx.get_peer());
        let state = Arc::new(Mutex::new(state));
        *self = Stadal::Running(state);
        let weak_self = self.weak_self().unwrap();
        self.inner().finish_setup(weak_self);
    }

    fn handle_request(&mut self, ctx: &RpcCtx, rpc: Self::Request) -> Result<Value, RemoteError> {
        println!("!!!!!");
        warn!("zzz");
        Ok(json!(1))
    }

    fn idle(&mut self, _ctx: &RpcCtx, token: usize) {
        self.inner().handle_idle(token);
    }
}