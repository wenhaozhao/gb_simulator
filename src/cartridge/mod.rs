use std::fmt::Formatter;
use std::fs::File;
use std::io::{Read, Write};

use crate::cartridge::mbc::mbc1::{CART_TYPE_MBC1, CART_TYPE_MBC1_RAM, CART_TYPE_MBC1_RAM_BATTERY, MBC1};
use crate::cartridge::mbc::mbc2::{CART_TYPE_MBC2, CART_TYPE_MBC2_BATTERY, MBC2};
use crate::cartridge::mbc::mbc3::{CART_TYPE_MBC3, CART_TYPE_MBC3_RAM_2, CART_TYPE_MBC3_RAM_BATTERY_2, CART_TYPE_MBC3_TIMER_BATTERY, CART_TYPE_MBC3_TIMER_RAM_BATTERY_2, MBC3};
use crate::cartridge::mbc::mbc5::{CART_TYPE_MBC5, CART_TYPE_MBC5_RAM, CART_TYPE_MBC5_RAM_BATTERY, CART_TYPE_MBC5_RUMBLE, CART_TYPE_MBC5_RUMBLE_RAM, CART_TYPE_MBC5_RUMBLE_RAM_BATTERY, MBC5};
use crate::mmu::Memory;
use crate::Result;

mod mbc;

/// HEADER 0x0100 - 0x014F
const HEADER_BASE: u16 = 0x0100;
/// HEADER 0x0100 - 0x014F
const HEADER_END: u16 = 0x014F;
/// HEADER 0x0100 - 0x014F
/// len: 0x50(80)
const HEADER_LEN: u16 = HEADER_END - HEADER_BASE + 1;

/// 程序执行入口 0x0100 - 0x0103
const HEADER_ENTRY_BASE: u16 = 0x0100;
/// 程序执行入口 0x0100 - 0x0103
const HEADER_ENTRY_END: u16 = 0x0103;

/// 任天堂logo 0x0104 - 0x0133
const HEADER_NINTENDO_LOGO_BASE: u16 = 0x0104;
/// 任天堂logo 0x0104 - 0x0133
const HEADER_NINTENDO_LOGO_END: u16 = 0x0133;
/// 任天堂logo 0x0104 - 0x0133
/// len: 0x30(48)
const HEADER_NINTENDO_LOGO_LEN: u16 = HEADER_NINTENDO_LOGO_END - HEADER_NINTENDO_LOGO_BASE + 1;

/// 任天堂logo 0x0104 - 0x0133
/// bitmap
const NINTENDO_LOGO: [u8; HEADER_NINTENDO_LOGO_LEN as usize] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11,
    0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E,
    0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

/// 标题 0x0134 - 0x0143
const HEADER_TITLE_BASE: u16 = 0x0134;
/// GameBoy
/// 标题 0x0134 - 0x0142
const HEADER_TITLE_END_GB: u16 = 0x0142;
const HEADER_TITLE_GB_LEN: u16 = HEADER_TITLE_END_GB - HEADER_TITLE_BASE + 1;
/// ColorGameBoy(CGB) 标志 0143
const HEADER_CGB_FLAG: u16 = 0x0143;
const CGB_FLAG_COMPATIBLE: u8 = 0x80;
const CGB_FLAG_ONLY: u8 = 0xC0;

#[derive(derive_more::Display)]
pub enum CGBFlag {
    None,
    Compatible,
    CGBOnly,
}

/// 制造商编码 0x013F - 0x0142
/// enable on cgb
const HEADER_MANUFACTURER_CODE_BASE: u16 = 0x013F;
/// 制造商编码 0x013F - 0x0142
/// enable on cgb
const HEADER_MANUFACTURER_CODE_END: u16 = 0x0142;
const HEADER_MANUFACTURER_CODE_LEN: u16 = HEADER_MANUFACTURER_CODE_END - HEADER_MANUFACTURER_CODE_BASE + 1;
/// ColorGameBoy
/// 标题 0x0134 - 0x013E
const HEADER_TITLE_END_CGB: u16 = 0x013E;
const HEADER_TITLE_CGB_LEN: u16 = HEADER_TITLE_END_CGB - HEADER_TITLE_BASE + 1;

