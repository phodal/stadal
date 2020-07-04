use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

use futures::{future, Future, Poll, Stream};
use futures::sync::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
use futures::sync::oneshot::{self, Receiver, Sender};
use log::{error, info};
use serde_json::{self, json, Value};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_codec::FramedRead;
use tokio_process::{Child, ChildStderr, ChildStdin, ChildStdout, CommandExt};

use srl::srl::errors::ClientError;
use srl::srl::protocol;
use srl::srl::protocol::{Client as InnerClient, Endpoint, Service, IntoStaticFuture, ServiceBuilder};
use futures::future::{Either, FutureResult};
use srl::srl::client::Client;

struct Core {
    #[allow(dead_code)]
    core: Child,
    stdout: ChildStdout,
    stdin: ChildStdin,
}

impl Read for Core {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stdout.read(buf)
    }
}

impl AsyncRead for Core {
    // FIXME: do I actually have to implement this?
    unsafe fn prepare_uninitialized_buffer(&self, buf: &mut [u8]) -> bool {
        self.stdout.prepare_uninitialized_buffer(buf)
    }
}

impl Write for Core {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stdin.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stdin.flush()
    }
}

impl AsyncWrite for Core {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        self.stdin.shutdown()
    }
}

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

pub struct CoreStderr();

impl CoreStderr {
    fn new(stderr: ChildStderr) -> Self {
        CoreStderr()
    }
}

impl Stream for CoreStderr {
    type Item = String;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.0.poll()
    }
}

#[derive(Debug)]
pub enum XiNotification {}


/// The `Frontend` trait must be implemented by clients. It defines how the
/// client handles notifications and requests coming from `xi-core`.
pub trait Frontend {
    type NotificationResult: IntoStaticFuture<Item = (), Error = ()>;
    fn handle_notification(&mut self, notification: XiNotification) -> Self::NotificationResult;
}

/// A trait to build a type that implements `Frontend`.
pub trait FrontendBuilder {
    /// The type to build
    type Frontend: Frontend;

    /// Build the frontend with the given client.
    fn build(self, client: Client) -> Self::Frontend;
}

impl<B> ServiceBuilder for B
    where
        B: FrontendBuilder,
        B::Frontend: Send,
{
    type Service = B::Frontend;

    fn build(self, client: InnerClient) -> B::Frontend {
        <Self as FrontendBuilder>::build(self, Client(client))
    }
}

impl<F: Frontend + Send> Service for F {
    type T = Value;
    type E = Value;
    type RequestFuture = Box<dyn Future<Item=Self::T, Error=Self::E> + 'static + Send>;
    type NotificationFuture = Either<
        <<F as Frontend>::NotificationResult as IntoStaticFuture>::Future,
        FutureResult<(), ()>,
    >;

    fn handle_request(&mut self, method: &str, params: Value) -> Self::RequestFuture {
        unimplemented!()
    }

    #[allow(clippy::cognitive_complexity)]
    fn handle_notification(&mut self, method: &str, params: Value) -> Self::NotificationFuture {
        unimplemented!()
    }
}

pub enum CoreEvent {
    Notify(XiNotification),
}

pub struct TuiServiceBuilder(UnboundedSender<CoreEvent>);

impl TuiServiceBuilder {
    pub fn new() -> (Self, UnboundedReceiver<CoreEvent>) {
        let (tx, rx) = unbounded();
        (TuiServiceBuilder(tx), rx)
    }
}

fn main() {
    println!("Hello, world!");
    tokio::run(future::lazy(move || {
        info!("starting xi-core");

        let (tui_service_builder, core_events_rx) = TuiServiceBuilder::new();
        let (client, core_stderr) = spawn(
            "/Users/fdhuang/repractise/stadal/target/debug/stadal",
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
