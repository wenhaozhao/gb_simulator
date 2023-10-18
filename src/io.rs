use crate::interrupt::IFR;
use crate::mmu::Memory;

/// ## IO
/// - FF00	FF7F	[I/O Registers](https://gbdev.io/pandocs/Memory_Map.html#io-ranges)
pub struct IO {
    ifr: IFR,
    buf: [u8; 0x0080],
}

impl IO {
    pub fn new() -> Self {
        IO { ifr: IFR::new(), buf: [0u8; 0x0080] }
    }
}

const OFFSET: usize = 0xFF00;

//todo 这是假的实现
impl Memory for IO {
    fn get(&self, i: u16) -> u8 {
        match i {
            0xFF00..=0xFF0E => self.buf[i as usize - OFFSET],
            0xFF0F => self.ifr.get(i),
            0xFF10..=0xFF7F => self.buf[i as usize - OFFSET],
            addr => panic!("WorkRam access denied, addr: 0x{:04X}", addr),
        }
    }

    fn set(&mut self, i: u16, v: u8) {
        match i {
            0xFF00..=0xFF0E => self.buf[i as usize - OFFSET] = v,
            0xFF0F => self.ifr.set(i, v),
            0xFF10..=0xFF7F => self.buf[i as usize - OFFSET] = v,
            addr => panic!("WorkRam access denied, addr: 0x{:04X}", addr),
        }
    }
}