/// 许可协议代码 0x0144 - 0x0145
const HEADER_NEW_LICENSEE_CODE_BASE: u16 = 0x0144;
/// 许可协议代码 0x0144 - 0x0145
const HEADER_NEW_LICENSEE_CODE_END: u16 = 0x0145;
const HEADER_NEW_LICENSEE_CODE_LEN: u16 = HEADER_NEW_LICENSEE_CODE_END - HEADER_NEW_LICENSEE_CODE_BASE + 1;

/// SuperGameBoy(SGB) 标志 0x0146
const HEADER_SGB_FLAG: u16 = 0x0146;

#[derive(derive_more::Display)]
pub enum SGBFlag {
    None,
    Ok,
}

/// 卡带(Cartridge)类型 0x0147
const HEADER_CART_TYPE: u16 = 0x0147;

/// ROM SIZE 0x0148
const HEADER_ROM_SIZE: u16 = 0x0148;

/// RAM SIZE 0x0149
const HEADER_RAM_SIZE: u16 = 0x0149;

/// 发售地代码 0x014A
const HEADER_DESTINATION_CODE: u16 = 0x014A;

/// (旧)许可协议代码 0x014B
const HEADER_OLD_LICENSEE_CODE: u16 = 0x014B;

/// Mask ROM version number 0x14C
const HEADER_MASK_ROM_VERSION_NUM: u16 = 0x14C;

/// header checksum 0x134 - 0x14C
const HEADER_CHECK_BASE: u16 = 0x0134;
/// header checksum 0x134 - 0x14C
const HEADER_CHECK_END: u16 = 0x14C;
const HEADER_CHECK_LEN: u16 = HEADER_CHECK_END - HEADER_CHECK_BASE + 1;
/// header checksum 0x134 - 0x14C
/// 0x014D
const HEADER_CHECKSUM: u16 = 0x014D;

/// 全局校验和 0x014E - 0x014F
const HEADER_GLOBAL_CHECKSUM_BASE: u16 = 0x014E;
/// 全局校验和 0x014E - 0x014F
const HEADER_GLOBAL_CHECKSUM_END: u16 = 0x014F;
const HEADER_GLOBAL_CHECKSUM_LEN: u16 = HEADER_GLOBAL_CHECKSUM_END - HEADER_GLOBAL_CHECKSUM_BASE + 1;


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
    fn get(&self, i: u16) -> u8 {
        self.mem[i as usize]
    }

    fn set(&mut self, _: u16, _: u8) {
        // ignore on rom only
    }
}

pub struct Ram {
    path: String,
    mem: Vec<u8>,
}

impl Ram {
    pub fn new<F>(path: String, size: usize, content_supplier: F) -> Result<Self>
        where F: FnOnce(usize) -> Vec<u8>
    {
        let mut ram_file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => {
                let mut file = File::create(&path).map_err(|err| format!("{:?}", err))?;
                let empty_content = content_supplier(size);
                let _ = file.write_all(&empty_content).map_err(|err| format!("{:?}", err))?;
                File::open(&path).map_err(|err| format!("{:?}", err))?
            }
        };
        let mut mem = Vec::new();
        ram_file.read_to_end(&mut mem).map_err(|err| format!("{:?}", err))?;
        Ok(Ram { path, mem })
    }

    pub fn from(mem: Vec<u8>, path: String) -> Self {
        Ram { path, mem }
    }

    pub fn dump(&self) -> Result<()> {
        let mut rom_file = File::create(&self.path).map_err(|err| format!("{:?}", err))?;
        rom_file.write_all(&self.mem).map_err(|err| format!("{:?}", err))?;
        Ok(())
    }

    pub fn mem(&self) -> &Vec<u8> {
        &self.mem
    }
}

impl Memory for Ram {
    fn get(&self, i: u16) -> u8 {
        self.mem[i as usize]
    }

    fn set(&mut self, i: u16, v: u8) {
        self.mem[i as usize] = v;
    }
}

const CART_TYPE_ROM_ONLY: u8 = 0x00;

struct RomOnly {
    rom: Rom,
}

