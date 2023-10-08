use std::fs::File;
use std::io::{Read, Write};

use crate::cartridge::mbc::mbc1::{CART_TYPE_MBC1, CART_TYPE_MBC1_RAM, CART_TYPE_MBC1_RAM_BATTERY, MBC1};
use crate::cartridge::mbc::mbc2::{CART_TYPE_MBC2, CART_TYPE_MBC2_BATTERY, MBC2};
use crate::cartridge::mbc::mbc3::{CART_TYPE_MBC3, CART_TYPE_MBC3_RAM_2, CART_TYPE_MBC3_RAM_BATTERY_2, CART_TYPE_MBC3_TIMER_BATTERY, CART_TYPE_MBC3_TIMER_RAM_BATTERY_2, MBC3};
use crate::cartridge::mbc::mbc5::{CART_TYPE_MBC5, CART_TYPE_MBC5_RAM, CART_TYPE_MBC5_RAM_BATTERY, CART_TYPE_MBC5_RUMBLE, CART_TYPE_MBC5_RUMBLE_RAM, CART_TYPE_MBC5_RUMBLE_RAM_BATTERY, MBC5};
use crate::memory::Memory;
use crate::Result;

pub mod mbc;
mod cartridge_types;

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
/// 制造商编码 0x013F - 0x0142
/// enable on cgb
const HEADER_MANUFACTURER_CODE_BASE: u16 = 0x013F;
/// 制造商编码 0x013F - 0x0142
/// enable on cgb
const HEADER_MANUFACTURER_CODE_END: u16 = 0x0142;
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

    fn title(&self) -> Vec<u8> {
        rom_title(self.rom())
    }

    fn title_string(&self) -> Result<String> {
        let title = self.title();
        rom_title_string(&title)
    }
}

pub fn power_up(rom_path: String, ram_path: String, rtc_path: String) -> Result<Box<dyn Cartridge>> {
    let rom = Rom::new(rom_path.clone())?;
    let _ = check_nintendo_logo(&rom)?;
    let _ = check_header(&rom)?;
    let cart_type = rom.get(HEADER_CART_TYPE);
    let cart = match cart_type {
        CART_TYPE_ROM_ONLY => RomOnly::power_up(rom),
        CART_TYPE_MBC1 | CART_TYPE_MBC1_RAM | CART_TYPE_MBC1_RAM_BATTERY => MBC1::power_up(rom, ram_path.clone()),
        CART_TYPE_MBC2 | CART_TYPE_MBC2_BATTERY => MBC2::power_up(rom, ram_path.clone()),
        CART_TYPE_MBC3_TIMER_BATTERY | CART_TYPE_MBC3_TIMER_RAM_BATTERY_2 | CART_TYPE_MBC3 | CART_TYPE_MBC3_RAM_2 | CART_TYPE_MBC3_RAM_BATTERY_2 => MBC3::power_up(rom, ram_path.clone(), rtc_path.clone()),
        CART_TYPE_MBC5 | CART_TYPE_MBC5_RAM | CART_TYPE_MBC5_RAM_BATTERY | CART_TYPE_MBC5_RUMBLE | CART_TYPE_MBC5_RUMBLE_RAM | CART_TYPE_MBC5_RUMBLE_RAM_BATTERY => MBC5::power_up(rom, ram_path.clone()),
        _ => Err(format!("Unsupported cartridge type: {}", cart_type))
    };
    cart
}

fn check_nintendo_logo(rom: &Rom) -> Result<()> {
    let logo = rom.gets(HEADER_NINTENDO_LOGO_BASE, HEADER_NINTENDO_LOGO_LEN);
    for i in 0..HEADER_NINTENDO_LOGO_LEN {
        let v = logo[i as usize];
        let expect = NINTENDO_LOGO[i as usize];
        if v != expect {
            return Err(format!("Nintendo logo check err, expect: {:?}, but got: {:?}", NINTENDO_LOGO, logo));
        }
    }
    Ok(())
}

fn check_header(rom: &Rom) -> Result<()> {
    let header_check = rom.gets(HEADER_CHECK_BASE, HEADER_CHECK_LEN);
    let mut checksum = 0u8;
    for b in &header_check {
        checksum = checksum.wrapping_sub(*b).wrapping_sub(1);
    }
    let expect = rom.get(HEADER_CHECKSUM);
    if checksum != expect {
        return Err(format!("Title checksum error, expect: {}, but got: {}", expect, checksum));
    }
    Ok(())
}

fn rom_title(rom: &Rom) -> Vec<u8> {
    let cgb_flag = rom.get(HEADER_CGB_FLAG);
    let title_len = match cgb_flag {
        CGB_FLAG_COMPATIBLE | CGB_FLAG_ONLY => HEADER_TITLE_CGB_LEN,
        _ => HEADER_TITLE_GB_LEN,
    };
    let title_bytes = rom.gets(HEADER_TITLE_BASE, title_len);
    title_bytes
}

fn rom_title_string(title_bytes: &Vec<u8>) -> Result<String> {
    let title_string = String::from_utf8(
        title_bytes.iter()
            .filter(|b| **b > 0)
            .map(|b| *b)
            .collect::<Vec<u8>>()
    )
        .map_err(|err| format!("Create title string error, msg: {:?}", err))?;
    Ok(title_string)
}

#[cfg(test)]
mod tests {
    use crate::cartridge::{check_header, check_nintendo_logo, power_up, Rom, rom_title, rom_title_string};

    #[test]
    fn test_check_nintendo_logo() {
        let rom = Rom::new(String::from("resources/cartridge/boxes.gb")).unwrap();
        check_nintendo_logo(&rom).unwrap()
    }

    #[test]
    fn test_check_header() {
        let rom = Rom::new(String::from("resources/cartridge/boxes.gb")).unwrap();
        check_header(&rom).unwrap();
    }

    #[test]
    fn test_get_rom_title() {
        let rom = Rom::new(String::from("resources/cartridge/boxes.gb")).unwrap();
        let title_bytes = rom_title(&rom);
        println!(" rom title_bytes=>{:?}", title_bytes);
        let title_string = rom_title_string(&title_bytes).unwrap();
        println!(" rom title_string => {}", title_string);
        assert_eq!(title_string, String::from("BOXES"));
    }

    #[test]
    fn test_power_up() {
        let rom_path = String::from("resources/cartridge/boxes.gb");
        let ram_path = String::from("target/_ram");
        let rtc_path = String::from("target/_rtc");
        let cart = power_up(rom_path, ram_path, rtc_path).unwrap();
        let title_string = cart.title_string().unwrap();
        println!(" rom title_string => {}", title_string);
        assert_eq!(title_string, String::from("BOXES"));
    }
}
