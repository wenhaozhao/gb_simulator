use std::cell::RefCell;
use std::rc::Rc;

use crate::mmu::{BlockRam, Memory};

const RAMS_SIZE: usize = 0x10usize;

/// ## WorkRam
/// - C000	CFFF	4 KiB Work RAM (WRAM)
/// - D000	DFFF	4 KiB Work RAM (WRAM)	In CGB mode, switchable bank 1~7
/// - FF80	FFFE	High RAM (HRAM)
pub struct WorkRam {
    rams: Vec<Rc<RefCell<Box<BlockRam<0x1000>>>>>, // 0xC000-0xDFFF, 0xFF80-0xFFFE
}

impl WorkRam {
    pub fn new() -> WorkRam {
        WorkRam {
            rams: vec![Rc::new(RefCell::new(Box::new(BlockRam::new()))); RAMS_SIZE],
        }
    }

    /// ram_index
    /// return (index, offset)
    fn ram_index(i: u16) -> (usize, u16) {
        match i {
            0xC000..=0xCFFF => (0, 0xC000),
            0xD000..=0xDFFF => {
                //todo switch bank...
                (0, 0xD000)
            }
            0xFF80..=0xFFFE => (RAMS_SIZE - 1, 0xFF80), // high ram
            addr => panic!("WorkRam access denied, addr: 0x{:04X}", addr),
        }
    }
}

impl Memory for WorkRam {
    fn get(&self, i: u16) -> u8 {
        let (ram_i, offset) = Self::ram_index(i);
        self.rams[ram_i].borrow().get(i - offset)
    }

    fn set(&mut self, i: u16, v: u8) {
        let (ram_i, offset) = Self::ram_index(i);
        self.rams[ram_i].borrow_mut().set(i - offset, v);
    }
}

