use crate::mmu::Memory;

pub mod mbc1;
pub mod mbc2;
pub mod mbc3;
pub mod mbc5;

/// rom 16KB
const ROM_BANK_LEN: u16 = 0x4000;

/// rom_0 0x0000 - 0x3FFF
const ROM_0_BASE: u16 = 0x0000;
/// rom_0 0x0000 - 0x3FFF
const ROM_0_END: u16 = ROM_0_BASE + ROM_BANK_LEN - 1;

/// rom_x 0x4000 - 0x7FFF
const ROM_X_BASE: u16 = 0x4000;
/// rom_x-7F 0x4000 - 0x7FFF
const ROM_X_END: u16 = ROM_X_BASE + ROM_BANK_LEN - 1;

/// ram 8KB
const RAM_BANK_LEN: u16 = 0x2000;
/// ram_x - 0xBFFF
const RAM_X_BASE: u16 = 0xA000;
/// ram_x 0xA000 - 0xBFFF
const RAM_X_END: u16 = RAM_X_BASE + RAM_BANK_LEN - 1;

pub trait MBC: Memory {}
