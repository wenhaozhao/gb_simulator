use crate::memory::Memory;

pub mod mbc1;
pub mod mbc2;

enum BankMode {
    ROM,
    RAM,
}

pub trait MBC: Memory {
}
