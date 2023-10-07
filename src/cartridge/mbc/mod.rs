use crate::memory::Memory;

pub mod mbc1;

enum BankMode {
    ROM,
    RAM,
}

pub trait MBC: Memory {
}