impl RomOnly {
    pub fn power_up(rom: Rom) -> Result<Box<dyn Cartridge>> {
        Ok(Box::new(RomOnly::new(rom)?))
    }

    fn new(rom: Rom) -> Result<Self> {
        Ok(RomOnly { rom })
    }
}

impl Cartridge for RomOnly {
    fn rom(&self) -> &Rom {
        &self.rom
    }

    fn ram(&self) -> Option<&Ram> {
        None
    }
}

impl Memory for RomOnly {
    fn get(&self, i: u16) -> u8 {
        self.rom.get(i)
    }

    fn set(&mut self, i: u16, v: u8) {
        self.rom.set(i, v)
    }
}

pub trait Cartridge: Memory {
    fn rom(&self) -> &Rom;
    fn ram(&self) -> Option<&Ram>;
    #[inline]
    fn header(&self) -> Vec<u8> {
        self.rom().gets(HEADER_BASE, HEADER_LEN)
    }
    #[inline]
    fn logo(&self) -> Vec<u8> {
        self.rom().gets(HEADER_NINTENDO_LOGO_BASE, HEADER_NINTENDO_LOGO_LEN)
    }

    #[inline]
    fn manufacturer_code(&self) -> Vec<u8> {
        self.rom().gets(HEADER_MANUFACTURER_CODE_BASE, HEADER_MANUFACTURER_CODE_LEN)
    }
    fn manufacturer_code_text(&self) -> Result<String> {
        String::from_utf8(self.manufacturer_code())
            .map_err(|err| format!("Create manufacturer code string error, msg: {:?}", err))
    }

    #[inline]
    fn cgb_flag(&self) -> CGBFlag {
        let flag = self.rom().get(HEADER_CGB_FLAG);
        match flag {
            CGB_FLAG_COMPATIBLE => CGBFlag::Compatible,
            CGB_FLAG_ONLY => CGBFlag::CGBOnly,
            _ => CGBFlag::None,
        }
    }

    #[inline]
    fn title_len(&self) -> u16 {
        let cgb_flag = self.cgb_flag();
        match cgb_flag {
            CGBFlag::Compatible | CGBFlag::CGBOnly => HEADER_TITLE_CGB_LEN,
            _ => HEADER_TITLE_GB_LEN,
        }
    }

    #[inline]
    fn title(&self) -> Vec<u8> {
        let title_len = self.title_len();
        let title_bytes = self.rom().gets(HEADER_TITLE_BASE, title_len);
        title_bytes
    }

    fn title_text(&self) -> Result<String> {
        let title_bytes = self.title();
        let title_text = String::from_utf8(
            title_bytes.iter()
                .filter(|b| **b > 0)
                .map(|b| *b)
                .collect::<Vec<u8>>()
        )
            .map_err(|err| format!("Create title string error, msg: {:?}", err))?;
        Ok(title_text)
    }

    fn licensee_code(&self) -> u16 {
        let old = self.rom().get(HEADER_OLD_LICENSEE_CODE);
        match old {
            0x33 => {
                let code_vec = self.rom().gets(HEADER_NEW_LICENSEE_CODE_BASE, HEADER_NEW_LICENSEE_CODE_LEN);
                let mut code = [0u8; 2];
                code.copy_from_slice(&code_vec);
                let code = u16::from_be_bytes(code);
                code
            }
            _ => old as u16
        }
    }

    #[inline]
    fn sgb_flag(&self) -> SGBFlag {
        match self.rom().get(HEADER_SGB_FLAG) {
            0x00 => SGBFlag::None,
            0x03 => SGBFlag::Ok,
            _ => SGBFlag::None
        }
    }

    #[inline]
    fn cart_type(&self) -> u8 {
        self.rom().get(HEADER_CART_TYPE)
    }

