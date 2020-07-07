mod disk;
pub use disk::get_disks;

mod cpu;
pub use cpu::get_cpu;

mod cleaner;
pub use cleaner::{get_clean_size, CleanSize};

mod lang;
pub use lang::{get_languages, Lang};

pub mod language;

mod host;
pub use host::{StadalHost, get_host};

pub mod memory;
pub use memory::{StadalMemory, get_memory};

