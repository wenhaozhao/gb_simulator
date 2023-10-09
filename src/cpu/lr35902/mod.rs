use std::cell::RefCell;

use crate::cpu::CPU;
use crate::cpu::lr35902::opcodes::get_opcode;
use crate::cpu::lr35902::registers::Registers;
use crate::mmu::Memory;

mod registers;
mod opcode;
mod opcodes;

pub struct LR35902 {
    register: Registers,
    memory: RefCell<Box<dyn Memory>>,
}

impl LR35902 {
    fn exe(&mut self) {
        let addr = 0x0000u16;
        let opcode = get_opcode(addr).expect(format!("Unsupported opcode {}", addr).as_str());
    }
}

impl CPU for LR35902 {}