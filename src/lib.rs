pub mod cartridge;
pub mod mmu;
pub mod cpu;

#[cfg(test)]
mod tests;
mod io;
mod gpu;
mod interrupt;

/// game frame frequency 60FPS
/// 16ms/frame
const FPS: u64 = 60;

pub type Result<T> = std::result::Result<T, String>;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum GBTerm {
    GB,
    // Original GameBoy (GameBoy Classic)
    GBP,
    // GameBoy Pocket/GameBoy Light
    GBC,
    // GameBoy Color
    SGB, // Super GameBoy
}
