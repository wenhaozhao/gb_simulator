pub mod memory;
pub mod cartridge;
pub mod mmu;

pub type Result<T> = std::result::Result<T, String>;