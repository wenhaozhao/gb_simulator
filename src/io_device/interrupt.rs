use crate::cpu::CPU;
use crate::mmu::Memory;

pub enum Flag {
    VBlank = 0x00,
    LCDStat = 0x01,
    Timer = 0x02,
    Serial = 0x03,
    Joypad = 0x04,
}

impl From<u8> for Flag {
    fn from(f: u8) -> Self {
        match f {
            0x00 => Flag::VBlank,
            0x01 => Flag::VBlank,
            0x02 => Flag::Timer,
            0x03 => Flag::Serial,
            0x04 => Flag::Joypad,
            f => panic!("Unknown interrupt flag: 0x{:04X}", f),
        }
    }
}

/// ## IE: InterruptEnableRegister
/// - FFFF	FFFF	Interrupt Enable register (IE)
/// - 7	6	5	4	    3	    2	    1	    0
/// -           Joypad  Serial  Timer   LCD     VBlank

pub struct IER {
    data: u8,
}

impl IER {
    pub fn new() -> IER {
        IER { data: 0x00 }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        (self.data & (0x01u8 << (flag as u8))) > 0
    }
}

impl Memory for IER {
    fn get(&self, i: u16) -> u8 {
        self.data
    }

    fn set(&mut self, i: u16, v: u8) {
        self.data = v
    }
}

pub struct IFR {
    data: u8,
}

impl IFR {
    pub fn new() -> IFR {
        IFR { data: 0x00 }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        (self.data & (0x01u8 << (flag as u8))) > 0
    }
}

impl Memory for IFR {
    fn get(&self, i: u16) -> u8 {
        self.data
    }

    fn set(&mut self, i: u16, v: u8) {
        self.data = v
    }
}


pub fn on_interrupt(cpu: &mut dyn CPU, flag: Flag, _ier: u8, _ifr: u8) {

    match flag {
        Flag::VBlank => {}
        Flag::LCDStat => {}
        Flag::Timer => {}
        Flag::Serial => {}
        Flag::Joypad => {}
    };
}