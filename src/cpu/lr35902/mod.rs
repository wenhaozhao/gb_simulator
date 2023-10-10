use std::cell::RefCell;

use crate::cpu::CPU;
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

    fn imm_u8(&mut self) -> u8{
        let addr = self.register.pc_get_and_incr();
        let val = self.memory.borrow().get(addr);
        val
    }

    fn imm_u16(&mut self) -> u16{
        let addr = self.register.pc_get_and_decr_by(0x02);
        let vec = self.memory.borrow().gets(addr, 0x02);
        let mut bytes=[00u8;2];
        bytes.copy_from_slice(&vec);
        u16::from_le_bytes(bytes)
    }

    fn exec(&mut self) {
        use opcode::OPCODE_PREFIX_CB as PREFIX_CB;
        let opcode_addr = self.imm_u8() as u16;
        let actual_opcode_addr = if opcode_addr ==  PREFIX_CB {
            PREFIX_CB|( self.imm_u8() as u16)
        } else {
            opcode_addr
        };
        let opcode = opcodes::get_opcode(actual_opcode_addr)
            .expect(format!("Unsupported opcode {:04X}", actual_opcode_addr).as_str());
        opcode.exec(self);
    }

}

impl CPU for LR35902 {}