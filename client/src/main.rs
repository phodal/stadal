extern crate futures;
#[macro_use]
extern crate log;
extern crate log4rs;
#[macro_use]
extern crate serde_json;

use std::io;

use failure::{AsFail, Error, Fail};
use futures::{Async, future, Future, Poll, Sink, Stream};
use futures::sync::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
use futures::sync::oneshot::{self, Receiver, Sender};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log::LevelFilter;
use xdg::BaseDirectories;

use xrl::{Client, ClientError, Frontend, FrontendBuilder, spawn, XiNotification};

use crate::core::{CoreEvent, Stadui, Command};

mod core;

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

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();

        writeln!(stderr, "error: {}", e).unwrap();
        error!("error: {}", e);

        writeln!(stderr, "caused by: {}", e.as_fail()).unwrap();
        error!("error: {}", e);

        writeln!(stderr, "backtrace: {:?}", e.backtrace()).unwrap();
        error!("error: {}", e);

        ::std::process::exit(1);
    }
}

fn configure_logs(logfile: &str) {
    let tui = FileAppender::builder().build(logfile).unwrap();
    let config = Config::builder()
        .appender(Appender::builder().build("tui", Box::new(tui)))
        .logger(
            Logger::builder()
                .appender("tui")
                .additive(false)
                .build("xi_tui", LevelFilter::Debug),
        )
        .logger(
            Logger::builder()
                .appender("tui")
                .additive(false)
                .build("xrl", LevelFilter::Info),
        )
        .build(Root::builder().appender("tui").build(LevelFilter::Info))
        .unwrap();
    let _ = log4rs::init_config(config).unwrap();
}

fn run() -> Result<(), Error> {
    configure_logs("client.log");
    tokio::run(future::lazy(move || {
        info!("starting xi-core");
        let (tui_service_builder, core_events_rx) = TuiServiceBuilder::new();
        let (client, core_stderr) = spawn(
            "/Users/fdhuang/repractise/stadal/target/debug/stadal",
            tui_service_builder,
        )
            .unwrap();

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
            let conf_dir = BaseDirectories::with_prefix("stadal")
                .ok()
                .and_then(|dirs| Some(dirs.get_config_home().to_string_lossy().into_owned()));

            let client_clone = client.clone();
            client
                .client_started(conf_dir.as_ref().map(|dir| &**dir), None)
                .map_err(|e| error!("failed to send \"client_started\" {:?}", e))
                .and_then(move |_| {
                    info!("initializing the TUI");
                    let mut ui = Stadui::new(client_clone, core_events_rx)
                        .expect("failed to initialize the Stadui");
                    ui.run_command(Command::SendMemory);
                    ui.map_err(|e| error!("Stadui exited with an error: {:?}", e))
                })
        }));

        Ok(())
    }));

    Ok(())
}
