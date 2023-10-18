use std::cell::RefCell;
use std::rc::Rc;

use crate::cpu::{CPU, CPUInfo, MemType};
use crate::cpu::lr35902::clock::Clock;
use crate::cpu::lr35902::registers::{Register, Registers};
use crate::GBTerm;
use crate::mmu::Memory;

mod registers;
mod opcode;
mod clock;

/// cpu frequency 4MHz
/// cycles per seconds
const FREQ: u64 = 0x0040_0000;


pub struct LR35902 {
    info: CPUInfo,
    clock: Clock,
    register: Registers,
    memory: MemType,
    stack: Vec<u16>,
    halted: bool,
    enable_interrupt: bool,
}

impl LR35902 {
    pub fn new(gb_term: GBTerm, memory: MemType) -> Rc<RefCell<Box<dyn CPU>>> {
        Rc::new(RefCell::new(Box::new(LR35902 {
            info: CPUInfo::new(FREQ),
            clock: Clock::new(),
            register: Registers::new(gb_term),
            memory, //RefCell::new(Box::new(crate::mmu::tests::TestMemory::new())),// RefCell<Box<dyn Memory>>
            stack: Vec::new(),
            halted: false,
            enable_interrupt: true,
        })))
    }
}

impl LR35902 {
    fn imm_u8(&mut self) -> u8 {
        let addr = self.register.get_and_incr_u16(Register::PC);
        let val = self.memory.borrow().get(addr);
        #[cfg(test)]
        println!("imm_u8: 0x{:04X} => 0x{:04X}", addr, val);
        val
    }

    fn imm_u16(&mut self) -> u16 {
        let addr = self.register.get_and_incr_by_u16(Register::PC, 0x02);
        let vec = self.memory.borrow().gets(addr, 0x02);
        let mut bytes = [00u8; 2];
        bytes.copy_from_slice(&vec);
        u16::from_le_bytes(bytes)
    }

    fn actual_run(&mut self, opcode: u16) {
        #[cfg(test)]
        println!("opcode: 0x{:04X}", opcode);
        let cycles = if self.halted {
            0x04u8
        } else {
            if opcode == opcode::CB_PREFIXED {
                self.cbprefixed_exec_opcode(opcode as u8)
            } else {
                self.unprefixed_exec_opcode(opcode as u8)
            }
        };
        if let Err(message) = self.clock.step(cycles) { eprintln!("{}", message); }
    }
}


impl CPU for LR35902 {
    fn memory(&self) -> MemType {
        Rc::clone(&self.memory)
    }

    fn run(&mut self) {
        let opcode = self.imm_u8() as u16;
        self.actual_run(opcode);
    }

    fn info(&self) -> &CPUInfo {
        &self.info
    }
}

#[cfg(test)]
mod tests {}