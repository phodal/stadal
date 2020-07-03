use std::{io, process};

use heim::{memory, Result, units::information};
use log::{error};

use crate::application::application::Stadal;

mod application;
mod domain;
mod infrastructure;

#[tokio::main]
async fn main() -> Result<()> {
    let mut state = Stadal::new();
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut rpc_looper = RpcLoop::new(stdout);

    // match rpc_looper.mainloop(|| stdin.lock(), &mut state) {
    //     Ok(_) => (),
    //     Err(err) => {
    //         error!("exited with error:\n{:?}", err);
    //         process::exit(1);
    //     }
    // }

    print_memory().await;

    Ok(())
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