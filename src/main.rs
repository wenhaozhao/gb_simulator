use std::io::Read;

// 获取游戏卡带的标题
fn rom_name(rom: &[u8]) -> String {
    let mut buf = String::new();
    let ic = 0x0134;
    // 根据是否支持 CGB 功能, 标题有不同的长度
    let oc = if rom[0x0143] == 0x80 { 0x013e } else { 0x0143 };
    for i in ic..oc {
        match rom[i] {
            0 => break,
            v => buf.push(v as char),
        }
    }
    buf
}

// 验证标题校验和
fn check_checksum(rom: &[u8]) -> bool {
    let mut v: u8 = 0;
    for i in 0x0134..0x014d {
        let b = rom[i];
        println!("{:02X}", b);
        v = v.wrapping_sub(b).wrapping_sub(1);
    }
    rom[0x014d] == v
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = std::fs::File::open("resources/cartridge/boxes.gb")?;
    let mut rom = Vec::new();
    f.read_to_end(&mut rom).unwrap();
    assert!(rom.len() >= 0x0150);
    assert!(check_checksum(&rom[..]));

    let rom_name = rom_name(&rom[..]);
    println!("{}", rom_name); // BOXES
    Ok(())
}
