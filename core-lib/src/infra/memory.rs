use heim::{memory, units::information};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StadalMemory {
    pub total: String,
    pub free: String,
    pub available: String,
    pub swap_total: String,
    pub swap_free: String,
    pub swap_used: String,
}

impl StadalMemory {
    pub fn new() -> StadalMemory {
        StadalMemory {
            total: "".to_string(),
            free: "".to_string(),
            available: "".to_string(),
            swap_total: "".to_string(),
            swap_free: "".to_string(),
            swap_used: "".to_string()
        }
    }
}

pub async fn get_memory() -> StadalMemory {
    let mut stadal_memory = StadalMemory::new();
    let (memory_result, swap_result) =
        futures::future::join(memory::memory(), memory::swap()).await;

    if let Ok(memory) = memory_result {
        stadal_memory.total = memory.total().get::<information::byte>().to_string();
        stadal_memory.free = memory.free().get::<information::byte>().to_string();
        stadal_memory.available = memory.available().get::<information::byte>().to_string();
    }

    if let Ok(swap) = swap_result {
        stadal_memory.swap_total = swap.total().get::<information::byte>().to_string();
        stadal_memory.swap_free = swap.free().get::<information::byte>().to_string();
        stadal_memory.swap_used = swap.used().get::<information::byte>().to_string();
    }

    stadal_memory
}

