use crate::io_device::cartridge::{Cartridge, Ram, Rom};
use crate::io_device::cartridge::mbc::{MBC, RAM_BANK_LEN, RAM_X_BASE, RAM_X_END, ROM_0_BASE, ROM_0_END, ROM_BANK_LEN, ROM_X_BASE, ROM_X_END};
use crate::mmu::Memory;
use crate::Result;

pub const CART_TYPE_MBC1: u8 = 0x01;
pub const CART_TYPE_MBC1_RAM: u8 = 0x02;
pub const CART_TYPE_MBC1_RAM_BATTERY: u8 = 0x03;

/// 4个ram
const RAM_BANK_COUNT: u8 = 0b0_11_00000 >> 5 - 0x00 + 1;

enum BankMode {
    ROM,
    RAM,
}

pub struct MBC1 {
    rom: Rom,
    ram: Ram,
    ///
    /// - BankMode   RAMBank   ROMBank
    /// - 1 bit      2 bit     5 bit
    /// -            4个ram
    ///
    bank: u8,
    ram_enable: bool,
}

impl MBC1 {
    pub fn power_up(rom: Rom, ram_path: String) -> Result<Box<dyn Cartridge>> {
        Ok(Box::new(MBC1::new(rom, ram_path)?))
    }

    fn new(rom: Rom, ram_path: String) -> Result<Self> {
        Ok(MBC1 {
            rom,
            ram: Ram::new(ram_path, RAM_BANK_LEN as usize * RAM_BANK_COUNT as usize, |len| vec![0u8; len])?,
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

impl Cartridge for MBC1 {
    fn rom(&self) -> &Rom {
        &self.rom
    }

    fn ram(&self) -> Option<&Ram> {
        Some(&self.ram)
    }
}

impl Memory for MBC1 {
    fn get(&self, addr: u16) -> u8 {
        match addr {
            ROM_0_BASE..=ROM_0_END => {
                self.rom.get(addr)
            }
            ROM_X_BASE..=ROM_X_END => {
                let addr = self.rom_bank_index() * ROM_BANK_LEN + addr - ROM_X_BASE;
                self.rom.get(addr)
            }
            RAM_X_BASE..=RAM_X_END => {
                if self.ram_enable {
                    let addr = self.ram_bank_index() * RAM_BANK_LEN + addr - RAM_X_BASE;
                    self.ram.get(addr)
                } else {
                    0x00
                }
            }
            _ => panic!("read addr 0x{:04X} denied", addr),
        }
    }

    fn set(&mut self, addr: u16, value: u8) {
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
                    let addr = self.ram_bank_index() * RAM_BANK_LEN + addr - RAM_X_BASE;
                    self.ram.set(addr, value)
                }
            }
            _ => panic!("write addr 0x{:04X} denied", addr),
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

