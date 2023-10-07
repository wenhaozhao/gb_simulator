use crate::cartridge::Rom;
use crate::memory::Memory;
use crate::Result;

struct RomOnly {
    rom: Rom,
}

impl RomOnly {
    pub fn new(rom_path: String) -> Result<Self> {
        let rom = Rom::new(rom_path)?;
        Ok(RomOnly { rom })
    }
}

impl Memory for RomOnly {
    fn read(&self, i: u16) -> u8 {
        self.rom.read(i)
    }

    fn write(&mut self, i: u16, v: u8) {
        self.rom.write(i, v)
    }
}