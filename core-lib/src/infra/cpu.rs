use serde_json::Value;
use heim::units::{frequency};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct StadalCpu {
    cores: String,
    current_ghz: String,
    min_ghz: String,
    max_ghz: String,
}

impl StadalCpu {
    pub fn new() -> StadalCpu {
        StadalCpu {
            cores: "".to_string(),
            current_ghz: "".to_string(),
            min_ghz: "".to_string(),
            max_ghz: "".to_string(),
        }
    }
}


// based on github.com/nushell/nushell/crates/nu_plugin_sys/src/nu/mod.rs
// MIT License
//
// Copyright (c) 2019 - 2020 Yehuda Katz, Jonathan Turner
pub async fn get_cpu() -> Option<Value> {
    match futures::future::try_join(heim::cpu::logical_count(), heim::cpu::frequency()).await {
        Ok((num_cpu, cpu_speed)) => {
            let mut cpu = StadalCpu::new();
            cpu.cores = num_cpu.to_string();

            let current_speed =
                (cpu_speed.current().get::<frequency::hertz>() as f64 / 1_000_000_000.0 * 100.0)
                    .round()
                    / 100.0;
            cpu.current_ghz = current_speed.to_string();

            if let Some(min_speed) = cpu_speed.min() {
                let min_speed =
                    (min_speed.get::<frequency::hertz>() as f64 / 1_000_000_000.0 * 100.0).round()
                        / 100.0;

                cpu.min_ghz = min_speed.to_string();

            }

            if let Some(max_speed) = cpu_speed.max() {
                let max_speed =
                    (max_speed.get::<frequency::hertz>() as f64 / 1_000_000_000.0 * 100.0).round()
                        / 100.0;

                cpu.max_ghz = max_speed.to_string();
            }

            Some(json!(cpu))
        }
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::infra::cpu::get_cpu;
    use futures::{Future, Stream};
    use serde_json::Value;
    use futures::executor::block_on;

    #[test]
    fn should_get_cpu_info() {
        let value = block_on(get_cpu_info());
        println!("{}, ", json!(value));
    }

    async fn get_cpu_info() -> Option<Value> {
        get_cpu().await
    }
}