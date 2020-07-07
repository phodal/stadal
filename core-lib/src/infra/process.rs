use std::time::Duration;
use std::usize;

use futures::{StreamExt, TryStreamExt};
use heim::process::{self as process, Process, ProcessResult};
use heim::units::{information, ratio, Ratio};
use futures::executor::block_on;
use std::collections::HashMap;
use std::cmp::Ordering::Equal;
use std::cmp::Ordering;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StadalProcess {
    pid: i32,
    name: String,
    status: String,
    cpu_usage: f32,
    mem: u64,
    virtual_mem: u64,
    parent: String,
    exe: String,
    command: String,
}

impl StadalProcess {
    pub fn new() -> StadalProcess {
        StadalProcess {
            pid: 0,
            name: "".to_string(),
            status: "".to_string(),
            cpu_usage: 0.0,
            mem: 0,
            virtual_mem: 0,
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

            stadal_process.pid = process.pid();
            if let Ok(name) = process.name().await {
                stadal_process.name = name;
            }
            if let Ok(status) = process.status().await {
                stadal_process.status = format!("{:?}", status);
            }
            stadal_process.cpu_usage = usage.get::<ratio::percent>();
            stadal_process.mem = memory.rss().get::<information::byte>();
            stadal_process.virtual_mem = memory.vms().get::<information::byte>();

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

#[derive(PartialEq, Eq)]
pub enum ProcessTableSortOrder {
    Ascending = 0,
    Descending = 1,
}

pub fn field_comparator() -> fn(&StadalProcess, &StadalProcess) -> Ordering {
    |pa, pb| pa.cpu_usage.partial_cmp(&pb.cpu_usage).unwrap_or(Equal)
}

pub fn get_sort_processes() -> Vec<StadalProcess> {
    let mut proc_vec = block_on(get_processes()).unwrap();
    let mut pm = HashMap::with_capacity(400);

    let mut process_ids = vec![];
    for x in proc_vec.clone() {
        pm.insert(x.pid, x.clone());
        process_ids.push(x.pid);
    }

    let sorter = field_comparator();
    let sortorder = &ProcessTableSortOrder::Descending;

    proc_vec.sort_by(|a, b| {
        let ord = sorter(a, b);
        match sortorder {
            ProcessTableSortOrder::Ascending => ord,
            ProcessTableSortOrder::Descending => ord.reverse(),
        }
    });

    // let mut results = vec![];
    // for (k, v) in pm.iter() {
    //     results.push(v.clone());
    // }
    //
    // results
    proc_vec
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    use crate::infra::process::{get_processes, get_sort_processes};

    #[test]
    fn get_processes_test() {
        let processes = block_on(get_processes()).unwrap();
        println!("{},", json!(&processes));
    }

    #[test]
    fn get_sort_processes_test() {
        let processes = get_sort_processes();
        println!("{},", json!(&processes));
    }
}