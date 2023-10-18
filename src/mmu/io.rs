use crate::mmu::Memory;

/// ## IO
/// - FF00	FF7F	[I/O Registers](https://gbdev.io/pandocs/Memory_Map.html#io-ranges)
pub struct IO {
    buf: [u8; 0x0080],
}

impl IO {
    pub fn new() -> Self {
        IO { buf: [0u8; 0x0080] }
    }
}

const OFFSET: usize = 0xFF00;

impl Memory for IO {
    fn get(&self, i: u16) -> u8 {
        //todo 这是假的实现
        self.buf[i as usize - OFFSET]
    }

    fn set(&mut self, i: u16, v: u8) {
        //todo 这是假的实现
        self.buf[i as usize - OFFSET] = v;
    }
}

