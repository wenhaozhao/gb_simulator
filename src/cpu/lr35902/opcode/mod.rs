use crate::cpu::CPU;
use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::registers::Flag;

mod unprefixed_opcode;
mod cbprefixed_opcode;

pub const CB_PREFIXED: u16 = 0xCB00;

impl LR35902 {
    fn alu_rlc(&mut self, val: u8) -> u8 {
        let flag_c = val & 0x80 == 0x80;
        let res = (val << 1) | (flag_c as u8);
        self.register.set_flag(Flag::Z, false);
        self.register.set_flag(Flag::N, false);
        self.register.set_flag(Flag::H, false);
        self.register.set_flag(Flag::C, flag_c);
        res
    }

    fn alu_rl(&mut self, val: u8) -> u8 {
        let flag_c = val & 0x80 == 0x80;
        let res = (val << 1) | (self.register.get_flag(Flag::C) as u8);
        self.register.set_flag(Flag::Z, false);
        self.register.set_flag(Flag::N, false);
        self.register.set_flag(Flag::H, false);
        self.register.set_flag(Flag::C, flag_c);
        res
    }

    fn alu_rrc(&mut self, val: u8) -> u8 {
        let flag_c = val & 0x01 == 0x01;
        let res = (if flag_c { 0x80 } else { 0x00 }) | (val >> 1);
        self.register.set_flag(Flag::Z, false);
        self.register.set_flag(Flag::N, false);
        self.register.set_flag(Flag::H, false);
        self.register.set_flag(Flag::C, flag_c);
        res
    }

    fn alu_rr(&mut self, val: u8) -> u8 {
        let flag_c = val & 0x01 == 0x01;
        let res = (if self.register.get_flag(Flag::C) { 0x80 } else { 0x00 }) | (val >> 1);
        self.register.set_flag(Flag::Z, false);
        self.register.set_flag(Flag::N, false);
        self.register.set_flag(Flag::H, false);
        self.register.set_flag(Flag::C, flag_c);
        res
    }
}
