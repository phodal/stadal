use crate::infra::cleaner::docker::{get_docker_env};

mod docker;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CleanSize {
    name: String,
    size: String,
    path: String,
}

impl CleanSize {
    fn new(name: String, size: String, path: String) -> CleanSize {
        CleanSize {
            name,
            size,
            path
        }
    }
}

pub fn get_clean_size() -> Vec<CleanSize> {
    let mut sizes = Vec::with_capacity(1);
    sizes.push(get_docker_env());

    sizes
}