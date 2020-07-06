use heim::{memory, units::information};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StadalMemory {
    pub total: String,
    pub free: String,
    pub available: String,
}

pub async fn get_memory() -> StadalMemory {
    let memory = memory::memory().await.unwrap();

    let total = memory.total().get::<information::byte>();
    let free = memory.free().get::<information::byte>();
    let available = memory.available().get::<information::byte>();

    StadalMemory {
        total: total.to_string(),
        free: free.to_string(),
        available: available.to_string(),
    }
}

