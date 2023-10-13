pub mod cartridge;
pub mod mmu;
pub mod cpu;

/// game frame frequency 60FPS
/// 16ms/frame
const FPS: u64 = 60;

pub type Result<T> = std::result::Result<T, String>;
