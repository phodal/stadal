use crate::infra::cleaner::docker::get_docker_size;

mod docker;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CleanSize {
    name: String,
    size: String,
}

impl CleanSize {
    fn new(name: String, size: String) -> CleanSize {
        CleanSize {
            name,
            size
        }
    }
}

pub fn get_clean_size() -> Vec<CleanSize> {
    let mut sizes = Vec::with_capacity(1);
    let size = get_docker_size();
    sizes.push(CleanSize::new(String::from("docker"), size.to_string()));

    sizes
}