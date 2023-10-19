use std::cell::RefCell;
use std::rc::Rc;

use crate::io_device::cartridge::RefCartridge;
use crate::io_device::interrupt::RefInterrupt;
use crate::io_device::joypad::RefJoypad;
use crate::io_device::video::Video;
use crate::io_device::wram::WorkRam;

pub type RefMemory = Rc<RefCell<Box<dyn Memory>>>;

pub trait Memory {
    fn get(&self, addr: u16) -> u8;

    fn get_u8(&self, addr: u16) -> u8 {
        self.get(addr)
    }

    fn get_u16(&self, addr: u16) -> u16 {
        let vec = self.gets(addr, 2);
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(&vec);
        u16::from_le_bytes(bytes)
    }

    fn gets(&self, addr: u16, size: u16) -> Vec<u8> {
        let mut vec = Vec::with_capacity(size as usize);
        for j in 0..size {
            vec.push(self.get(addr + j));
        }
        vec
    }

    fn set(&mut self, addr: u16, v: u8);

    fn set_u8(&mut self, addr: u16, v: u8) {
        self.set(addr, v);
    }

    fn set_u16(&mut self, addr: u16, val: u16) {
        let bytes = val.to_le_bytes();
        let vec = bytes.to_vec();
        self.sets(addr, &vec);
    }

    fn sets(&mut self, addr: u16, bytes: &Vec<u8>) {
        let mut i = addr;
        for v in bytes {
            self.set(i, *v);
            i += 1;
        }
    }
}

pub type RAM<const CAP: usize> = Rc<RefCell<Box<Vec<u8>>>>;

pub fn new_ram<const CAP: usize>() -> RAM<CAP> where {
    Rc::new(RefCell::new(Box::new(vec![0u8; CAP])))
}

pub struct MMU {
    cart: RefCartridge,
    video: Video,
    work_ram: WorkRam,
    prohibited: Prohibited,
    interrupt: RefInterrupt,
    joypad: RefJoypad,
}

impl MMU {
    pub fn new(
        cart: RefCartridge,
        interrupt: RefInterrupt,
        joypad: RefJoypad,
    ) -> RefMemory {
        Rc::new(RefCell::new(Box::new(MMU {
            cart: cart,
            video: Video::new(),
            work_ram: WorkRam::new(),
            prohibited: Prohibited::new(),
            interrupt,
            joypad,
        })))
    }
}

