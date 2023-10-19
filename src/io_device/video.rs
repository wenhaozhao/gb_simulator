use crate::io_device::IO_ADDR_VBK;
use crate::mmu::{Memory, RAM};

/// IO_ADDR_VBK
///
const BANK_SIZE: usize = 0x02;
const V_RAM_CAPACITY: usize = 0x1000;

const OAM_CAPACITY: usize = 0xA0;

/// ## GPU
/// - 8000	9FFF	8 KiB Video RAM (VRAM)	In CGB mode, switchable bank 0/1
/// - FE00	FE9F	[Object attribute memory (OAM)](https://gbdev.io/pandocs/OAM.html#object-attribute-memory-oam)
/// - FF4F	FF4F    [VBK](IO_ADDR_VBK)	    VRAM bank	R/W	CGB
///
pub struct Video {
    vram_bank: u8,
    vrams: Vec<RAM<V_RAM_CAPACITY>>,
    oam: RAM<OAM_CAPACITY>,
}

impl Video {
    pub fn new() -> Video {
        Video {
            vram_bank: 0x00,
            vrams: vec![crate::mmu::new_ram::<V_RAM_CAPACITY>(); BANK_SIZE],
            oam: crate::mmu::new_ram::<OAM_CAPACITY>(),
        }
    }
}

impl Memory for Video {
    fn get(&self, addr: u16) -> u8 {
        match addr {
            IO_ADDR_VBK => 0xFE | self.vram_bank,
            0x8000..=0x9FFF => self.vrams[self.vram_bank as usize].borrow()[addr as usize - 0x8000],
            0xFE00..=0xFE9F => self.oam.borrow()[addr as usize - 0xFE00],
            addr => panic!("Video access denied, addr: 0x{:04X}", addr),
        }
    }

    fn set(&mut self, addr: u16, v: u8) {
        match addr {
            IO_ADDR_VBK => self.vram_bank = v & 0x01,
            0x8000..=0x9FFF => self.vrams[self.vram_bank as usize].borrow_mut()[addr as usize - 0x8000] = v,
            0xFE00..=0xFE9F => self.oam.borrow_mut()[addr as usize - 0xFE00] = v,
            addr => panic!("Video access denied, addr: 0x{:04X}", addr),
        }
    }
}