use crate::mmu::Memory;

/// ## InterruptFlag
/// - FFFF	FFFF	[Interrupt](https://gbdev.io/pandocs/Interrupts.html#interrupts) Enable register (IE)
pub struct InterruptFlag {
    // todo 尚未实现
}

impl InterruptFlag {
    pub fn new() -> InterruptFlag {
        InterruptFlag {}
    }
}

impl Memory for InterruptFlag {
    fn get(&self, i: u16) -> u8 {
        0
    }

    fn set(&mut self, i: u16, v: u8) {}
}