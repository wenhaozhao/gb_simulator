use std::cell::RefCell;
use std::rc::Rc;

use crate::mmu::{BlockRam, Memory};

/// ## GPU
/// - 8000	9FFF	8 KiB Video RAM (VRAM)	In CGB mode, switchable bank 0/1
/// - FE00	FE9F	[Object attribute memory (OAM)](https://gbdev.io/pandocs/OAM.html#object-attribute-memory-oam)
pub struct GPU {
    buf: Rc<RefCell<Box<BlockRam<0x2000>>>>,  // 0x8000-0x9FFF
}

impl GPU {
    pub fn new() -> GPU {
        GPU { buf: Rc::new(RefCell::new(Box::new(BlockRam::new()))) }
    }
}

const OFFSET: u16 = 0x8000;

impl Memory for GPU {
    fn get(&self, i: u16) -> u8 {
        self.buf.borrow().get(i - OFFSET)
    }

    fn set(&mut self, i: u16, v: u8) {
        self.buf.borrow_mut().set(i - OFFSET, v)
    }
}