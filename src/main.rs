use std::{fs, io, process};
use std::path::{Path, PathBuf};

use futures::executor::block_on;
use heim::{memory, Result, units::information};
use log::{error, info, warn};

use xi_rpc::RpcLoop;
use core_lib::application::Stadal;

fn setup_logging(logging_path: &Path) -> Result<()> {
    let level_filter = match std::env::var("XI_LOG") {
        Ok(level) => match level.to_lowercase().as_ref() {
            "trace" => log::LevelFilter::Trace,
            "debug" => log::LevelFilter::Debug,
            _ => log::LevelFilter::Info,
        },
        Err(_) => log::LevelFilter::Info,
    };

    let mut fern_dispatch = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message,
            ))
        })
        .level(level_filter)
        .chain(io::stderr());

    info!("Logging with fern is set up");

    fern_dispatch = fern_dispatch.chain(fern::log_file(logging_path)?);
    fern_dispatch.apply();

    info!("Writing logs to: {}", logging_path.display());
    Ok(())
}

#[tokio::main]
async fn main() {
    let mut state = Stadal::new();
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut rpc_looper = RpcLoop::new(stdout);

    let memory = print_memory();

    block_on(memory);

    if let Err(e) = setup_logging(PathBuf::from("./stadal.log").as_path()) {
        eprintln!("[ERROR] setup_logging returned error, logging not enabled: {:?}", e);
    }

    match rpc_looper.mainloop(|| stdin.lock(), &mut state) {
        Ok(_) => (),
        Err(err) => {
            error!("exited with error:\n{:?}", err);
            process::exit(1);
        }
    }
}

async fn print_memory() {
    let memory = memory::memory().await.unwrap();
    let swap = memory::swap().await.unwrap();

    println!("              total        free   available");
    println!(
        "{:>7} {:>11?} {:>11?} {:>11?}",
        "Mem:",
        memory.total().get::<information::megabyte>(),
        memory.free().get::<information::megabyte>(),
        memory.available().get::<information::megabyte>(),
    );
    println!(
        "{:>7} {:>11?} {:>11?} {:>11?}",
        "Swap:",
        swap.total().get::<information::megabyte>(),
        swap.used().get::<information::megabyte>(),
        swap.free().get::<information::megabyte>(),
    );
}