use crate::cartridge::{Ram, Rom};
use crate::cartridge::mbc::{BankMode, MBC};
use crate::memory::Memory;
use crate::Result;

pub struct MBC1 {
    rom: Rom,
    ram: Ram,
    ///
    /// - BankMode   RAMBank   ROMBank
    /// - 1 bit      2 bit     5 bit
    ///
    bank: u8,
    ram_enable: bool,
}

impl MBC1 {
    pub fn new(rom_path: String, ram_path: String) -> Result<Self> {
        Ok(MBC1 {
            rom: Rom::new(rom_path)?,
            ram: Ram::new(ram_path)?,
            bank: 0x01,
            ram_enable: false,
        })
    }

    fn bank_mode(&self) -> BankMode {
        let bank = self.bank;
        match bank & 0b1000_0000 {
            0b1000_0000 => {
                BankMode::RAM
            }
            _ => BankMode::ROM,
        }
    }

    fn rom_bank_index(&self) -> u16 {
        let index = match self.bank_mode() {
            BankMode::ROM => self.bank & 0b0_11_11111,
            BankMode::RAM => self.bank & 0b0_00_11111,
        };
        index as u16
    }

    fn ram_bank_index(&self) -> u16 {
        let index = match self.bank_mode() {
            BankMode::ROM => 0x00,
            BankMode::RAM => self.bank & 0b0_11_00000 >> 5,
        };
        index as u16
    }
}


/// rom 16KB
const ROM_BANK_SIZE: u16 = 0x4000;
/// rom_0 0x0000 - 0x3FFF
const ROM_0_BASE: u16 = 0x0000;
/// rom_0 0x0000 - 0x3FFF
const ROM_0_END: u16 = ROM_0_BASE + ROM_BANK_SIZE - 1;
/// rom_01-7F 0x4000 - 0x7FFF
const ROM_X_BASE: u16 = 0x4000;
/// rom_01-7F 0x4000 - 0x7FFF
const ROM_X_END: u16 = ROM_X_BASE + ROM_BANK_SIZE - 1;

/// ram 8KB
const RAM_BANK_SIZE: u16 = 0x2000;
/// ram_00-03 0xA000 - 0xBFFF
const RAM_X_BASE: u16 = 0xA000;
/// ram_00-03 0xA000 - 0xBFFF
const RAM_X_END: u16 = RAM_X_BASE + RAM_BANK_SIZE - 1;

impl Memory for MBC1 {
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
                if self.ram_enable {
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
                // RAM 启用/禁用标志
                let after = (value & 0x0A) == 0x0A;
                if !after && !(after && self.ram_enable) {
                    // ram access: enable -> disable
                    let _ = self.ram.dump();
                }
                self.ram_enable = after;
            }
            0x2000..=0x3FFF => {
                // Bank Number 第 0-4 位
                let value = value & 0b0_00_11111;
                let value = match value {
                    0x00 | 0x20 | 0x40 | 0x60 => value | 0x01,
                    _ => value
                };
                self.bank = (self.bank & 0b1_11_00000) | value;
            }
            0x4000..=0x5FFF => {
                // Bank Number 第 5-6 位
                let value = (value & 0b0000_0011) << 5;
                self.bank = (self.bank & 0b1_00_11111) | value;
            }
            0x6000..=0x7FFF => {
                // Bank 模式选择
                let value = (value & 0b0000_0001) << 7;
                self.bank = self.bank | value;
            }
            RAM_X_BASE..=RAM_X_END => {
                if self.ram_enable {
                    let addr = self.ram_bank_index() * RAM_BANK_SIZE + addr - ROM_X_BASE;
                    self.ram.write(addr, value)
                }
            }
            _ => panic!("write addr {} denied", addr),
        }
    }
}

impl MBC for MBC1 {}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        todo!()
    }
}

