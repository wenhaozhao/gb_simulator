use std::str::FromStr;
use std::time::SystemTime;

use crate::cartridge::{Cartridge, Ram, Rom};
use crate::cartridge::mbc::{MBC, RAM_BANK_LEN, RAM_X_BASE, RAM_X_END, ROM_0_BASE, ROM_0_END, ROM_BANK_LEN, ROM_X_BASE, ROM_X_END};
use crate::memory::Memory;
use crate::Result;

pub const CART_TYPE_MBC3_TIMER_BATTERY: u8 = 0x0F;
pub const CART_TYPE_MBC3_TIMER_RAM_BATTERY_2: u8 = 0x10;
pub const CART_TYPE_MBC3: u8 = 0x11;
pub const CART_TYPE_MBC3_RAM_2: u8 = 0x12;
pub const CART_TYPE_MBC3_RAM_BATTERY_2: u8 = 0x13;

///
/// RealTimeClock, RTC
/// - 0x08  RTC S   Seconds   0-59 (0-3Bh)
/// - 0x09  RTC M   Minutes   0-59 (0-3Bh)
/// - 0x0A  RTC H   Hours     0-23 (0-17h)
/// - 0x0B  RTC DL  Lower 8 bits of Day Counter (0-FFh)
/// - 0x0C  RTC DH  Upper 1 bit of Day Counter, Carry Bit, Halt Flag
///     - Bit 0  Most significant bit of Day Counter (Bit 8)
///     - Bit 6  Halt (0=Active, 1=Stop Timer)
///     - Bit 7  Day Counter Carry Bit (1=Counter Overflow)
///
pub struct RTC {
    ram: Ram,
    // big endian
    t0: u64,
}

impl RTC {
    fn new(path: String) -> Result<Self> {
        let ram = Ram::new(path.clone(), 8, |len| {
            let t0 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
            let bytes = t0.to_be_bytes().to_vec();
            bytes
        })?;
        let mut bytes: [u8; 8] = Default::default();
        bytes.copy_from_slice(ram.mem());
        // big endian
        let t0 = u64::from_be_bytes(bytes);
        Ok(RTC { ram, t0 })
    }

    fn latched(&mut self) -> Result<()> {
        let t0 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        // big endian
        let bytes = t0.to_be_bytes().to_vec();
        self.ram.sets(0, &bytes);
        self.ram.dump()?;
        self.t0 = t0;
        Ok(())
    }
}

impl Memory for RTC {
    fn get(&self, reg: u16) -> u8 {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let t0 = self.t0;
        let elapse = now - t0;
        let v = match reg {
            0x08 => elapse % 60, // rtc seconds
            0x09 => elapse / 60 % 60, //rtc minutes
            0x0A => elapse / 3600 % 24, // rtc hours
            0x0B => (elapse / 3600 / 24), // rtc dl days
            0x0C => (elapse / 3600 / 24) & 0x1FF >> 8,
            _ => panic!("read rtc reg {} denied", reg),
        };
        (v & 0xFF) as u8
    }

    fn set(&mut self, _: u16, _: u8) {
        // ignore rtc write
    }
}

/// 4个ram
const RAM_BANK_COUNT: u8 = 0x03 - 0x00 + 1;

pub struct MBC3 {
    rom: Rom,
    ram: Ram,
    rtc: RTC,
    rom_bank: u8,
    /// 0x01 - 0x07
    ram_bank: u8,
    /// ram 0x00 - 0x03, rtc 0x08 - 0x0C
    ram_enable: bool,
}

impl MBC3 {
    pub fn power_up(rom: Rom, ram_path: String, rtc_path: String) -> Result<Box<dyn Cartridge>> {
        Ok(Box::new(MBC3::new(rom, ram_path, rtc_path)))
    }

    fn new(rom: Rom, ram_path: String, rtc_path: String) -> Result<Self> {
        Ok(MBC3 {
            rom,
            ram: Ram::new(ram_path, RAM_BANK_LEN * RAM_BANK_COUNT, |len| vec![0u8; len as usize])?,
            rtc: RTC::new(rtc_path)?,
            rom_bank: 0x01,
            ram_bank: 0x00,
            ram_enable: false,
        })
    }

    fn rom_bank_index(&self) -> u16 {
        let index = self.rom_bank & 0x7F;
        index as u16
    }

    fn ram_bank_index(&self) -> u16 {
        let index = self.ram_bank & 0x0F;
        index as u16
    }
}

impl Cartridge for MBC3 {
    fn rom(&self) -> &Rom {
        &self.rom
    }

    fn ram(&self) -> Option<&Ram> {
        Some(&self.ram)
    }
}

impl Memory for MBC3 {
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
                // ram or rtc
                if self.ram_enable {
                    let index = self.ram_bank_index();
                    match index {
                        0x00..=0x03 => {
                            // ram
                            let addr = index * RAM_BANK_LEN + addr - RAM_X_BASE;
                            self.ram.get(addr)
                        }
                        0x08..=0x0C => {
                            // rtc registers
                            self.rtc.get(index)
                        }
                        _ => 0x00,
                    }
                } else {
                    0x00
                }
            }
            _ => panic!("read addr {} denied", addr),
        }
    }

    fn set(&mut self, addr: u16, value: u8) {
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
            0x2000..=0x3FFF => {
                // Rom Bank Number
                let value = value & 0x7F;
                let value = match value {
                    0x00 => 0x01,
                    _ => value
                };
                self.rom_bank = (self.rom_bank & 0x80) | value;
            }
            0x4000..=0x5FFF => {
                // Ram/RTC Bank Number
                let value = value & 0x0F;
                self.ram_bank = (self.ram_bank & 0xF0) | value;
            }
            0x6000..=0x7FFF => {
                // 锁存时钟数据
                if value & 0x01 == 0x01 {
                    let _ = self.rtc.latched();
                }
            }
            RAM_X_BASE..=RAM_X_END => {
                // ram or rtc
                if self.ram_enable {
                    let index = self.ram_bank_index();
                    match index {
                        0x00..=0x03 => {
                            // ram
                            let addr = index * RAM_BANK_LEN + addr - RAM_X_BASE;
                            self.ram.set(addr, value)
                        }
                        0x08..=0x0C => {
                            // rtc registers
                            self.rtc.set(index, value)
                        }
                        _ => (),
                    }
                }
            }
            _ => panic!("write addr {} denied", addr),
        }
    }
}

impl MBC for MBC3 {}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        todo!()
    }
}

