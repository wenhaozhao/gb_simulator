use crate::io_device::IO_ADDR_SVBK;
use crate::mmu::{Memory, RAM};

const BANK_SIZE: usize = 0x10;
const RAM_CAPACITY: usize = 0x1000;

/// ## WorkRam
/// - C000	CFFF	4 KiB Work RAM (WRAM)
/// - D000	DFFF	4 KiB Work RAM (WRAM)	In CGB mode, switchable bank 1~7
/// - FF80	FFFE	High RAM (HRAM)
/// - FF70  FF70    [SVBK](IO_ADDR_SVBK)	WRAM bank	R/W	CGB
pub struct WorkRam {
    ram_bank: u8,
    rams: Vec<RAM<RAM_CAPACITY>>, // 0xC000-0xDFFF, 0xFF80-0xFFFE
}

impl WorkRam {
    pub fn new() -> WorkRam {
        WorkRam {
            ram_bank: 0x01u8,
            rams: vec![crate::mmu::new_ram::<RAM_CAPACITY>(); BANK_SIZE],
        }
    }
}

impl Memory for WorkRam {
    fn get(&self, addr: u16) -> u8 {
        match addr {
            0xC000..=0xCFFF => self.rams[0].borrow()[addr as usize - 0xC000],
            0xD000..=0xDFFF => self.rams[self.ram_bank as usize].borrow()[addr as usize - 0xD000],
            0xFF80..=0xFFFE => self.rams[BANK_SIZE - 1].borrow()[addr as usize - 0xFF80],
            IO_ADDR_SVBK => self.ram_bank,
            addr => panic!("WorkRam access denied, addr: 0x{:04X}", addr),
        }
    }

    fn set(&mut self, addr: u16, v: u8) {
        match addr {
            0xC000..=0xCFFF => self.rams[0].borrow_mut()[addr as usize - 0xC000] = v,
            0xD000..=0xDFFF => self.rams[self.ram_bank as usize].borrow_mut()[addr as usize - 0xD000] = v,
            0xFF80..=0xFFFE => self.rams[BANK_SIZE - 1].borrow_mut()[addr as usize - 0xFF80] = v,
            IO_ADDR_SVBK => {
                self.ram_bank = match v & 0x07 {
                    0 => 1,
                    v => v
                }
            }
            addr => panic!("WorkRam access denied, addr: 0x{:04X}", addr),
        }
    }
}

