use std::cell::RefCell;
use std::rc::Rc;

use crate::gpu::GPU;
use crate::interrupt::IER;
use crate::io::IO;
use crate::mmu::cartridge::{Cartridge, CartType};
use crate::mmu::prohibited_mem::ProhibitedMem;
use crate::mmu::work_ram::WorkRam;

mod work_ram;
mod prohibited_mem;
mod cartridge;

pub trait Memory {
    fn get(&self, i: u16) -> u8;

    fn get_u8(&self, i: u16) -> u8 {
        self.get(i)
    }

    fn get_u16(&self, i: u16) -> u16 {
        let vec = self.gets(i, 2);
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(&vec);
        u16::from_le_bytes(bytes)
    }

    fn gets(&self, i: u16, size: u16) -> Vec<u8> {
        let mut vec = Vec::with_capacity(size as usize);
        for j in 0..size {
            vec.push(self.get(i + j));
        }
        vec
    }

    fn set(&mut self, i: u16, v: u8);

    fn set_u8(&mut self, i: u16, v: u8) {
        self.set(i, v);
    }

    fn set_u16(&mut self, i: u16, val: u16) {
        let bytes = val.to_le_bytes();
        let vec = bytes.to_vec();
        self.sets(i, &vec);
    }

    fn sets(&mut self, i: u16, bytes: &Vec<u8>) {
        let mut i = i;
        for v in bytes {
            self.set(i, *v);
            i += 1;
        }
    }
}

pub struct MMU {
    cart: Cartridge,
    gpu: GPU,
    wram: WorkRam,
    prohibited: ProhibitedMem,
    io: IO,
    ier: IER, // interrupt enable register
}

impl MMU {
    pub fn new(cart: CartType) -> Rc<RefCell<Box<dyn Memory>>> {
        Rc::new(RefCell::new(Box::new(MMU {
            cart: Cartridge::new(cart),
            gpu: GPU::new(),
            wram: WorkRam::new(),
            prohibited: ProhibitedMem::new(),
            io: IO::new(),
            ier: IER::new(),
        })))
    }
}

pub const MMU_ADDR_IER: u16 = 0xFFFF;
pub const MMU_ADDR_IFR: u16 = 0xFF0F;

impl Memory for MMU {
    fn get(&self, addr: u16) -> u8 {
        let memory: &dyn Memory = match addr {
            0x0000..=0x7FFF => &self.cart,
            0x8000..=0x9FFF => &self.gpu,
            0xA000..=0xBFFF => &self.cart,
            0xC000..=0xDFFF => &self.wram,
            0xE000..=0xFDFF => &self.prohibited,
            0xFE00..=0xFE9F => &self.gpu,
            0xFEA0..=0xFEFF => &self.prohibited,
            0xFF00..=0xFF7F => &self.io,
            0xFF80..=0xFFFE => &self.wram,
            MMU_ADDR_IER => &self.ier,
            addr => panic!("MMU access denied, addr: 0x{:04X}", addr),
        };
        memory.get(addr)
    }

    fn set(&mut self, i: u16, v: u8) {
        let memory: &mut dyn Memory = match i {
            0x0000..=0x7FFF => &mut self.cart,
            0x8000..=0x9FFF => &mut self.gpu,
            0xA000..=0xBFFF => &mut self.cart,
            0xC000..=0xDFFF => &mut self.wram,
            0xE000..=0xFDFF => &mut self.prohibited,
            0xFE00..=0xFE9F => &mut self.gpu,
            0xFEA0..=0xFEFF => &mut self.prohibited,
            0xFF00..=0xFF7F => &mut self.io,
            0xFF80..=0xFFFE => &mut self.wram,
            0xFFFF..=0xFFFF => &mut self.ier,
            addr => panic!("MMU access denied, addr: 0x{:04X}", addr),
        };
        memory.set(i, v)
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