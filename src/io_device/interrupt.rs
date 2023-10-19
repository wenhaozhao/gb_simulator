use std::cell::RefCell;
use std::rc::Rc;

use crate::cpu::CPU;
use crate::io_device::{IO_ADDR_INT_E, IO_ADDR_INT_F};
use crate::mmu::{Memory, RefMemory};

/// ## Interrupt Flag
/// - 7	6	5	4	    3	    2	    1	    0
/// -           Joypad  Serial  Timer   LCD     VBlank
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
            0x01 => Flag::LCDStat,
            0x02 => Flag::Timer,
            0x03 => Flag::Serial,
            0x04 => Flag::Joypad,
            f => panic!("Unknown interrupt flag: 0x{:04X}", f),
        }
    }
}

type IntHandlerFunc = fn(RefMemory);

struct IDT {
    v_blank: Option<IntHandlerFunc>,
    lcd_stat: Option<IntHandlerFunc>,
    timer: Option<IntHandlerFunc>,
    serial: Option<IntHandlerFunc>,
    joypad: Option<IntHandlerFunc>,
}

//todo
/// ## [IntControl](https://gbdev.io/pandocs/Interrupts.html#interrupts)
/// - [Interrupt Flag](Flag)
/// - 0xFF0F    interrupt_enable
/// - 0xFFFE    interrupt_flag
/// ## int_enable: InterruptEnableRegister
pub struct Interrupt {
    /// interrupt_enable
    int_e: u8,
    /// interrupt_flag
    int_f: u8,
    idt: IDT,
}

pub type RefInterrupt = Rc<RefCell<Interrupt>>;


impl Interrupt {
    pub fn new() -> RefInterrupt {
        Rc::new(RefCell::new(Interrupt {
            int_e: 0,
            int_f: 0,
            idt: IDT { v_blank: None, lcd_stat: None, timer: None, serial: None, joypad: None },
        }))
    }

    pub fn set_handler_fn(&mut self, flag: Flag, handler: IntHandlerFunc) {
        match flag {
            Flag::VBlank => self.idt.v_blank = Some(handler),
            Flag::LCDStat => self.idt.lcd_stat = Some(handler),
            Flag::Timer => self.idt.timer = Some(handler),
            Flag::Serial => self.idt.serial = Some(handler),
            Flag::Joypad => self.idt.joypad = Some(handler),
        }
    }

    /// 触发中断标志
    pub fn toggle(&mut self, flag: Flag) {
        self.int_f |= (0x01 << (flag as u8));
    }

    /// 中断处理(IDT)
    pub fn handle(&mut self, mem: RefMemory, flag: Flag) {
        if let Some(handler) = match flag {
            Flag::VBlank => self.idt.v_blank,
            Flag::LCDStat => self.idt.lcd_stat,
            Flag::Timer => self.idt.timer,
            Flag::Serial => self.idt.serial,
            Flag::Joypad => self.idt.joypad,
        } {
            handler(Rc::clone(&mem))
        }
    }
}

impl Memory for Interrupt {
    fn get(&self, addr: u16) -> u8 {
        match addr {
            IO_ADDR_INT_F => self.int_f,
            IO_ADDR_INT_E => self.int_e,
            addr => panic!("Interrupt access denied, addr: 0x{:04X}", addr),
        }
    }

    fn set(&mut self, addr: u16, v: u8) {
        match addr {
            IO_ADDR_INT_F => self.int_f = v & 0b0001_1111,
            IO_ADDR_INT_E => self.int_e = v & 0b0001_1111,
            addr => panic!("Interrupt access denied, addr: 0x{:04X}", addr),
        }
    }
}