    fn cart_type_text(&self) -> String {
        String::from(match self.cart_type() {
            0x00 => "ROM ONLY",
            0x01 => "MBC1",
            0x02 => "MBC1+RAM",
            0x03 => "MBC1+RAM+BATTERY",
            0x05 => "MBC2",
            0x06 => "MBC2+BATTERY",
            0x08 => "ROM+RAM",
            0x09 => "ROM+RAM+BATTERY",
            0x0b => "MMM01",
            0x0c => "MMM01+RAM",
            0x0d => "MMM01+RAM+BATTERY",
            0x0f => "MBC3+TIMER+BATTERY",
            0x10 => "MBC3+TIMER+RAM+BATTERY",
            0x11 => "MBC3",
            0x12 => "MBC3+RAM",
            0x13 => "MBC3+RAM+BATTERY",
            0x15 => "MBC4",
            0x16 => "MBC4+RAM",
            0x17 => "MBC4+RAM+BATTERY",
            0x19 => "MBC5",
            0x1a => "MBC5+RAM",
            0x1b => "MBC5+RAM+BATTERY",
            0x1c => "MBC5+RUMBLE",
            0x1d => "MBC5+RUMBLE+RAM",
            0x1e => "MBC5+RUMBLE+RAM+BATTERY",
            0xfc => "POCKET CAMERA",
            0xfd => "BANDAI TAMA5",
            0xfe => "HuC3",
            0x1f => "HuC1+RAM+BATTERY",
            n => panic!("Unsupported cartridge type: 0x{:02x}", n),
        })
    }

    // Specifies the ROM Size of the cartridge. Typically calculated as "32KB shl N".
    fn rom_size(&self) -> usize {
        match self.rom().get(HEADER_ROM_SIZE) {
            0x00 => 0x4000 * 2,
            0x01 => 0x4000 * 4,
            0x02 => 0x4000 * 8,
            0x03 => 0x4000 * 16,
            0x04 => 0x4000 * 32,
            0x05 => 0x4000 * 64,
            0x06 => 0x4000 * 128,
            0x07 => 0x4000 * 256,
            0x08 => 0x4000 * 512,
            0x52 => 0x4000 * 72,
            0x53 => 0x4000 * 80,
            0x54 => 0x4000 * 96,
            n => panic!("Unsupported rom size: 0x{:02x}", n),
        }
    }

    // Specifies the size of the external RAM in the cartridge (if any).
    fn ram_size(&self) -> usize {
        match self.rom().get(HEADER_RAM_SIZE) {
            0x00 => 0,
            0x01 => 0x400 * 2,
            0x02 => 0x400 * 8,
            0x03 => 0x400 * 32,
            0x04 => 0x400 * 128,
            0x05 => 0x400 * 64,
            n => panic!("Unsupported ram size: 0x{:02x}", n),
        }
    }

    /// 发售地代码
    #[inline]
    fn destination_code(&self) -> u8 {
        self.rom().get(HEADER_DESTINATION_CODE)
    }

    /// 发售地代码
    fn destination_code_text(&self) -> String {
        String::from(match self.destination_code() {
            0x00 => "Japan",
            _ => "Overseas"
        })
    }

    #[inline]
    fn mask_rom_version_number(&self) -> u8 {
        self.rom().get(HEADER_MASK_ROM_VERSION_NUM)
    }

    #[inline]
    fn header_checksum(&self) -> u8 {
        self.rom().get(HEADER_CHECKSUM)
    }

    fn global_checksum(&self) -> u16 {
        let vec = self.rom().gets(HEADER_GLOBAL_CHECKSUM_BASE, HEADER_GLOBAL_CHECKSUM_LEN);
        let mut checksum = [0u8; 2];
        checksum.copy_from_slice(&vec);
        u16::from_be_bytes(checksum)
    }

    fn info(&self) -> CartridgeInfo {
        CartridgeInfo {
            cgb_flag: self.cgb_flag(),
            title: self.title_text().unwrap_or(String::from("Unknown")),
            licensee_code: self.licensee_code(),
            sgb_flag: self.sgb_flag(),
            cart_type: self.cart_type_text(),
            rom_size: self.rom_size(),
            ram_size: self.ram_size(),
            destination_code: self.destination_code_text(),
            mask_rom_version_number: self.mask_rom_version_number(),
            header_checksum: self.header_checksum(),
            global_checksum: self.global_checksum(),
        }
    }

