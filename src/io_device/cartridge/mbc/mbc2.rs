use crate::io_device::cartridge::{Cartridge, Ram, Rom};
use crate::io_device::cartridge::mbc::{MBC, ROM_0_BASE, ROM_0_END, ROM_BANK_LEN, ROM_X_BASE, ROM_X_END};
use crate::mmu::Memory;
use crate::Result;

pub const CART_TYPE_MBC2: u8 = 0x05;
pub const CART_TYPE_MBC2_BATTERY: u8 = 0x06;

pub struct MBC2 {
    rom: Rom,
    /// ram 512 * 4 Bits
    ram: Ram,
    bank: u8,
    ram_enable: bool,
}

impl MBC2 {
    pub fn power_up(rom: Rom, ram_path: String) -> Result<Box<dyn Cartridge>> {
        Ok(Box::new(MBC2::new(rom, ram_path)?))
    }

    fn new(rom: Rom, ram_path: String) -> Result<Self> {
        Ok(MBC2 {
            rom,
            ram: Ram::new(ram_path, RAM_BANK_LEN as usize, |len| vec![0u8; len])?,
            bank: 0x01,
            ram_enable: false,
        })
    }

    fn rom_bank_index(&self) -> u16 {
        let index = self.bank & 0x0F;
        index as u16
    }
}

impl Cartridge for MBC2 {
    fn rom(&self) -> &Rom {
        &self.rom
    }

    fn ram(&self) -> Option<&Ram> {
        Some(&self.ram)
    }
}

/// ram 512 * 4 Bits
const RAM_BANK_LEN: u16 = 0x200;
/// ram 0xA000 - 0xA1FF
const RAM_X_BASE: u16 = 0xA000;
/// ram 0xA000 - 0xA1FF
const RAM_X_END: u16 = RAM_X_BASE + RAM_BANK_LEN - 1;

impl Memory for MBC2 {
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
                    let v = self.rom.get(addr - RAM_X_BASE);
                    v & 0x0F // 由于数据由 4 Bits 组成, 因此该存储区中只有每个字节的低 4 位才会被使用
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
                // 只有高位地址字节的最低有效位为零才能启用/关闭 RAM
                if addr & 0x0100 == 0x0000 {
                    let after = (value & 0x0A) == 0x0A;
                    if !after && !(after && self.ram_enable) {
                        // ram access: enable -> disable
                        let _ = self.ram.dump();
                    }
                    self.ram_enable = after;
                }
            }
            0x2000..=0x3FFF => {
                // Bank Number 第 0-3 位
                // 只有写入地址的高位字节的最低有效位为 1 才能正确写入
                if addr & 0x0100 == 0x0100 {
                    // 数值的低 4 位将作为当前的 ROM Bank Number
                    let value = value & 0x0F;
                    self.bank = (self.bank & 0xF0) | value;
                }
            }
            RAM_X_BASE..=RAM_X_END => {
                if self.ram_enable {
                    self.ram.set(addr - RAM_X_BASE, value)
                }
            }
            _ => panic!("write addr 0x{:04X} denied", addr),
        }
    }
}

impl MBC for MBC2 {}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        todo!()
    }
}