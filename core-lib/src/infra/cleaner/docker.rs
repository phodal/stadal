use std::{fs};

pub fn get_docker_size() -> u64 {
    let home = std::env::var("HOME").unwrap();
    let path = format!("{}/Library/Containers/com.docker.docker/Data/vms/0/data/Docker.raw", home);
    match fs::metadata(path) {
        Ok(size) => {
            size.len()
        },
        Err(_) => {
            0
        },
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn try_get_docker_size() {

    }
}