    fn check_logo(&self) -> Result<()> {
        let logo = self.logo();
        for i in 0..HEADER_NINTENDO_LOGO_LEN {
            let v = logo[i as usize];
            let expect = NINTENDO_LOGO[i as usize];
            if v != expect {
                return Err(format!("Nintendo logo check err, expect: {:?}, but got: {:?}", NINTENDO_LOGO, logo));
            }
        }
        Ok(())
    }

    fn check_header(&self) -> Result<()> {
        let header_check = self.rom().gets(HEADER_CHECK_BASE, HEADER_CHECK_LEN);
        let mut checksum = 0u8;
        for b in &header_check {
            checksum = checksum.wrapping_sub(*b).wrapping_sub(1);
        }
        let expect = self.header_checksum();
        if checksum != expect {
            return Err(format!("Title checksum error, expect: {}, but got: {}", expect, checksum));
        }
        Ok(())
    }
}

pub struct CartridgeInfo {
    cgb_flag: CGBFlag,
    title: String,
    licensee_code: u16,
    sgb_flag: SGBFlag,
    cart_type: String,
    rom_size: usize,
    ram_size: usize,
    destination_code: String,
    mask_rom_version_number: u8,
    header_checksum: u8,
    global_checksum: u16,
}

impl core::fmt::Display for CartridgeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"
>>>>>>>> Cartridge Info <<<<<<<<
cgb_flag: {},
title: {},
licensee_code: {},
sgb_flag: {},
cart_type: {},
rom_size: {},
ram_size: {},
destination_code: {},
mask_rom_version_number: {},
header_checksum: {},
global_checksum: {}
>>>>>>>> Cartridge Info <<<<<<<<
"#,
               self.cgb_flag,
               self.title,
               self.licensee_code,
               self.sgb_flag,
               self.cart_type,
               self.rom_size,
               self.ram_size,
               self.destination_code,
               self.mask_rom_version_number,
               self.header_checksum,
               self.global_checksum,
        )
    }
}

pub fn power_up(rom_path: String, ram_path: String, rtc_path: String) -> Result<Box<dyn Cartridge>> {
    let rom = Rom::new(rom_path.clone())?;
    let cart_type = rom.get(HEADER_CART_TYPE);
    let cart = match cart_type {
        CART_TYPE_ROM_ONLY => RomOnly::power_up(rom),
        CART_TYPE_MBC1 | CART_TYPE_MBC1_RAM | CART_TYPE_MBC1_RAM_BATTERY => MBC1::power_up(rom, ram_path.clone()),
        CART_TYPE_MBC2 | CART_TYPE_MBC2_BATTERY => MBC2::power_up(rom, ram_path.clone()),
        CART_TYPE_MBC3_TIMER_BATTERY | CART_TYPE_MBC3_TIMER_RAM_BATTERY_2 | CART_TYPE_MBC3 | CART_TYPE_MBC3_RAM_2 | CART_TYPE_MBC3_RAM_BATTERY_2 => MBC3::power_up(rom, ram_path.clone(), rtc_path.clone()),
        CART_TYPE_MBC5 | CART_TYPE_MBC5_RAM | CART_TYPE_MBC5_RAM_BATTERY | CART_TYPE_MBC5_RUMBLE | CART_TYPE_MBC5_RUMBLE_RAM | CART_TYPE_MBC5_RUMBLE_RAM_BATTERY => MBC5::power_up(rom, ram_path.clone()),
        _ => Err(format!("Unsupported cartridge type: {}", cart_type))
    }?;
    let _ = cart.check_logo()?;
    let _ = cart.check_header()?;
    Ok(cart)
}

#[cfg(test)]
mod tests {
    use crate::cartridge::power_up;

    #[test]
    fn test_power_up() {
        let rom_path = String::from("resources/cartridge/boxes.gb");
        let ram_path = String::from("target/_ram");
        let rtc_path = String::from("target/_rtc");
        let cart = power_up(rom_path, ram_path, rtc_path).unwrap();
        let title_text = cart.title_text().unwrap();
        println!(" rom title_text => {}", title_text);
        assert_eq!(title_text, String::from("BOXES"));
        println!(" rom manufacturer_code => {:02X?}", cart.manufacturer_code());
        let info = cart.info();
        println!("{}", info);
    }
}
