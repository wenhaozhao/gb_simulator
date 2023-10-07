use std::fmt::format;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::memory::Memory;
use crate::Result;

pub mod rom_only;
pub mod mbc;

pub struct Rom {
    mem: Vec<u8>,
}

impl Rom {
    pub fn new(path: String) -> Result<Self> {
        let mut rom_file = File::open(path).map_err(|err| format!("{:?}", err))?;
        let mut mem = Vec::new();
        rom_file.read_to_end(&mut mem).map_err(|err| format!("{:?}", err))?;
        Ok(Rom { mem })
    }
}

impl Memory for Rom {
    fn read(&self, i: u16) -> u8 {
        self.mem[i as usize]
    }

    fn write(&mut self, _: u16, _: u8) {
        // ignore on rom only
    }
}

pub struct Ram {
    path: String,
    mem: Vec<u8>,
}

impl Ram {
    pub fn new(path: String) -> Result<Self> {
        let mut rom_file = File::open(&path).map_err(|err| format!("{:?}", err))?;
        let mut mem = Vec::new();
        rom_file.read_to_end(&mut mem).map_err(|err| format!("{:?}", err))?;
        Ok(Ram { path, mem })
    }
}

impl Memory for Ram {
    fn read(&self, i: u16) -> u8 {
        self.mem[i as usize]
    }

    fn write(&mut self, i: u16, v: u8) {
        self.mem[i as usize] = v;
    }
}


pub trait Cartridge {}


pub fn power_up(path: &Path) -> Box<dyn Cartridge> {
    todo!()
}

#[cfg(test)]
mod cartridge_tests {}
