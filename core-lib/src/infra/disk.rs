use heim::disk;
use heim::units::{information};
use std::ffi::OsStr;
use futures::StreamExt;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StadalDisk {
    device: String,
    filesystem: String,
    mount: String,
    total: String,
    used: String,
    free: String,
}

impl StadalDisk {
    pub fn new() -> StadalDisk {
        StadalDisk {
            device: "".to_string(),
            filesystem: "".to_string(),
            mount: "".to_string(),
            total: "".to_string(),
            used: "".to_string(),
            free: "".to_string(),
        }
    }
}

pub async fn get_disks() -> Option<Vec<StadalDisk>> {
    let mut output = vec![];

    let partitions = heim::disk::partitions_physical().await?;
    futures::pin_mut!(partitions);

    while let Some(part) = partitions.next().await {
        let part = part?;
        let mut sdisk = StadalDisk::new();
        sdisk.device = part.device().unwrap_or_else(|| OsStr::new("N/A")).to_string_lossy();
        sdisk.filesystem = part.file_system().as_str();
        sdisk.mount = part.mount_point().to_string_lossy();
        if let Ok(usage) = disk::usage(part.mount_point().to_path_buf()).await {
            sdisk.mount = usage.total().get::<information::byte>().to_string();
            sdisk.used = usage.used().get::<information::byte>().to_string();
            sdisk.free = usage.free().get::<information::byte>().to_string();
        }

        output.push(sdisk);
    }

    if !output.is_empty() {
        Some(output)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::infra::disk::get_disks;
    use futures::executor::block_on;

    #[test]
    fn get_disks_sizes() {
        let disks = block_on(get_disks()).unwrap();
        println!("{},", json!(&disks));
    }
}