use std::{fs};
use crate::infra::CleanSize;

pub fn get_docker_env() -> CleanSize {
    let home = std::env::var("HOME").unwrap();
    let path = format!("{}/Library/Containers/com.docker.docker/Data/vms/0/data/Docker.raw", home);
    match fs::metadata(path.clone()) {
        Ok(size) => {
            CleanSize::new(
                String::from("docker"),
                size.len().to_string(),
                path,
            )
        }
        Err(_) => {
            CleanSize::new(
                String::from(""),
                String::from(""),
                String::from("")
            )
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn try_get_docker_size() {}
}