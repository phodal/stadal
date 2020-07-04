use std::future::Future;
use std::process::{Command, Stdio};

use log::{error, info};
use tokio_process::CommandExt;
use serde_json::{self, Value, json};

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

/// Start Xi core, and spawn an RPC client on the current tokio executor.
///
/// # Panics
///
/// This function calls
/// [`tokio::spawn`](https://docs.rs/tokio/0.1.21/tokio/executor/fn.spawn.html)
/// so it will panic if the default executor is not set or if spawning
/// onto the default executor returns an error.
pub fn spawn<B, F>(executable: &str, builder: B) -> Result<(Client, CoreStderr), ClientError>
    where
        F: Frontend + 'static + Send,
        B: FrontendBuilder<Frontend = F> + 'static,
{
    spawn_command(Command::new(executable), builder)
}

/// Same as [`spawn`] but accepts an arbitrary [`std::process::Command`].
pub fn spawn_command<B, F>(
    mut command: Command,
    builder: B,
) -> Result<(Client, CoreStderr), ClientError> {
    info!("starting xi-core");
    let mut xi_core = command
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .env("RUST_BACKTRACE", "1")
        .spawn_async()?;

    let stdout = xi_core.stdout().take().unwrap();
    let stdin = xi_core.stdin().take().unwrap();
    let stderr = xi_core.stderr().take().unwrap();
    let core = Core {
        core: xi_core,
        stdout,
        stdin,
    };

    let (endpoint, client) = Endpoint::new(core, builder);

    info!("spawning the Xi-RPC endpoint");
    tokio::spawn(endpoint.map_err(|e| error!("Endpoint exited with an error: {:?}", e)));

    Ok((Client(client), CoreStderr::new(stderr)))
}

fn main() {
    println!("Hello, world!");
    tokio::run(future::lazy(move || {
        info!("starting xi-core");

        let (client, core_stderr) = spawn(
            matches.value_of("core").unwrap_or("/Users/fdhuang/repractise/stadal/target/debug/stadal"),
            tui_service_builder,
        ).unwrap();

        info!("starting logging xi-core errors");
        tokio::spawn(
            core_stderr
                .for_each(|msg| {
                    error!("core: {}", msg);
                    Ok(())
                })
                .map_err(|_| ()),
        );

        tokio::spawn(future::lazy(move || {
            let client_clone = client.clone();
            client.notify("hello", json!({ "view_id": 1 }));

        }));

        Ok(())
    }));
}
