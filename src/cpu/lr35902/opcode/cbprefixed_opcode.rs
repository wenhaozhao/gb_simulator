use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::registers::{Flag, Register};

static OPCODE_CYCLES: [[u8; 2]; 0x100] = [
    [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0],
    [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0],
    [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0],
    [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0],
    [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0],
    [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0],
    [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0],
    [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0],
    [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0],
    [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0],
    [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0],
    [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0],
    [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0],
    [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0],
    [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0],
    [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [16, 0], [8, 0],
];

impl LR35902 {
    pub fn cbprefixed_exec_opcode(&mut self, opcode: u8) -> u8 {
        match opcode {
            // RLC
            0x00..=0x07 => {
                match Self::get_reg(opcode) {
                    Register::HL => {
                        let addr = self.register.get_u16(Register::HL);
                        let val = self.memory.borrow().get_u8(addr);
                        let res = self.alu_rlc(val);
                        self.memory.borrow_mut().set(addr, res);
                    }
                    reg => {
                        let res = self.alu_rlc(self.register.get_u8(reg));
                        self.register.set_u8(reg, res);
                    }
                }
                OPCODE_CYCLES[opcode as usize][0]
            }
            // RRC
            0x08..=0x0F => {
                match Self::get_reg(opcode) {
                    Register::HL => {
                        let addr = self.register.get_u16(Register::HL);
                        let val = self.memory.borrow().get_u8(addr);
                        let res = self.alu_rrc(val);
                        self.memory.borrow_mut().set(addr, res);
                    }
                    reg => {
                        let res = self.alu_rrc(self.register.get_u8(reg));
                        self.register.set_u8(reg, res);
                    }
                }
                OPCODE_CYCLES[opcode as usize][0]
            }
            // RL
            0x10..=0x17 => {
                match Self::get_reg(opcode) {
                    Register::HL => {
                        let addr = self.register.get_u16(Register::HL);
                        let val = self.memory.borrow().get_u8(addr);
                        let res = self.alu_rl(val);
                        self.memory.borrow_mut().set(addr, res);
                    }
                    reg => {
                        let res = self.alu_rl(self.register.get_u8(reg));
                        self.register.set_u8(reg, res);
                    }
                }
                OPCODE_CYCLES[opcode as usize][0]
            }
            // RR
            0x18..=0x1F => {
                match Self::get_reg(opcode) {
                    Register::HL => {
                        let addr = self.register.get_u16(Register::HL);
                        let val = self.memory.borrow().get_u8(addr);
                        let res = self.alu_rr(val);
                        self.memory.borrow_mut().set(addr, res);
                    }
                    reg => {
                        let res = self.alu_rr(self.register.get_u8(reg));
                        self.register.set_u8(reg, res);
                    }
                }
                OPCODE_CYCLES[opcode as usize][0]
            }
            // SLA
            0x20..=0x27 => {
                match Self::get_reg(opcode) {
                    Register::HL => {
                        let addr = self.register.get_u16(Register::HL);
                        let val = self.memory.borrow().get_u8(addr);
                        let res = self.alu_sla(val);
                        self.memory.borrow_mut().set(addr, res);
                    }
                    reg => {
                        let res = self.alu_sla(self.register.get_u8(reg));
                        self.register.set_u8(reg, res);
                    }
                }
                OPCODE_CYCLES[opcode as usize][0]
            }
            // SRA
            0x28..=0x2F => {
                match Self::get_reg(opcode) {
                    Register::HL => {
                        let addr = self.register.get_u16(Register::HL);
                        let val = self.memory.borrow().get_u8(addr);
                        let res = self.alu_sra(val);
                        self.memory.borrow_mut().set(addr, res);
                    }
                    reg => {
                        let res = self.alu_sra(self.register.get_u8(reg));
                        self.register.set_u8(reg, res);
                    }
                }
                OPCODE_CYCLES[opcode as usize][0]
            }
            // SWAP
            0x30..=0x37 => {
                match Self::get_reg(opcode) {
                    Register::HL => {
                        let addr = self.register.get_u16(Register::HL);
                        let val = self.memory.borrow().get_u8(addr);
                        let res = self.alu_swap(val);
                        self.memory.borrow_mut().set(addr, res);
                    }
                    reg => {
                        let res = self.alu_swap(self.register.get_u8(reg));
                        self.register.set_u8(reg, res);
                    }
                }
                OPCODE_CYCLES[opcode as usize][0]
            }
            // SRL
            0x38..=0x3F => {
                match Self::get_reg(opcode) {
                    Register::HL => {
                        let addr = self.register.get_u16(Register::HL);
                        let val = self.memory.borrow().get_u8(addr);
                        let res = self.alu_srl(val);
                        self.memory.borrow_mut().set(addr, res);
                    }
                    reg => {
                        let res = self.alu_srl(self.register.get_u8(reg));
                        self.register.set_u8(reg, res);
                    }
                }
                OPCODE_CYCLES[opcode as usize][0]
            }
            // BIT [0-7],B-A
            0x40..=0x7F => {
                let n = match (opcode) {
                    0x40..=0x47 => 0,
                    0x48..=0x4F => 1,
                    0x50..=0x57 => 2,
                    0x50..=0x5F => 3,
                    0x60..=0x67 => 4,
                    0x60..=0x6F => 5,
                    0x70..=0x77 => 6,
                    0x70..=0x7F => 7,
                    other => panic!("Unsupported opcode: 0x{:04X}", other),
                };
                match Self::get_reg(opcode) {
                    Register::HL => {
                        let addr = self.register.get_u16(Register::HL);
                        let val = self.memory.borrow().get_u8(addr);
                        self.alu_bit(val, n);
                    }
                    reg => {
                        self.alu_bit(self.register.get_u8(reg), n);
                    }
                }
                OPCODE_CYCLES[opcode as usize][0]
            }
            // RES [0-7],B-A
            0x80..=0xBF => {
                let n = match (opcode) {
                    0x80..=0x87 => 0,
                    0x88..=0x8F => 1,
                    0x90..=0x97 => 2,
                    0x90..=0x9F => 3,
                    0xA0..=0xA7 => 4,
                    0xA0..=0xAF => 5,
                    0xB0..=0xB7 => 6,
                    0xB0..=0xBF => 7,
                    other => panic!("Unsupported opcode: 0x{:04X}", other),
                };
                match Self::get_reg(opcode) {
                    Register::HL => {
                        let addr = self.register.get_u16(Register::HL);
                        let val = self.memory.borrow().get_u8(addr);
                        let res = self.alu_reset(val, n);
                        self.memory.borrow_mut().set(addr, res);
                    }
                    reg => {
                        let res = self.alu_reset(self.register.get_u8(reg), n);
                        self.register.set_u8(reg, res);
                    }
                }
                OPCODE_CYCLES[opcode as usize][0]
            }
            // SET [0-7],B-A
            0xC0..=0xFF => {
                let n = match (opcode) {
                    0xC0..=0xC7 => 0,
                    0xC8..=0xCF => 1,
                    0xD0..=0xD7 => 2,
                    0xD0..=0xDF => 3,
                    0xE0..=0xE7 => 4,
                    0xE0..=0xEF => 5,
                    0xF0..=0xF7 => 6,
                    0xF0..=0xFF => 7,
                    other => panic!("Unsupported opcode: 0x{:04X}", other),
                };
                match Self::get_reg(opcode) {
                    Register::HL => {
                        let addr = self.register.get_u16(Register::HL);
                        let val = self.memory.borrow().get_u8(addr);
                        let res = self.alu_set(val, n);
                        self.memory.borrow_mut().set(addr, res);
                    }
                    reg => {
                        let res = self.alu_set(self.register.get_u8(reg), n);
                        self.register.set_u8(reg, res);
                    }
                }
                OPCODE_CYCLES[opcode as usize][0]
            }
            other => panic!("Unsupported opcode: 0x{:04X}", other),
        }
    }

    fn get_reg(opcode: u8) -> Register {
        match (opcode & 0x0F) {
            0x00 | 0x08 => Register::B,
            0x01 | 0x09 => Register::C,
            0x02 | 0x0A => Register::D,
            0x03 | 0x0B => Register::E,
            0x04 | 0x0C => Register::H,
            0x05 | 0x0D => Register::L,
            0x06 | 0x0E => Register::HL,
            0x07 | 0x0F => Register::A,
            other => panic!("Unsupported opcode: 0x{:04X}", other),
        }
    }

    fn alu_sla(&mut self, val: u8) -> u8 {
        let res = val << 1;
        self.register.set_flag(Flag::Z, res == 0);
        self.register.set_flag(Flag::N, false);
        self.register.set_flag(Flag::H, false);
        self.register.set_flag(Flag::C, val & 0x80 == 0x80);
        res
    }

    // 算术右移(带符号右移)
    fn alu_sra(&mut self, val: u8) -> u8 {
        let res = (val & 0x80) | (val >> 1); // 符号位
        self.register.set_flag(Flag::Z, res == 0);
        self.register.set_flag(Flag::N, false);
        self.register.set_flag(Flag::H, false);
        self.register.set_flag(Flag::C, val & 0x01 == 0x01);
        res
    }

    fn alu_swap(&mut self, val: u8) -> u8 {
        self.register.set_flag(Flag::Z, val == 0);
        self.register.set_flag(Flag::N, false);
        self.register.set_flag(Flag::H, false);
        self.register.set_flag(Flag::C, false);
        (val << 4) | (val >> 4)
    }

    // 逻辑右移
    fn alu_srl(&mut self, val: u8) -> u8 {
        let res = (val >> 1);
        self.register.set_flag(Flag::Z, res == 0);
        self.register.set_flag(Flag::N, false);
        self.register.set_flag(Flag::H, false);
        self.register.set_flag(Flag::C, val & 0x01 == 0x01);
        res
    }

    fn alu_bit(&mut self, val: u8, n: u8) -> bool {
        let res = val & (1 << n) == 0x00;
        self.register.set_flag(Flag::Z, res);
        self.register.set_flag(Flag::N, false);
        self.register.set_flag(Flag::H, true);
        res
    }

    fn alu_reset(&mut self, val: u8, n: u8) -> u8 {
        val & (!(1 << n))
    }

    fn alu_set(&mut self, val: u8, n: u8) -> u8 {
        val | (1 << n)
    }
}
