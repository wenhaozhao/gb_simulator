use crate::io_device::interrupt::IFR;
use crate::mmu::Memory;

pub mod joypad;
pub mod serial;
pub mod divider;
pub mod interrupt;
pub mod video;
pub mod infrared;
pub mod wram;
pub mod cartridge;



/// ## [VBK](https://gbdev.io/pandocs/CGB_Registers.html#ff4f--vbk-cgb-mode-only-vram-bank)
/// - $FF4F	VBK	VRAM bank	R/W	CGB
pub const IO_ADDR_VBK: u16 = 0xFF4F;
/// ## [SVBK](https://gbdev.io/pandocs/CGB_Registers.html#ff70--svbk-cgb-mode-only-wram-bank)
/// - $FF70 SVBK	WRAM bank	R/W	CGB
pub const IO_ADDR_SVBK: u16 = 0xFF70;

/// ## IO
/// - FF00	FF7F	[I/O Registers](https://gbdev.io/pandocs/Memory_Map.html#io-ranges)
pub struct IOBus {
    ifr: IFR,
    buf: [u8; 0x0080],
}

impl IOBus {
    pub fn new() -> Self {
        IOBus { ifr: IFR::new(), buf: [0u8; 0x0080] }
    }
}

const OFFSET: usize = 0xFF00;

//todo 这是假的实现
impl Memory for IOBus {
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

