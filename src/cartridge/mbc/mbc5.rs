use std::str::FromStr;

use crate::cartridge::{Ram, Rom};
use crate::cartridge::mbc::{MBC, RAM_BANK_SIZE, RAM_X_BASE, RAM_X_END, ROM_0_BASE, ROM_0_END, ROM_BANK_SIZE, ROM_X_BASE, ROM_X_END};
use crate::memory::Memory;
use crate::Result;

pub struct MBC5 {
    rom: Rom,
    ram: Ram,
    rom_bank: u16,
    ram_bank: u16,
    ram_enable: bool,
}

impl MBC5 {
    pub fn new(rom_path: String, ram_path: String) -> Result<Self> {
        Ok(MBC5 {
            rom: Rom::new(rom_path)?,
            ram: Ram::new(ram_path)?,
            rom_bank: 0x0001,
            ram_bank: 0x0000,
            ram_enable: false,
        })
    }

    fn rom_bank_index(&self) -> u16 {
        self.rom_bank & 0x01FF
    }

    fn ram_bank_index(&self) -> u16 {
        self.ram_bank & 0x000F
    }
}

impl Memory for MBC5 {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            ROM_0_BASE..=ROM_0_END => {
                self.rom.read(addr)
            }
            ROM_X_BASE..=ROM_X_END => {
                let addr = self.rom_bank_index() * ROM_BANK_SIZE + addr - ROM_X_BASE;
                self.rom.read(addr)
            }
            RAM_X_BASE..=RAM_X_END => {
                // ram or rtc
                if self.ram_enable {
                    let index = self.ram_bank_index();
                    // ram
                    let addr = self.ram_bank_index() * RAM_BANK_SIZE + addr - RAM_X_BASE;
                    self.ram.read(addr)
                } else {
                    0x00
                }
            }
            _ => panic!("read addr {} denied", addr),
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => {
                // RAM/RTC 启用/禁用标志
                let after = (value & 0x0A) == 0x0A;
                if !after && !(after && self.ram_enable) {
                    // ram access: enable -> disable
                    let _ = self.ram.dump();
                }
                self.ram_enable = after;
            }
            0x2000..=0x2FFF => {
                // Rom Bank Number 0-7
                let value = value as u16;
                self.rom_bank = (self.rom_bank & 0xFE00) | value;
            }
            0x3000..=0x3FFF => {
                // Rom Bank Number 8
                let value = ((value & 0x01) as u16) << 8;
                self.rom_bank = (self.rom_bank & 0xFE00) | value;
            }
            0x4000..=0x5FFF => {
                // Ram/RTC Bank Number
                let value = (value & 0x0F) as u16;
                self.ram_bank = (self.ram_bank & 0xFFF0) | value;
            }
            RAM_X_BASE..=RAM_X_END => {
                if self.ram_enable {
                    let addr = self.ram_bank_index() * RAM_BANK_SIZE + addr - RAM_X_BASE;
                    self.ram.write(addr, value)
                }
            }
            _ => panic!("write addr {} denied", addr),
        }
    }
}

impl MBC for MBC5 {}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        todo!()
    }
}

