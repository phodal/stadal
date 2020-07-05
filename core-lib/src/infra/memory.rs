use heim::{memory, units::information};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StadalMemory {
    pub total: String,
    pub free: String,
    pub available: String,
}

pub async fn get_memory() -> StadalMemory {
    let memory = memory::memory().await.unwrap();

    let total = memory.total().get::<information::megabyte>();
    let free = memory.free().get::<information::megabyte>();
    let available = memory.available().get::<information::megabyte>();

    StadalMemory {
        total: total.to_string(),
        free: free.to_string(),
        available: available.to_string(),
    }
}

