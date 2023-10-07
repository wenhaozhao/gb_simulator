use crate::cartridge::{Ram, Rom};
use crate::cartridge::mbc::{BankMode, MBC};
use crate::memory::Memory;
use crate::Result;

pub struct MBC2 {
    rom: Rom,
    ram: Ram,
    bank: u8,
    ram_enable: bool,
}

impl MBC2 {
    pub fn new(rom_path: String, ram_path: String) -> Result<Self> {
        Ok(MBC2 {
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
        let index = self.bank & 0x0F;
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
/// rom_01-0F 0x4000 - 0x7FFF
const ROM_X_BASE: u16 = 0x4000;
/// rom_01-0F 0x4000 - 0x7FFF
const ROM_X_END: u16 = ROM_X_BASE + ROM_BANK_SIZE - 1;

/// ram 512 * 4 Bits
const RAM_BANK_SIZE: u16 = 0x200;
/// ram 0xA000 - 0xA1FF
const RAM_X_BASE: u16 = 0xA000;
/// ram 0xA000 - 0xA1FF
const RAM_X_END: u16 = RAM_X_BASE + RAM_BANK_SIZE - 1;

impl Memory for MBC2 {
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
                    let v = self.rom.read(adr - RAM_X_BASE);
                    v & 0x0F // 由于数据由 4 Bits 组成, 因此该存储区中只有每个字节的低 4 位才会被使用
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
                // 只有高位地址字节的最低有效位为零才能启用/关闭 RAM
                if offset & 0x0100 == 0x0000 {
                    let after = (value & 0x0A) == 0x0A;
                    if !after && !(after && self.ram_enable) {
                        // ram access: enable -> disable
                        self.ram.dump();
                    }
                    self.ram_enable = after;
                }
            }
            0x2000..=0x3FFF => {
                // Bank Number 第 0-3 位
                // 只有写入地址的高位字节的最低有效位为 1 才能正确写入
                if offset & 0x0100 == 0x0100 {
                    // 数值的低 4 位将作为当前的 ROM Bank Number
                    let value = value & 0x0F;
                    self.bank = (self.bank & 0xF0) | value;
                }
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

impl MBC for MBC2 {}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        todo!()
    }
}