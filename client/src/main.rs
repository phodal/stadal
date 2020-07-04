extern crate futures;
#[macro_use]
extern crate log;
extern crate log4rs;
#[macro_use]
extern crate serde_json;

use failure::{Error};
use futures::{future, Future, Stream};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log::LevelFilter;
use xdg::BaseDirectories;

use xrl::spawn;

use crate::core::{Command, Stadui, TuiServiceBuilder};

mod core;

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

pub fn run() -> Result<(), Error> {
    configure_logs("client.log");
    tokio::run(future::lazy(move || {
        info!("starting xi-core");
        let (tui_service_builder, core_events_rx) = TuiServiceBuilder::new();
        let (client, core_stderr) = spawn(
            "/Users/fdhuang/repractise/stadaljs/target/debug/stadaljs",
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
            let conf_dir = BaseDirectories::with_prefix("stadaljs")
                .ok()
                .and_then(|dirs| Some(dirs.get_config_home().to_string_lossy().into_owned()));

            let client_clone = client.clone();
            client
                .client_started(conf_dir.as_ref().map(|dir| &**dir), None)
                .map_err(|e| error!("failed to send \"client_started\" {:?}", e))
                .and_then(move |_| {
                    info!("initializing the Stadui");
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
