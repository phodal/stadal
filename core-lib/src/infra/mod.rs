mod cleaner;


mod lang;
pub use lang::{get_languages, Lang};

pub mod language;

mod host;
pub use host::{StadalHost, get_host};

pub mod memory;
pub use memory::{StadalMemory, get_memory};

pub mod notif;
