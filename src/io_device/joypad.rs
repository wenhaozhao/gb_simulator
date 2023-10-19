use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use minifb::Key;

use crate::io_device::interrupt::{Flag, Interrupt, RefInterrupt};
use crate::io_device::IO_ADDR_JOYPAD;
use crate::mmu::Memory;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum JoypadKey {
    Right = 0b0001_0001,
    Left = 0b0001_0010,
    Up = 0b0001_0100,
    Down = 0b0001_1000,
    A = 0b0010_0001,
    B = 0b0010_0010,
    Select = 0b0010_0100,
    Start = 0b0010_1000,
}

pub type RefJoypad = Rc<RefCell<Joypad>>;

/// ## [Gameboy手柄](https://gbdev.io/pandocs/Joypad_Input.html#ff00--p1joyp-joypad)
pub struct Joypad {
    key_mapper: HashMap<Key, JoypadKey>,
    interrupt: Rc<RefCell<Interrupt>>,
    matrix: u8,
}

impl Joypad {
    pub fn new(interrupt: RefInterrupt) -> RefJoypad {
        Rc::new(RefCell::new(Joypad {
            interrupt,
            matrix: 0xFFu8,
            key_mapper: [
                (Key::Home, JoypadKey::A), (Key::End, JoypadKey::A),
                (Key::PageUp, JoypadKey::B), (Key::PageDown, JoypadKey::B),
                (Key::Space, JoypadKey::Select),
                (Key::Enter, JoypadKey::Start),
                (Key::Right, JoypadKey::Right), (Key::D, JoypadKey::Right),
                (Key::Left, JoypadKey::Left), (Key::A, JoypadKey::Left),
                (Key::Up, JoypadKey::Up), (Key::W, JoypadKey::Up),
                (Key::Down, JoypadKey::Down), (Key::S, JoypadKey::Down),
            ].iter().cloned().collect(),
        }))
    }

    fn keydown(&mut self, key: Key) {
        if let Some(key) = self.key_mapper.get(&key) {
            self.matrix &= !(*key as u8);
            self.interrupt.borrow_mut().toggle(Flag::Joypad);
        }
    }

    fn keyup(&mut self, key: Key) {
        if let Some(key) = self.key_mapper.get(&key) {
            self.matrix |= (*key as u8);
        }
    }
}

impl Memory for Joypad {
    fn get(&self, addr: u16) -> u8 {
        match addr {
            IO_ADDR_JOYPAD => self.matrix,
            addr => panic!("Joypad access denied, addr: 0x{:04X}", addr),
        }
    }

    fn set(&mut self, addr: u16, v: u8) {
        match addr {
            IO_ADDR_JOYPAD => self.matrix = v,
            addr => panic!("Joypad access denied, addr: 0x{:04X}", addr),
        }
    }
}

impl minifb::InputCallback for Joypad {
    fn add_char(&mut self, _uni_char: u32) {}

    fn set_key_state(&mut self, key: Key, state: bool) {
        if state {
            self.keydown(key);
        } else {
            self.keyup(key);
        }
    }
}

