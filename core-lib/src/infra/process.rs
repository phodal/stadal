use std::time::Duration;
use std::usize;

use futures::{StreamExt, TryStreamExt};
use heim::process::{self as process, Process, ProcessResult};
use heim::units::{information, ratio, Ratio};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StadalProcess {
    pid: String,
    name: String,
    status: String,
    cpu: String,
    mem: String,
    virtual_mem: String,
    parent: String,
    exe: String,
    command: String,
}

impl StadalProcess {
    pub fn new() -> StadalProcess {
        StadalProcess {
            pid: "".to_string(),
            name: "".to_string(),
            status: "".to_string(),
            cpu: "".to_string(),
            mem: "".to_string(),
            virtual_mem: "".to_string(),
            parent: "".to_string(),
            exe: "".to_string(),
            command: "".to_string(),
        }
    }
}

async fn usage(process: Process) -> ProcessResult<(process::Process, Ratio, process::Memory)> {
    let usage_1 = process.cpu_usage().await?;
    futures_timer::Delay::new(Duration::from_millis(100)).await;
    let usage_2 = process.cpu_usage().await?;

    let memory = process.memory().await?;

    Ok((process, usage_2 - usage_1, memory))
}

pub async fn get_processes() -> Option<Vec<StadalProcess>> {
    let mut output = vec![];
    let mut results = process::processes()
        .map_ok(|process| {
            usage(process)
        })
        .try_buffer_unordered(usize::MAX);
    futures::pin_mut!(results);


    while let Some(res) = results.next().await {
        if let Ok((process, usage, memory)) = res {
            let mut stadal_process = StadalProcess::new();

            stadal_process.pid = process.pid().to_string();
            if let Ok(name) = process.name().await {
                stadal_process.name = name;
            }
            if let Ok(status) = process.status().await {
                stadal_process.status = format!("{:?}", status);
            }
            stadal_process.cpu = usage.get::<ratio::percent>().to_string();
            stadal_process.mem = memory.rss().get::<information::byte>().to_string();
            stadal_process.virtual_mem = memory.vms().get::<information::byte>().to_string();

            if let Ok(parent_pid) = process.parent_pid().await {
                stadal_process.parent = parent_pid.to_string();
            }

            if let Ok(exe) = process.exe().await {
                stadal_process.exe = exe.to_string_lossy().to_string();
            }

            #[cfg(not(windows))]
                {
                    if let Ok(command) = process.command().await {
                        stadal_process.command = command.to_os_string().to_string_lossy().to_string();
                    }
                }

            output.push(stadal_process)

        }
    }

    if !output.is_empty() {
        Some(output)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    use crate::infra::process::get_processes;

    #[test]
    fn get_disks_sizes() {
        let processes = block_on(get_processes()).unwrap();
        println!("{},", json!(&processes));
    }
}