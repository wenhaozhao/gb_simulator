use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::registers::{Flag, Register};

static OPCODE_CYCLES: [[u8; 2]; 0x100] = [
    [4, 0], [12, 0], [8, 0], [8, 0], [4, 0], [4, 0], [8, 0], [4, 0], [20, 0], [8, 0], [8, 0], [8, 0], [4, 0], [4, 0], [8, 0], [4, 0],
    [4, 0], [12, 0], [8, 0], [8, 0], [4, 0], [4, 0], [8, 0], [4, 0], [12, 0], [8, 0], [8, 0], [8, 0], [4, 0], [4, 0], [8, 0], [4, 0],
    [12, 8], [12, 0], [8, 0], [8, 0], [4, 0], [4, 0], [8, 0], [4, 0], [12, 8], [8, 0], [8, 0], [8, 0], [4, 0], [4, 0], [8, 0], [4, 0],
    [12, 8], [12, 0], [8, 0], [8, 0], [12, 0], [12, 0], [12, 0], [4, 0], [12, 8], [8, 0], [8, 0], [8, 0], [4, 0], [4, 0], [8, 0], [4, 0],
    [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [8, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [8, 0], [4, 0],
    [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [8, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [8, 0], [4, 0],
    [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [8, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [8, 0], [4, 0],
    [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [8, 0], [4, 0], [8, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [8, 0], [4, 0],
    [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [8, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [8, 0], [4, 0],
    [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [8, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [8, 0], [4, 0],
    [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [8, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [8, 0], [4, 0],
    [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [8, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [4, 0], [8, 0], [4, 0],
    [20, 8], [12, 0], [16, 12], [16, 0], [24, 12], [16, 0], [8, 0], [16, 0], [20, 8], [16, 0], [16, 12], [4, 0], [24, 12], [24, 0], [8, 0], [16, 0],
    [20, 8], [12, 0], [16, 12], [0, 0], [24, 12], [16, 0], [8, 0], [16, 0], [20, 8], [16, 0], [16, 12], [0, 0], [24, 12], [0, 0], [8, 0], [16, 0],
    [12, 0], [12, 0], [8, 0], [0, 0], [0, 0], [16, 0], [8, 0], [16, 0], [16, 0], [4, 0], [16, 0], [0, 0], [0, 0], [0, 0], [8, 0], [16, 0],
    [12, 0], [12, 0], [8, 0], [4, 0], [0, 0], [16, 0], [8, 0], [16, 0], [12, 0], [8, 0], [16, 0], [4, 0], [0, 0], [0, 0], [8, 0], [16, 0],
];

impl LR35902 {
    pub fn unprefixed_exec_opcode(&mut self, opcode: u8) -> u8 {
        match opcode {
            0x00 => OPCODE_CYCLES[opcode as usize][0], // NOP
            0x10 => OPCODE_CYCLES[opcode as usize][0], // STOP
            0x27 => self.alu_daa(opcode), // DAA
            0x2F => { // CPL
                // A= A xor 0xFF 等价于 A = !A
                self.register.set_u8(Register::A, !self.register.get_u8(Register::A));
                self.register.set_flag(Flag::N, true);
                self.register.set_flag(Flag::H, true);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x37 => { // SCF
                self.register.set_flag(Flag::C, true);
                self.register.set_flag(Flag::N, false);
                self.register.set_flag(Flag::H, false);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x3F => {
                // CCF
                let flag_c_before = self.register.get_flag(Flag::C);
                self.register.set_flag(Flag::C, !flag_c_before);
                self.register.set_flag(Flag::N, false);
                self.register.set_flag(Flag::H, false);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x76 => { // HALT
                self.halted = true;
                OPCODE_CYCLES[opcode as usize][0]
            }
            0xF3 => { // DI
                self.enable_interrupt = false;
                OPCODE_CYCLES[opcode as usize][0]
            }
            0xFB => { //EI
                self.enable_interrupt = true;
                OPCODE_CYCLES[opcode as usize][0]
            }
            // JR
            0x18 => {
                let val = self.imm_u8();
                self.alu_jr_i8(opcode, val);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x20 => {
                if !self.register.get_flag(Flag::Z) {
                    let val = self.imm_u8();
                    self.alu_jr_i8(opcode, val);
                    OPCODE_CYCLES[opcode as usize][0]
                } else {
                    OPCODE_CYCLES[opcode as usize][1]
                }
            }
            0x28 => {
                if self.register.get_flag(Flag::Z) {
                    let val = self.imm_u8();
                    self.alu_jr_i8(opcode, val);
                    OPCODE_CYCLES[opcode as usize][0]
                } else {
                    OPCODE_CYCLES[opcode as usize][1]
                }
            }
            0x30 => {
                if !self.register.get_flag(Flag::C) {
                    let val = self.imm_u8();
                    self.alu_jr_i8(opcode, val);
                    OPCODE_CYCLES[opcode as usize][0]
                } else {
                    OPCODE_CYCLES[opcode as usize][1]
                }
            }
            0x38 => {
                if self.register.get_flag(Flag::C) {
                    let val = self.imm_u8();
                    self.alu_jr_i8(opcode, val);
                    OPCODE_CYCLES[opcode as usize][0]
                } else {
                    OPCODE_CYCLES[opcode as usize][1]
                }
            }
            // JP
            0xC2 => {
                if !self.register.get_flag(Flag::Z) {
                    let val = self.imm_u16();
                    self.register.set_u16(Register::PC, val);
                    OPCODE_CYCLES[opcode as usize][0]
                } else {
                    OPCODE_CYCLES[opcode as usize][1]
                }
            }
            0xCA => {
                if self.register.get_flag(Flag::Z) {
                    let val = self.imm_u16();
                    self.register.set_u16(Register::PC, val);
                    OPCODE_CYCLES[opcode as usize][0]
                } else {
                    OPCODE_CYCLES[opcode as usize][1]
                }
            }
            0xD2 => {
                if !self.register.get_flag(Flag::C) {
                    let val = self.imm_u16();
                    self.register.set_u16(Register::PC, val);
                    OPCODE_CYCLES[opcode as usize][0]
                } else {
                    OPCODE_CYCLES[opcode as usize][1]
                }
            }
            0xDA => {
                if self.register.get_flag(Flag::C) {
                    let val = self.imm_u16();
                    self.register.set_u16(Register::PC, val);
                    OPCODE_CYCLES[opcode as usize][0]
                } else {
                    OPCODE_CYCLES[opcode as usize][1]
                }
            }
            0xC3 => {
                let val = self.imm_u16();
                self.register.set_u16(Register::PC, val);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0xE9 => {
                let reg_hl = self.register.get_u16(Register::HL);
                let val = self.memory.borrow().get_u16(reg_hl);
                self.register.set_u16(Register::PC, val);
                OPCODE_CYCLES[opcode as usize][0]
            }
            // RET
            0xC0 => {
                if !self.register.get_flag(Flag::Z) {
                    self.alu_ret(opcode)
                } else {
                    OPCODE_CYCLES[opcode as usize][1]
                }
            }
            0xC8 => {
                if self.register.get_flag(Flag::Z) {
                    self.alu_ret(opcode)
                } else {
                    OPCODE_CYCLES[opcode as usize][1]
                }
            }
            0xC9 => {
                self.alu_ret(opcode);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0xD0 => {
                if !self.register.get_flag(Flag::C) {
                    self.alu_ret(opcode)
                } else {
                    OPCODE_CYCLES[opcode as usize][1]
                }
            }
            0xD8 => {
                if self.register.get_flag(Flag::C) {
                    self.alu_ret(opcode)
                } else {
                    OPCODE_CYCLES[opcode as usize][1]
                }
            }
            0xD9 => { // RETI
                let cycles = self.alu_ret(opcode);
                self.enable_interrupt = true;
                cycles
            }
            // POP
            0xC1 => self.alu_pop(opcode, Register::BC),
            0xD1 => self.alu_pop(opcode, Register::DE),
            0xE1 => self.alu_pop(opcode, Register::HL),
            0xF1 => self.alu_pop(opcode, Register::AF),
            // PUSH
            0xC5 => self.alu_push(opcode, Register::BC),
            0xD5 => self.alu_push(opcode, Register::DE),
            0xE5 => self.alu_push(opcode, Register::HL),
            0xF5 => self.alu_push(opcode, Register::AF),
            // RST
            0xC7 => self.alu_call(opcode, 0x00),
            0xD7 => self.alu_call(opcode, 0x10),
            0xE7 => self.alu_call(opcode, 0x20),
            0xF7 => self.alu_call(opcode, 0x30),
            0xCF => self.alu_call(opcode, 0x08),
            0xDF => self.alu_call(opcode, 0x18),
            0xEF => self.alu_call(opcode, 0x28),
            0xFF => self.alu_call(opcode, 0x38),
            //CALL
            0xC4 => {
                if !self.register.get_flag(Flag::Z) {
                    let addr = self.imm_u16();
                    self.alu_call(opcode, addr)
                } else {
                    OPCODE_CYCLES[opcode as usize][1]
                }
            }
            0xCC => {
                if self.register.get_flag(Flag::Z) {
                    let addr = self.imm_u16();
                    self.alu_call(opcode, addr)
                } else {
                    OPCODE_CYCLES[opcode as usize][1]
                }
            }
            0xCD => {
                let addr = self.imm_u16();
                self.alu_call(opcode, addr)
            }
            0xD4 => {
                if !self.register.get_flag(Flag::C) {
                    let addr = self.imm_u16();
                    self.alu_call(opcode, addr)
                } else {
                    OPCODE_CYCLES[opcode as usize][1]
                }
            }
            0xDC => {
                if self.register.get_flag(Flag::C) {
                    let addr = self.imm_u16();
                    self.alu_call(opcode, addr)
                } else {
                    OPCODE_CYCLES[opcode as usize][1]
                }
            }
            0x07 => { // RLCA
                let res = self.alu_rlc(self.register.get_u8(Register::A));
                self.register.set_u8(Register::A, res);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x17 => { // RLA
                let res = self.alu_rl(self.register.get_u8(Register::A));
                self.register.set_u8(Register::A, res);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x0F => { // RRCA
                let res = self.alu_rrc(self.register.get_u8(Register::A));
                self.register.set_u8(Register::A, res);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x1F => { // RRA
                let res = self.alu_rr(self.register.get_u8(Register::A));
                self.register.set_u8(Register::A, res);
                OPCODE_CYCLES[opcode as usize][0]
            },
            // LD
            0x01 => {
                let addr = self.imm_u16();
                self.register.set_u16(Register::BC, addr);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x11 => {
                let addr = self.imm_u16();
                self.register.set_u16(Register::DE, addr);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x21 => {
                let addr = self.imm_u16();
                self.register.set_u16(Register::HL, addr);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x31 => {
                let addr = self.imm_u16();
                self.register.set_u16(Register::SP, addr);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x02 => {
                self.memory.borrow_mut().set_u8(self.register.get_u16(Register::BC), self.register.get_u8(Register::A));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x12 => {
                self.memory.borrow_mut().set_u8(self.register.get_u16(Register::DE), self.register.get_u8(Register::A));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x22 => {
                self.memory.borrow_mut().set_u8(self.register.get_and_incr_u16(Register::HL), self.register.get_u8(Register::A));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x32 => {
                self.memory.borrow_mut().set_u8(self.register.get_and_decr_u16(Register::HL), self.register.get_u8(Register::A));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x06 => {
                let val = self.imm_u8();
                self.register.set_u8(Register::B, val);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x16 => {
                let val = self.imm_u8();
                self.register.set_u8(Register::D, val);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x26 => {
                let val = self.imm_u8();
                self.register.set_u8(Register::H, val);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x36 => {
                let addr = self.register.get_u16(Register::HL);
                let val = self.imm_u8();
                self.memory.borrow_mut().set_u8(addr, val);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x08 => {
                let addr = self.imm_u16();
                let val = self.register.get_u16(Register::SP);
                self.memory.borrow_mut().set_u16(addr, val);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x0A => {
                self.register.set_u8(Register::A, self.memory.borrow().get_u8(self.register.get_u16(Register::BC)));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x1A => {
                self.register.set_u8(Register::A, self.memory.borrow().get_u8(self.register.get_u16(Register::DE)));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x2A => {
                let addr = self.register.get_and_incr_u16(Register::HL);
                self.register.set_u8(Register::A, self.memory.borrow().get_u8(addr));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x3A => {
                let addr = self.register.get_and_decr_u16(Register::HL);
                self.register.set_u8(Register::A, self.memory.borrow().get_u8(addr));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x0E => {
                let val = self.imm_u8();
                self.register.set_u8(Register::C, val);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x1E => {
                let val = self.imm_u8();
                self.register.set_u8(Register::E, val);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x2E => {
                let val = self.imm_u8();
                self.register.set_u8(Register::L, val);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x3E => {
                let val = self.imm_u8();
                self.register.set_u8(Register::A, val);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x40 => {
                self.register.set_u8(Register::B, self.register.get_u8(Register::B));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x50 => {
                self.register.set_u8(Register::D, self.register.get_u8(Register::B));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x60 => {
                self.register.set_u8(Register::H, self.register.get_u8(Register::B));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x70 => {
                self.memory.borrow_mut().set_u8(self.register.get_u16(Register::HL), self.register.get_u8(Register::B));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x41 => {
                self.register.set_u8(Register::B, self.register.get_u8(Register::C));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x51 => {
                self.register.set_u8(Register::D, self.register.get_u8(Register::C));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x61 => {
                self.register.set_u8(Register::H, self.register.get_u8(Register::C));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x71 => {
                self.memory.borrow_mut().set_u8(self.register.get_u16(Register::HL), self.register.get_u8(Register::C));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x42 => {
                self.register.set_u8(Register::B, self.register.get_u8(Register::D));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x52 => {
                self.register.set_u8(Register::D, self.register.get_u8(Register::D));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x62 => {
                self.register.set_u8(Register::H, self.register.get_u8(Register::D));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x72 => {
                self.memory.borrow_mut().set_u8(self.register.get_u16(Register::HL), self.register.get_u8(Register::D));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x43 => {
                self.register.set_u8(Register::B, self.register.get_u8(Register::E));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x53 => {
                self.register.set_u8(Register::D, self.register.get_u8(Register::E));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x63 => {
                self.register.set_u8(Register::H, self.register.get_u8(Register::E));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x73 => {
                self.memory.borrow_mut().set_u8(self.register.get_u16(Register::HL), self.register.get_u8(Register::E));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x44 => {
                self.register.set_u8(Register::B, self.register.get_u8(Register::H));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x54 => {
                self.register.set_u8(Register::D, self.register.get_u8(Register::H));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x64 => {
                self.register.set_u8(Register::H, self.register.get_u8(Register::H));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x74 => {
                self.memory.borrow_mut().set_u8(self.register.get_u16(Register::HL), self.register.get_u8(Register::H));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x45 => {
                self.register.set_u8(Register::B, self.register.get_u8(Register::L));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x55 => {
                self.register.set_u8(Register::D, self.register.get_u8(Register::L));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x65 => {
                self.register.set_u8(Register::H, self.register.get_u8(Register::L));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x75 => {
                self.memory.borrow_mut().set_u8(self.register.get_u16(Register::HL), self.register.get_u8(Register::L));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x46 => {
                self.register.set_u8(Register::B, self.memory.borrow().get_u8(self.register.get_u16(Register::HL)));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x56 => {
                self.register.set_u8(Register::D, self.memory.borrow().get_u8(self.register.get_u16(Register::HL)));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x66 => {
                self.register.set_u8(Register::H, self.memory.borrow().get_u8(self.register.get_u16(Register::HL)));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x47 => {
                self.register.set_u8(Register::B, self.register.get_u8(Register::A));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x57 => {
                self.register.set_u8(Register::D, self.register.get_u8(Register::A));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x67 => {
                self.register.set_u8(Register::H, self.register.get_u8(Register::A));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x77 => {
                self.memory.borrow_mut().set_u8(self.register.get_u16(Register::HL), self.register.get_u8(Register::A));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x48 => {
                self.register.set_u8(Register::C, self.register.get_u8(Register::B));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x58 => {
                self.register.set_u8(Register::E, self.register.get_u8(Register::B));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x68 => {
                self.register.set_u8(Register::L, self.register.get_u8(Register::B));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x78 => {
                self.register.set_u8(Register::A, self.register.get_u8(Register::B));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x49 => {
                self.register.set_u8(Register::C, self.register.get_u8(Register::C));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x59 => {
                self.register.set_u8(Register::E, self.register.get_u8(Register::C));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x69 => {
                self.register.set_u8(Register::L, self.register.get_u8(Register::C));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x79 => {
                self.register.set_u8(Register::A, self.register.get_u8(Register::C));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x4A => {
                self.register.set_u8(Register::C, self.register.get_u8(Register::D));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x5A => {
                self.register.set_u8(Register::E, self.register.get_u8(Register::D));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x6A => {
                self.register.set_u8(Register::L, self.register.get_u8(Register::D));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x7A => {
                self.register.set_u8(Register::A, self.register.get_u8(Register::D));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x4B => {
                self.register.set_u8(Register::C, self.register.get_u8(Register::E));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x5B => {
                self.register.set_u8(Register::E, self.register.get_u8(Register::E));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x6B => {
                self.register.set_u8(Register::L, self.register.get_u8(Register::E));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x7B => {
                self.register.set_u8(Register::A, self.register.get_u8(Register::E));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x4C => {
                self.register.set_u8(Register::C, self.register.get_u8(Register::H));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x5C => {
                self.register.set_u8(Register::E, self.register.get_u8(Register::H));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x6C => {
                self.register.set_u8(Register::L, self.register.get_u8(Register::H));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x7C => {
                self.register.set_u8(Register::A, self.register.get_u8(Register::H));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x4D => {
                self.register.set_u8(Register::C, self.register.get_u8(Register::L));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x5D => {
                self.register.set_u8(Register::E, self.register.get_u8(Register::L));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x6D => {
                self.register.set_u8(Register::L, self.register.get_u8(Register::L));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x7D => {
                self.register.set_u8(Register::A, self.register.get_u8(Register::L));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x4E => {
                self.register.set_u8(Register::C, self.memory.borrow().get_u8(self.register.get_u16(Register::HL)));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x5E => {
                self.register.set_u8(Register::E, self.memory.borrow().get_u8(self.register.get_u16(Register::HL)));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x6E => {
                self.register.set_u8(Register::L, self.memory.borrow().get_u8(self.register.get_u16(Register::HL)));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x7E => {
                self.register.set_u8(Register::A, self.memory.borrow().get_u8(self.register.get_u16(Register::HL)));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x4F => {
                self.register.set_u8(Register::C, self.register.get_u8(Register::A));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x5F => {
                self.register.set_u8(Register::E, self.register.get_u8(Register::A));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x6F => {
                self.register.set_u8(Register::L, self.register.get_u8(Register::A));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x7F => {
                self.register.set_u8(Register::A, self.register.get_u8(Register::A));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0xE2 => {
                self.memory.borrow_mut().set_u8(0xFF00 | (self.register.get_u16(Register::C)), self.register.get_u8(Register::A));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0xF2 => {
                self.register.set_u8(Register::A, self.memory.borrow().get_u8(0xFF00 | (self.register.get_u16(Register::C))));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0xF8 => {
                // LD HL,SP+r8
                let v1 = self.register.get_u16(Register::SP);
                let v2 = self.imm_u8() as i8 as i16 as u16;
                let res = v1.wrapping_add(v2);
                self.register.set_flag(Flag::Z, false);
                self.register.set_flag(Flag::N, false);
                self.register.set_flag(Flag::H, (v1 & 0x000F) + (v2 & 0x000F) > 0x000F);
                self.register.set_flag(Flag::C, (v1 & 0x00FF) + (v2 & 0x00FF) > 0x00FF);
                self.register.set_u16(Register::HL, res);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0xF9 => {
                self.register.set_u16(Register::SP, self.register.get_u16(Register::HL));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0xEA => {
                let val = self.register.get_u8(Register::A);
                let addr = self.imm_u16();
                self.memory.borrow_mut().set_u8(addr, val);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0xFA => {
                let addr = self.imm_u16();
                self.register.set_u8(Register::A, self.memory.borrow().get_u8(addr));
                OPCODE_CYCLES[opcode as usize][0]
            }
            0xE0 => { // LDH
                let val = self.register.get_u8(Register::A);
                let addr = 0xFF00 | self.imm_u8() as u16;
                self.memory.borrow_mut().set_u8(addr, val);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0xF0 => { // LDH
                let addr = 0xFF00 | self.imm_u8() as u16;
                let val = self.memory.borrow().get_u8(addr);
                self.register.set_u8(Register::A, val);
                OPCODE_CYCLES[opcode as usize][0]
            }
            // INC
            0x03 => {
                let _ = self.register.incr_and_get_u16(Register::BC);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x13 => {
                let _ = self.register.incr_and_get_u16(Register::DE);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x23 => {
                let _ = self.register.incr_and_get_u16(Register::HL);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x33 => {
                let _ = self.register.incr_and_get_u16(Register::SP);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x04 => self.alu_incr_reg(opcode, Register::B),
            0x14 => self.alu_incr_reg(opcode, Register::D),
            0x24 => self.alu_incr_reg(opcode, Register::H),
            0x34 => self.alu_incr_addr(opcode, self.register.get_u16(Register::HL)),
            0x0C => self.alu_incr_reg(opcode, Register::C),
            0x1C => self.alu_incr_reg(opcode, Register::E),
            0x2C => self.alu_incr_reg(opcode, Register::L),
            0x3C => self.alu_incr_reg(opcode, Register::A),
            // DEC
            0x05 => self.alu_decr_reg(opcode, Register::B),
            0x15 => self.alu_decr_reg(opcode, Register::D),
            0x25 => self.alu_decr_reg(opcode, Register::H),
            0x35 => self.alu_decr_addr(opcode, self.register.get_u16(Register::HL)),
            0x0B => {
                let _ = self.register.decr_and_get_u16(Register::BC);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x1B => {
                let _ = self.register.decr_and_get_u16(Register::DE);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x2B => {
                let _ = self.register.decr_and_get_u16(Register::HL);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x3B => {
                let _ = self.register.decr_and_get_u16(Register::SP);
                OPCODE_CYCLES[opcode as usize][0]
            }
            0x0D => self.alu_decr_reg(opcode, Register::C),
            0x1D => self.alu_decr_reg(opcode, Register::E),
            0x2D => self.alu_decr_reg(opcode, Register::L),
            0x3D => self.alu_decr_reg(opcode, Register::A),
            // ADD
            0x09 => self.alu_add_hl(opcode, self.register.get_u16(Register::BC)),
            0x19 => self.alu_add_hl(opcode, self.register.get_u16(Register::BC)),
            0x29 => self.alu_add_hl(opcode, self.register.get_u16(Register::BC)),
            0x39 => self.alu_add_hl(opcode, self.register.get_u16(Register::BC)),
            0x80 => self.alu_add(opcode, self.register.get_u8(Register::B)),
            0x81 => self.alu_add(opcode, self.register.get_u8(Register::C)),
            0x82 => self.alu_add(opcode, self.register.get_u8(Register::D)),
            0x83 => self.alu_add(opcode, self.register.get_u8(Register::E)),
            0x84 => self.alu_add(opcode, self.register.get_u8(Register::H)),
            0x85 => self.alu_add(opcode, self.register.get_u8(Register::L)),
            0x86 => {
                let addr = self.register.get_u16(Register::HL);
                let val = self.memory.borrow().get_u8(addr);
                self.alu_add(opcode, val)
            }
            0x87 => self.alu_add(opcode, self.register.get_u8(Register::A)),
            0xC6 => {
                let val = self.imm_u8();
                self.alu_add(opcode, val)
            }
            0xE8 => {
                let val = self.imm_u8();
                self.alu_add_sp_i8(opcode, val)
            }
            // ADC
            0x88 => self.alu_adc(opcode, self.register.get_u8(Register::B)),
            0x89 => self.alu_adc(opcode, self.register.get_u8(Register::C)),
            0x8A => self.alu_adc(opcode, self.register.get_u8(Register::D)),
            0x8B => self.alu_adc(opcode, self.register.get_u8(Register::E)),
            0x8C => self.alu_adc(opcode, self.register.get_u8(Register::H)),
            0x8D => self.alu_adc(opcode, self.register.get_u8(Register::L)),
            0x8E => {
                let addr = self.register.get_u16(Register::HL);
                let val = self.memory.borrow().get_u8(addr);
                self.alu_adc(opcode, val)
            }
            0x8F => self.alu_adc(opcode, self.register.get_u8(Register::A)),
            0xCE => {
                let val = self.imm_u8();
                self.alu_adc(opcode, val)
            }
            // SUB
            0x90 => self.alu_sub(opcode, self.register.get_u8(Register::B)),
            0x91 => self.alu_sub(opcode, self.register.get_u8(Register::C)),
            0x92 => self.alu_sub(opcode, self.register.get_u8(Register::D)),
            0x93 => self.alu_sub(opcode, self.register.get_u8(Register::E)),
            0x94 => self.alu_sub(opcode, self.register.get_u8(Register::H)),
            0x95 => self.alu_sub(opcode, self.register.get_u8(Register::L)),
            0x96 => {
                let addr = self.register.get_u16(Register::HL);
                let val = self.memory.borrow().get_u8(addr);
                self.alu_sub(opcode, val)
            }
            0x97 => self.alu_sub(opcode, self.register.get_u8(Register::A)),
            0xD6 => {
                let val = self.imm_u8();
                self.alu_sub(opcode, val)
            }
            // SBC
            0x98 => self.alu_sbc(opcode, self.register.get_u8(Register::B)),
            0x99 => self.alu_sbc(opcode, self.register.get_u8(Register::C)),
            0x9A => self.alu_sbc(opcode, self.register.get_u8(Register::D)),
            0x9B => self.alu_sbc(opcode, self.register.get_u8(Register::E)),
            0x9C => self.alu_sbc(opcode, self.register.get_u8(Register::H)),
            0x9D => self.alu_sbc(opcode, self.register.get_u8(Register::L)),
            0x9E => {
                let addr = self.register.get_u16(Register::HL);
                let val = self.memory.borrow().get_u8(addr);
                self.alu_sbc(opcode, val)
            }
            0x9F => self.alu_sbc(opcode, self.register.get_u8(Register::A)),
            0xDE => {
                let val = self.imm_u8();
                self.alu_sbc(opcode, val)
            }
            // AND
            0xA0 => self.alu_and(opcode, self.register.get_u8(Register::B)),
            0xA1 => self.alu_and(opcode, self.register.get_u8(Register::C)),
            0xA2 => self.alu_and(opcode, self.register.get_u8(Register::D)),
            0xA3 => self.alu_and(opcode, self.register.get_u8(Register::E)),
            0xA4 => self.alu_and(opcode, self.register.get_u8(Register::H)),
            0xA5 => self.alu_and(opcode, self.register.get_u8(Register::L)),
            0xA6 => {
                let addr = self.register.get_u16(Register::HL);
                let val = self.memory.borrow().get_u8(addr);
                self.alu_and(opcode, val)
            }
            0xA7 => self.alu_and(opcode, self.register.get_u8(Register::A)),
            0xE6 => {
                let val = self.imm_u8();
                self.alu_and(opcode, val)
            }
            // XOR
            0xA8 => self.alu_xor(opcode, self.register.get_u8(Register::B)),
            0xA9 => self.alu_xor(opcode, self.register.get_u8(Register::C)),
            0xAA => self.alu_xor(opcode, self.register.get_u8(Register::D)),
            0xAB => self.alu_xor(opcode, self.register.get_u8(Register::E)),
            0xAC => self.alu_xor(opcode, self.register.get_u8(Register::H)),
            0xAD => self.alu_xor(opcode, self.register.get_u8(Register::L)),
            0xAE => {
                let addr = self.register.get_u16(Register::HL);
                let val = self.memory.borrow().get_u8(addr);
                self.alu_xor(opcode, val)
            }
            0xAF => self.alu_xor(opcode, self.register.get_u8(Register::A)),
            0xEE => {
                let val = self.imm_u8();
                self.alu_xor(opcode, val)
            }
            // OR
            0xB0 => self.alu_or(opcode, self.register.get_u8(Register::B)),
            0xB1 => self.alu_or(opcode, self.register.get_u8(Register::C)),
            0xB2 => self.alu_or(opcode, self.register.get_u8(Register::D)),
            0xB3 => self.alu_or(opcode, self.register.get_u8(Register::E)),
            0xB4 => self.alu_or(opcode, self.register.get_u8(Register::H)),
            0xB5 => self.alu_or(opcode, self.register.get_u8(Register::L)),
            0xB6 => {
                let addr = self.register.get_u16(Register::HL);
                let val = self.memory.borrow().get_u8(addr);
                self.alu_or(opcode, val)
            }
            0xB7 => self.alu_or(opcode, self.register.get_u8(Register::A)),
            0xF6 => {
                let val = self.imm_u8();
                self.alu_or(opcode, val)
            }
            // CP
            0xB8 => self.alu_cp(opcode, self.register.get_u8(Register::B)),
            0xB9 => self.alu_cp(opcode, self.register.get_u8(Register::C)),
            0xBA => self.alu_cp(opcode, self.register.get_u8(Register::D)),
            0xBB => self.alu_cp(opcode, self.register.get_u8(Register::E)),
            0xBC => self.alu_cp(opcode, self.register.get_u8(Register::H)),
            0xBD => self.alu_cp(opcode, self.register.get_u8(Register::L)),
            0xBE => {
                let addr = self.register.get_u16(Register::HL);
                let val = self.memory.borrow().get_u8(addr);
                self.alu_cp(opcode, val)
            }
            0xBF => self.alu_cp(opcode, self.register.get_u8(Register::A)),
            0xFE => {
                let val = self.imm_u8();
                self.alu_cp(opcode, val)
            }
            other => panic!("Unsupported opcode:{}", other),
        }
    }

    /// ### 加法十进制调整
    /// - 将十六进制加法结果调整为BCD码表示
    ///     - 0x09+0x08 = 0x11 -> 0x17 (BCD)
    /// - [wiki](https://baike.baidu.com/item/%E5%8D%81%E8%BF%9B%E5%88%B6%E8%B0%83%E6%95%B4%E6%8C%87%E4%BB%A4/20870450)
    fn alu_daa(&mut self, opcode: u8) -> u8 {
        let mut reg_a = self.register.get_u8(Register::A);
        let mut adjust = if self.register.get_flag(Flag::C) { 0x60 } else { 0x00 };
        if self.register.get_flag(Flag::H) {
            adjust |= 0x06;
        };
        if !self.register.get_flag(Flag::N) {
            if reg_a & 0x0F > 0x09 {
                adjust |= 0x06;
            };
            if reg_a > 0x99 {
                adjust |= 0x60;
            };
            reg_a = reg_a.wrapping_add(adjust);
        } else {
            reg_a = reg_a.wrapping_sub(adjust);
        }
        self.register.set_flag(Flag::Z, reg_a == 0x00);
        self.register.set_flag(Flag::H, false);
        self.register.set_flag(Flag::C, adjust >= 0x60);
        self.register.set_u8(Register::A, reg_a);
        OPCODE_CYCLES[opcode as usize][0]
    }

    fn alu_jr_i8(&mut self, opcode: u8, val: u8) -> u8 {
        let pc = self.register.get_u16(Register::PC);
        self.register.set_u16(Register::PC, (pc as i32).wrapping_add(val as i8 as i32) as u16);
        OPCODE_CYCLES[opcode as usize][0]
    }

    fn alu_ret(&mut self, opcode: u8) -> u8 {
        let reg = Register::PC;
        self.alu_pop(opcode, reg)
    }

    fn alu_pop(&mut self, opcode: u8, reg: Register) -> u8 {
        let sp = self.register.get_and_incr_by_u16(Register::SP, 0x02);
        let val = self.memory.borrow().get_u16(sp);
        self.stack.pop();
        self.register.set_u16(reg, val);
        OPCODE_CYCLES[opcode as usize][0]
    }

    fn alu_push(&mut self, opcode: u8, reg: Register) -> u8 {
        let sp = self.register.decr_by_and_get_u16(Register::SP, 0x02);
        let val = self.register.get_u16(reg);
        self.memory.borrow_mut().set_u16(sp, val);
        self.stack.push(val);
        OPCODE_CYCLES[opcode as usize][0]
    }

    fn alu_call(&mut self, opcode: u8, addr: u16) -> u8 {
        let cycles = self.alu_push(opcode, Register::PC);
        self.register.set_u16(Register::PC, addr);
        cycles
    }

    fn alu_incr_reg(&mut self, opcode: u8, reg: Register) -> u8 {
        let before = self.register.get_u8(reg);
        let after = before.wrapping_add(1);
        self.register.set_u8(reg, after);
        self.register.set_flag(Flag::Z, after == 0);
        self.register.set_flag(Flag::N, false);
        self.register.set_flag(Flag::H, (before & 0x0F) + 0x01 > 0x0F);
        OPCODE_CYCLES[opcode as usize][0]
    }

    fn alu_incr_addr(&mut self, opcode: u8, addr: u16) -> u8 {
        let before = self.memory.borrow().get_u8(addr);
        let after = before.wrapping_add(1);
        self.register.set_flag(Flag::Z, after == 0);
        self.register.set_flag(Flag::N, false);
        self.register.set_flag(Flag::H, (before & 0x0F) + 0x01 > 0x0F);
        self.memory.borrow_mut().set_u8(addr, after);
        OPCODE_CYCLES[opcode as usize][0]
    }

    fn alu_decr_reg(&mut self, opcode: u8, reg: Register) -> u8 {
        let before = self.register.get_u8(reg);
        let after = before.wrapping_sub(1);
        self.register.set_u8(reg, after);
        self.register.set_flag(Flag::Z, after == 0);
        self.register.set_flag(Flag::N, true);
        self.register.set_flag(Flag::H, before.trailing_zeros() >= 4);
        OPCODE_CYCLES[opcode as usize][0]
    }

    fn alu_decr_addr(&mut self, opcode: u8, addr: u16) -> u8 {
        let before = self.memory.borrow().get_u8(addr);
        let after = before.wrapping_sub(1);
        self.register.set_flag(Flag::Z, after == 0);
        self.register.set_flag(Flag::N, true);
        self.register.set_flag(Flag::H, before.trailing_zeros() >= 4);
        self.memory.borrow_mut().set_u8(addr, after);
        OPCODE_CYCLES[opcode as usize][0]
    }

    fn alu_add_hl(&mut self, opcode: u8, val: u16) -> u8 {
        let reg_hl = self.register.get_u16(Register::HL);
        let res = reg_hl.wrapping_add(val);
        self.register.set_flag(Flag::N, false);
        self.register.set_flag(Flag::H, (val & 0x0FFF) + (reg_hl & 0x0FFF) > 0x0FFF);
        self.register.set_flag(Flag::C, val > (0xFFFF - reg_hl));
        self.register.set_u16(Register::HL, res);
        OPCODE_CYCLES[opcode as usize][0]
    }

    fn alu_add(&mut self, opcode: u8, val: u8) -> u8 {
        let reg_a = self.register.get_u8(Register::A);
        let res = reg_a.wrapping_add(val);
        self.register.set_flag(Flag::Z, res == 0x00);
        self.register.set_flag(Flag::N, false);
        self.register.set_flag(Flag::H, (reg_a & 0x0F) + (val & 0x0F) > 0x0F);
        self.register.set_flag(Flag::C, ((reg_a as u16) & 0x00FF) + ((val as u16) & 0x00FF) > 0x00FF);
        self.register.set_u8(Register::A, res);
        OPCODE_CYCLES[opcode as usize][0]
    }

    fn alu_add_sp_i8(&mut self, opcode: u8, val: u8) -> u8 {
        let reg_sp = self.register.get_u16(Register::SP);
        let val = val as i8 as i16 as u16;
        let res = reg_sp.wrapping_add(val);
        self.register.set_flag(Flag::Z, false);
        self.register.set_flag(Flag::N, false);
        self.register.set_flag(Flag::H, (reg_sp & 0x000F) + (val & 0x000F) > 0x000F);
        self.register.set_flag(Flag::C, (reg_sp & 0x00FF) + (val & 0x00FF) > 0x00FF);
        self.register.set_u16(Register::SP, res);
        OPCODE_CYCLES[opcode as usize][0]
    }

    fn alu_adc(&mut self, opcode: u8, val: u8) -> u8 {
        let reg_a = self.register.get_u8(Register::A);
        let flag_c = self.register.get_flag(Flag::C) as u8;
        let res = reg_a.wrapping_add(val).wrapping_add(flag_c);
        self.register.set_flag(Flag::Z, res == 0x00);
        self.register.set_flag(Flag::N, false);
        self.register.set_flag(Flag::H, (reg_a & 0x0F) + (val & 0x0F) + (flag_c & 0x0F) > 0x0F);
        self.register.set_flag(Flag::C, ((reg_a as u16) & 0x00FF) + ((val as u16) & 0x00FF) + ((flag_c as u16) & 0x00FF) > 0x00FF);
        self.register.set_u8(Register::A, res);
        OPCODE_CYCLES[opcode as usize][0]
    }

    fn alu_sub(&mut self, opcode: u8, val: u8) -> u8 {
        let reg_a = self.register.get_u8(Register::A);
        let res = reg_a.wrapping_sub(val);
        self.register.set_flag(Flag::Z, res == 0);
        self.register.set_flag(Flag::N, true);
        self.register.set_flag(Flag::H, (reg_a & 0x0F) < (val & 0x0F));
        self.register.set_flag(Flag::C, (reg_a as u16) < (val as u16));
        self.register.set_u8(Register::A, res);
        OPCODE_CYCLES[opcode as usize][0]
    }

    fn alu_sbc(&mut self, opcode: u8, val: u8) -> u8 {
        let reg_a = self.register.get_u8(Register::A);
        let flag_c = self.register.get_flag(Flag::C) as u8;
        let res = reg_a.wrapping_sub(val).wrapping_sub(flag_c);
        self.register.set_flag(Flag::Z, res == 0x00);
        self.register.set_flag(Flag::N, true);
        self.register.set_flag(Flag::H, (reg_a & 0x0F) < ((val & 0x0F) + (flag_c & 0x0F)));
        self.register.set_flag(Flag::C, ((reg_a as u16) & 0x00FF) < ((val as u16) & 0x00FF) + ((flag_c as u16) & 0x00FF));
        self.register.set_u8(Register::A, res);
        OPCODE_CYCLES[opcode as usize][0]
    }

    fn alu_and(&mut self, opcode: u8, val: u8) -> u8 {
        let reg_a = self.register.get_u8(Register::A);
        let res = reg_a & val;
        self.register.set_flag(Flag::Z, res == 0x00);
        self.register.set_flag(Flag::N, false);
        self.register.set_flag(Flag::H, true);
        self.register.set_flag(Flag::C, false);
        self.register.set_u8(Register::A, res);
        OPCODE_CYCLES[opcode as usize][0]
    }

    fn alu_xor(&mut self, opcode: u8, val: u8) -> u8 {
        let reg_a = self.register.get_u8(Register::A);
        let res = reg_a ^ val;
        self.register.set_flag(Flag::Z, res == 0x00);
        self.register.set_flag(Flag::N, false);
        self.register.set_flag(Flag::H, false);
        self.register.set_flag(Flag::C, false);
        self.register.set_u8(Register::A, res);
        OPCODE_CYCLES[opcode as usize][0]
    }

    fn alu_or(&mut self, opcode: u8, val: u8) -> u8 {
        let reg_a = self.register.get_u8(Register::A);
        let res = reg_a | val;
        self.register.set_flag(Flag::Z, res == 0x00);
        self.register.set_flag(Flag::N, false);
        self.register.set_flag(Flag::H, false);
        self.register.set_flag(Flag::C, false);
        self.register.set_u8(Register::A, res);
        OPCODE_CYCLES[opcode as usize][0]
    }

    /// compare
    fn alu_cp(&mut self, opcode: u8, val: u8) -> u8 {
        let reg_a = self.register.get_u8(Register::A);
        let flag_c = self.register.get_flag(Flag::C) as u8;
        let res = reg_a.wrapping_sub(val).wrapping_sub(flag_c);
        self.register.set_flag(Flag::Z, res == 0x00);
        self.register.set_flag(Flag::N, true);
        self.register.set_flag(Flag::H, (reg_a & 0x0F) < ((val & 0x0F) + (flag_c & 0x0F)));
        self.register.set_flag(Flag::C, ((reg_a as u16) & 0x00FF) < ((val as u16) & 0x00FF) + ((flag_c as u16) & 0x00FF));
        OPCODE_CYCLES[opcode as usize][0]
    }
}