impl Memory for MMU {
    fn get(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.cart.borrow().get(addr),
            0x8000..=0x9FFF => self.video.get(addr),
            0xA000..=0xBFFF => self.cart.borrow().get(addr),
            0xC000..=0xDFFF => self.work_ram.get(addr),
            0xE000..=0xFDFF => self.prohibited.get(addr),
            0xFE00..=0xFE9F => self.video.get(addr),
            0xFEA0..=0xFEFF => self.prohibited.get(addr),
            0xFF00 => self.joypad.borrow().get(addr),
            0xFF01 => todo!(), /// SB Serial transfer data R/W All
            0xFF02 => todo!(), /// SC Serial transfer control R/W Mixed
            0xFF04 => todo!(), /// DIV Divider register R/W All
            0xFF05 => todo!(), /// TIMA Timer counter R/W All
            0xFF06 => todo!(), /// TMA Timer modulo R/W All
            0xFF07 => todo!(), /// TAC Timer control R/W All
            0xFF0F => self.interrupt.borrow().get(addr),
            0xFF10 => todo!(), /// NR10 Sound channel 1 sweep R/W All
            0xFF11 => todo!(), /// NR11 Sound channel 1 length timer & duty cycle Mixed All
            0xFF12 => todo!(), /// NR12 Sound channel 1 volume & envelope R/W All
            0xFF13 => todo!(), /// NR13 Sound channel 1 period low W All
            0xFF14 => todo!(), /// NR14 Sound channel 1 period high & control Mixed All
            0xFF16 => todo!(), /// NR21 Sound channel 2 length timer & duty cycle Mixed All
            0xFF17 => todo!(), /// NR22 Sound channel 2 volume & envelope R/W All
            0xFF18 => todo!(), /// NR23 Sound channel 2 period low W All
            0xFF19 => todo!(), /// NR24 Sound channel 2 period high & control Mixed All
            0xFF1A => todo!(), /// NR30 Sound channel 3 DAC enable R/W All
            0xFF1B => todo!(), /// NR31 Sound channel 3 length timer W All
            0xFF1C => todo!(), /// NR32 Sound channel 3 output level R/W All
            0xFF1D => todo!(), /// NR33 Sound channel 3 period low W All
            0xFF1E => todo!(), /// NR34 Sound channel 3 period high & control Mixed All
            0xFF20 => todo!(), /// NR41 Sound channel 4 length timer W All
            0xFF21 => todo!(), /// NR42 Sound channel 4 volume & envelope R/W All
            0xFF22 => todo!(), /// NR43 Sound channel 4 frequency & randomness R/W All
            0xFF23 => todo!(), /// NR44 Sound channel 4 control Mixed All
            0xFF24 => todo!(), /// NR50 Master volume & VIN panning R/W All
            0xFF25 => todo!(), /// NR51 Sound panning R/W All
            0xFF26 => todo!(), /// NR52 Sound on/off Mixed All
            0xFF30..=0xFF3F => todo!(), /// Wave RAM Storage for one of the sound channels’ waveform R/W All
            0xFF40 => todo!(), /// LCDC LCD control R/W All
            0xFF41 => todo!(), /// STAT LCD status Mixed All
            0xFF42 => todo!(), /// SCY Viewport Y position R/W All
            0xFF43 => todo!(), /// SCX Viewport X position R/W All
            0xFF44 => todo!(), /// LY LCD Y coordinate R All
            0xFF45 => todo!(), /// LYC LY compare R/W All
            0xFF46 => todo!(), /// DMA OAM DMA source address & start R/W All
            0xFF47 => todo!(), /// BGP BG palette data R/W DMG
            0xFF48 => todo!(), /// OBP0 OBJ palette 0 data R/W DMG
            0xFF49 => todo!(), /// OBP1 OBJ palette 1 data R/W DMG
            0xFF4A => todo!(), /// WY Window Y position R/W All
            0xFF4B => todo!(), /// WX Window X position plus 7 R/W All
            0xFF4D => todo!(), /// KEY1 Prepare speed switch Mixed CGB
            0xFF4F => self.video.get(addr), /// VBK VRAM bank R/W CGB
            0xFF51 => todo!(), /// HDMA1 VRAM DMA source high W CGB
            0xFF52 => todo!(), /// HDMA2 VRAM DMA source low W CGB
            0xFF53 => todo!(), /// HDMA3 VRAM DMA destination high W CGB
            0xFF54 => todo!(), /// HDMA4 VRAM DMA destination low W CGB
            0xFF55 => todo!(), /// HDMA5 VRAM DMA length/mode/start R/W CGB
            0xFF56 => todo!(), /// RP Infrared communications port Mixed CGB
            0xFF68 => todo!(), /// BCPS/BGPI Background color palette specification / Background palette index R/W CGB
            0xFF69 => todo!(), /// BCPD/BGPD Background color palette data / Background palette data R/W CGB
            0xFF6A => todo!(), /// OCPS/OBPI OBJ color palette specification / OBJ palette index R/W CGB
            0xFF6B => todo!(), /// OCPD/OBPD OBJ color palette data / OBJ palette data R/W CGB
            0xFF6C => todo!(), /// OPRI Object priority mode R/W CGB
            0xFF70 => self.work_ram.get(addr), /// SVBK WRAM bank R/W CGB
            0xFF76 => todo!(), /// PCM12 Audio digital outputs 1 & 2 R CGB
            0xFF77 => todo!(), /// PCM34 Audio digital outputs 3 & 4 R CGB
            0xFF80..=0xFFFE => self.work_ram.get(addr),
            0xFFFF => self.interrupt.borrow().get(addr),
            addr => panic!("MMU access denied, addr: 0x{:04X}", addr),
        }
    }

    fn set(&mut self, addr: u16, v: u8) {
        match addr {
            0x0000..=0x7FFF => self.cart.borrow_mut().set(addr, v),
            0x8000..=0x9FFF => self.video.set(addr, v),
            0xA000..=0xBFFF => self.cart.borrow_mut().set(addr, v),
            0xC000..=0xDFFF => self.work_ram.set(addr, v),
            0xE000..=0xFDFF => self.prohibited.set(addr, v),
            0xFE00..=0xFE9F => self.video.set(addr, v),
            0xFEA0..=0xFEFF => self.prohibited.set(addr, v),
            0xFF00 => self.joypad.borrow_mut().set(addr, v),
            0xFF01 => todo!(), /// SB Serial transfer data R/W All
            0xFF02 => todo!(), /// SC Serial transfer control R/W Mixed
            0xFF04 => todo!(), /// DIV Divider register R/W All
            0xFF05 => todo!(), /// TIMA Timer counter R/W All
            0xFF06 => todo!(), /// TMA Timer modulo R/W All
            0xFF07 => todo!(), /// TAC Timer control R/W All
            0xFF0F => self.interrupt.borrow_mut().set(addr, v),
            0xFF10 => todo!(), /// NR10 Sound channel 1 sweep R/W All
            0xFF11 => todo!(), /// NR11 Sound channel 1 length timer & duty cycle Mixed All
            0xFF12 => todo!(), /// NR12 Sound channel 1 volume & envelope R/W All
            0xFF13 => todo!(), /// NR13 Sound channel 1 period low W All
            0xFF14 => todo!(), /// NR14 Sound channel 1 period high & control Mixed All
            0xFF16 => todo!(), /// NR21 Sound channel 2 length timer & duty cycle Mixed All
            0xFF17 => todo!(), /// NR22 Sound channel 2 volume & envelope R/W All
            0xFF18 => todo!(), /// NR23 Sound channel 2 period low W All
            0xFF19 => todo!(), /// NR24 Sound channel 2 period high & control Mixed All
            0xFF1A => todo!(), /// NR30 Sound channel 3 DAC enable R/W All
            0xFF1B => todo!(), /// NR31 Sound channel 3 length timer W All
            0xFF1C => todo!(), /// NR32 Sound channel 3 output level R/W All
            0xFF1D => todo!(), /// NR33 Sound channel 3 period low W All
            0xFF1E => todo!(), /// NR34 Sound channel 3 period high & control Mixed All
            0xFF20 => todo!(), /// NR41 Sound channel 4 length timer W All
            0xFF21 => todo!(), /// NR42 Sound channel 4 volume & envelope R/W All
            0xFF22 => todo!(), /// NR43 Sound channel 4 frequency & randomness R/W All
            0xFF23 => todo!(), /// NR44 Sound channel 4 control Mixed All
            0xFF24 => todo!(), /// NR50 Master volume & VIN panning R/W All
            0xFF25 => todo!(), /// NR51 Sound panning R/W All
            0xFF26 => todo!(), /// NR52 Sound on/off Mixed All
            0xFF30..=0xFF3F => todo!(), /// Wave RAM Storage for one of the sound channels’ waveform R/W All
            0xFF40 => todo!(), /// LCDC LCD control R/W All
            0xFF41 => todo!(), /// STAT LCD status Mixed All
            0xFF42 => todo!(), /// SCY Viewport Y position R/W All
            0xFF43 => todo!(), /// SCX Viewport X position R/W All
            0xFF44 => todo!(), /// LY LCD Y coordinate R All
            0xFF45 => todo!(), /// LYC LY compare R/W All
            0xFF46 => todo!(), /// DMA OAM DMA source address & start R/W All
            0xFF47 => todo!(), /// BGP BG palette data R/W DMG
            0xFF48 => todo!(), /// OBP0 OBJ palette 0 data R/W DMG
            0xFF49 => todo!(), /// OBP1 OBJ palette 1 data R/W DMG
            0xFF4A => todo!(), /// WY Window Y position R/W All
            0xFF4B => todo!(), /// WX Window X position plus 7 R/W All
            0xFF4D => todo!(), /// KEY1 Prepare speed switch Mixed CGB
            0xFF4F => self.video.set(addr, v), /// VBK VRAM bank R/W CGB
            0xFF51 => todo!(), /// HDMA1 VRAM DMA source high W CGB
            0xFF52 => todo!(), /// HDMA2 VRAM DMA source low W CGB
            0xFF53 => todo!(), /// HDMA3 VRAM DMA destination high W CGB
            0xFF54 => todo!(), /// HDMA4 VRAM DMA destination low W CGB
            0xFF55 => todo!(), /// HDMA5 VRAM DMA length/mode/start R/W CGB
            0xFF56 => todo!(), /// RP Infrared communications port Mixed CGB
            0xFF68 => todo!(), /// BCPS/BGPI Background color palette specification / Background palette index R/W CGB
            0xFF69 => todo!(), /// BCPD/BGPD Background color palette data / Background palette data R/W CGB
            0xFF6A => todo!(), /// OCPS/OBPI OBJ color palette specification / OBJ palette index R/W CGB
            0xFF6B => todo!(), /// OCPD/OBPD OBJ color palette data / OBJ palette data R/W CGB
            0xFF6C => todo!(), /// OPRI Object priority mode R/W CGB
            0xFF70 => self.work_ram.set(addr, v), /// SVBK WRAM bank R/W CGB
            0xFF76 => todo!(), /// PCM12 Audio digital outputs 1 & 2 R CGB
            0xFF77 => todo!(), /// PCM34 Audio digital outputs 3 & 4 R CGB
            0xFF80..=0xFFFE => self.work_ram.set(addr, v),
            0xFFFF => self.interrupt.borrow_mut().set(addr, v),
            addr => panic!("MMU access denied, addr: 0x{:04X}", addr),
        };
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BlockRam<const SIZE: usize> {
    bytes: [u8; SIZE],
}

impl<const SIZE: usize> BlockRam<SIZE> {
    pub fn new() -> BlockRam<SIZE> {
        BlockRam {
            bytes: [0u8; SIZE]
        }
    }
}

impl<const SIZE: usize> Memory for BlockRam<SIZE> {
    fn get(&self, i: u16) -> u8 {
        self.bytes[i as usize]
    }

    fn set(&mut self, i: u16, v: u8) {
        self.bytes[i as usize] = v;
    }
}

/// ## ProhibitedMem
/// - E000	FDFF	Mirror of C000~DDFF (ECHO RAM)	Nintendo says use of this area is prohibited.
/// - FEA0	FEFF	Not Usable	Nintendo says use of this area is prohibited
pub struct Prohibited {}

impl Prohibited {
    pub fn new() -> Prohibited {
        Prohibited {}
    }
}

impl Memory for Prohibited {
    fn get(&self, i: u16) -> u8 {
        panic!("Prohibited access denied, addr: 0x{:04X}", i);
    }

    fn set(&mut self, i: u16, _v: u8) {
        panic!("Prohibited access denied, addr: 0x{:04X}", i);
    }
}

#[cfg(test)]
pub mod tests {
    use crate::mmu::Memory;

    pub struct TestMemory {
        mem: [u8; 0xFFFF],
    }

    impl TestMemory {
        pub fn new() -> Self {
            Self {
                mem: [0u8; 0xFFFF],
            }
        }
    }

    impl Memory for TestMemory {
        fn get(&self, i: u16) -> u8 {
            self.mem[i as usize]
        }

        fn set(&mut self, i: u16, v: u8) {
            self.mem[i as usize] = v;
        }
    }
}