use crate::mmu::Memory;

/// ## ProhibitedMem
/// - E000	FDFF	Mirror of C000~DDFF (ECHO RAM)	Nintendo says use of this area is prohibited.
/// - FEA0	FEFF	Not Usable	Nintendo says use of this area is prohibited
pub struct ProhibitedMem {}

impl ProhibitedMem {
    pub fn new() -> ProhibitedMem {
        ProhibitedMem {}
    }
}

impl Memory for ProhibitedMem {
    fn get(&self, i: u16) -> u8 {
        panic!("ProhibitedMem access denied, addr: 0x{:04X}", i);
    }

    fn set(&mut self, i: u16, v: u8) {
        panic!("ProhibitedMem access denied, addr: 0x{:04X}", i);
    }
}