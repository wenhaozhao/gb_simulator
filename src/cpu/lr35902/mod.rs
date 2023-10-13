use std::cell::RefCell;
use std::ops::Sub;
use std::rc::Rc;

use opcode::OPCODE_PREFIX_CB as PREFIX_CB;

use crate::cpu::{CPU, CPUInfo};
use crate::cpu::lr35902::clock::Clock;
use crate::cpu::lr35902::opcode::Opcode;
use crate::cpu::lr35902::opcodes::OPCODES;
use crate::cpu::lr35902::registers::Registers;
use crate::mmu::Memory;

mod registers;
mod opcode;
mod opcodes;
mod clock;

/// cpu frequency 4MHz
/// cycles per seconds
const FREQ: u64 = 0x0040_0000;


pub struct LR35902 {
    info: CPUInfo,
    clock: Clock,
    opcodes: &'static [Option<&'static dyn Opcode>; 0x0200],
    register: Registers,
    memory: Rc<RefCell<Box<dyn Memory>>>,
}

impl LR35902 {
    pub fn new(memory: Rc<RefCell<Box<dyn Memory>>>) -> Self {
        LR35902 {
            info: CPUInfo::new(FREQ),
            clock: Clock::new(),
            opcodes: &OPCODES,
            register: Registers::new(),
            memory, //RefCell::new(Box::new(crate::mmu::tests::TestMemory::new())),// RefCell<Box<dyn Memory>>
        }
    }
}

impl LR35902 {
    fn imm_u8(&mut self) -> u8 {
        let addr = self.register.pc_get_and_incr();
        let val = self.memory.borrow().get(addr);
        val
    }

    fn imm_u16(&mut self) -> u16 {
        let addr = self.register.pc_get_and_decr_by(0x02);
        let vec = self.memory.borrow().gets(addr, 0x02);
        let mut bytes = [00u8; 2];
        bytes.copy_from_slice(&vec);
        u16::from_le_bytes(bytes)
    }

    fn actual_run(&mut self, actual_opcode_addr: u16) {
        let opcode = opcodes::get_opcode(actual_opcode_addr)
            .expect(format!("Unsupported opcode {:04X}", actual_opcode_addr).as_str());
        let cycles = opcode.exec(self);
        if let Err(message) = self.clock.step(cycles) { eprintln!("{}", message); }
    }
}

impl CPU for LR35902 {
    fn run(&mut self) {
        let opcode_addr = self.imm_u8() as u16;
        let actual_opcode_addr = if opcode_addr == PREFIX_CB {
            PREFIX_CB | (self.imm_u8() as u16)
        } else {
            opcode_addr
        };
        self.actual_run(actual_opcode_addr);
    }

    fn info(&self) -> &CPUInfo {
        &self.info
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::cpu::lr35902::LR35902;
    use crate::mmu::Memory;

    #[test]
    pub fn test_actual_run() {
        let mem_ref: Rc<RefCell<Box<dyn Memory>>> = Rc::new(RefCell::new(Box::new(crate::mmu::tests::TestMemory::new())));
        let mut cpu = LR35902::new(Rc::clone(&mem_ref));
        let actual_opcode_addr = 0x0020u16;
        let mut count = 100000;
        loop {
            cpu.actual_run(actual_opcode_addr);
            count -= 1;
            if count <= 0 {
                return;
            }
        }
    }
}