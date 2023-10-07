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
const ROM_0_BASE: u16 = 0x0000;
const ROM_0_END: u16 = ROM_0_BASE + ROM_BANK_SIZE - 1;
// 0x3FFF
const ROM_X_BASE: u16 = 0x4000;
const ROM_X_END: u16 = ROM_X_BASE + ROM_BANK_SIZE - 1; // 0x7FFF

/// ram 8KB
const RAM_BANK_SIZE: u16 = 0x2000;
const RAM_X_BASE: u16 = 0xA000;
const RAM_X_END: u16 = RAM_X_BASE + RAM_BANK_SIZE - 1; // 0xBFFF

impl Memory for MBC1 {
    fn read(&self, adr: u16) -> u8 {
        match adr {
            ROM_0_BASE..=ROM_0_END => {
                self.rom.read(adr)
            }
            ROM_X_BASE..=ROM_X_END => {
                let adr = self.rom_bank_index() * ROM_BANK_SIZE + adr - ROM_X_BASE;
                self.rom.read(adr)
            }
            RAM_X_BASE..=RAM_X_END => {
                if self.ram_enable {
                    let adr = self.ram_bank_index() * RAM_BANK_SIZE + adr - RAM_X_BASE;
                    self.ram.read(adr)
                } else {
                    0x00
                }
            }
            _ => {
                panic!("read addr {} denied", adr)
            }
        }
    }

    fn write(&mut self, offset: u16, value: u8) {
        match offset {
            0x0000..=0x1FFF => {
                // RAM 启用/禁用标志
                self.ram_enable = (value & 0x0A) == 0x0A;
                // todo ram dump ?
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
                    let adr = self.ram_bank_index() * RAM_BANK_SIZE + offset - ROM_X_BASE;
                    self.ram.write(adr, value)
                }
            }
            _ => {
                panic!("write addr {} denied", offset)
            }
        }
    }
}

impl MBC for MBC1 {}

