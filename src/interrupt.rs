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

pub struct IE {
    data: u8,
}

impl IE {
    pub fn new() -> IE {
        IE { data: 0x00 }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        (self.data & (0x01u8 << (flag as u8))) > 0
    }
}

impl Memory for IE {
    fn get(&self, i: u16) -> u8 {
        self.data
    }

    fn set(&mut self, i: u16, v: u8) {
        self.data = v
    }
}

pub struct IF {
    data: u8,
}

impl IF {
    pub fn new() -> IF {
        IF { data: 0x00 }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        (self.data & (0x01u8 << (flag as u8))) > 0
    }
}

impl Memory for IF {
    fn get(&self, i: u16) -> u8 {
        self.data
    }

    fn set(&mut self, i: u16, v: u8) {
        self.data = v
    }
}
