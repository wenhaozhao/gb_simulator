use std::cell::RefCell;
use std::rc::Rc;

use crate::mmu::Memory;

pub type CartType = Rc<RefCell<Box<dyn crate::cartridge::Cartridge>>>;

/// ## Cartridge
/// - 0000	3FFF	16 KiB ROM bank 00	From cartridge, usually a fixed bank
/// - 4000	7FFF	16 KiB ROM Bank 01~NN	From cartridge, switchable bank via mapper (if any)
/// - A000	BFFF	8 KiB External RAM	From cartridge, switchable bank if any
pub struct Cartridge {
    cart: CartType,
}

impl Cartridge {
    pub fn new(cart: CartType) -> Cartridge {
        Cartridge { cart }
    }
}

impl Memory for Cartridge {
    fn get(&self, i: u16) -> u8 {
        self.cart.borrow().get(i)
    }

    fn set(&mut self, i: u16, v: u8) {
        self.cart.borrow_mut().set(i, v)
    }
}