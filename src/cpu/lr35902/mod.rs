use std::cell::RefCell;
use std::time::Duration;

use opcode::OPCODE_PREFIX_CB as PREFIX_CB;

use crate::cpu::CPU;
use crate::cpu::lr35902::opcode::Opcode;
use crate::cpu::lr35902::registers::Registers;
use crate::mmu::Memory;

mod registers;
mod opcode;
mod opcodes;

pub struct LR35902 {
    opcodes: &'static [Option<&'static dyn Opcode>; 0x0200],
    register: Registers,
    memory: RefCell<Box<dyn Memory>>,
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
}

const CLOCK_FREQUENCY: u64 = 0x0040_0000;

// 4MHz
impl LR35902 {
    fn sleep(&self, duration: Duration) {}
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
}

impl LR35902 {
    fn actual_run(&mut self, actual_opcode_addr: u16) {
        let opcode = opcodes::get_opcode(actual_opcode_addr)
            .expect(format!("Unsupported opcode {:04X}", actual_opcode_addr).as_str());
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::cpu::lr35902::{LR35902, opcodes};
    use crate::cpu::lr35902::opcodes::OPCODES;
    use crate::cpu::lr35902::registers::Registers;
    use crate::mmu::Memory;

    #[test]
    pub fn test_actual_run() {
        let mut cpu = LR35902 {
            opcodes: &OPCODES,
            register: Registers::default(),
            memory: RefCell::new(Box::new(Memory::TestMemory::new())),// RefCell<Box<dyn Memory>>
        };
        let actual_opcode_addr = 0x0020u16;

        let opcode = opcodes::get_opcode(actual_opcode_addr)
            .expect(format!("Unsupported opcode {:04X}", actual_opcode_addr).as_str());
        let cycles = opcode_exec!(opcode, &cpu);
    }
}