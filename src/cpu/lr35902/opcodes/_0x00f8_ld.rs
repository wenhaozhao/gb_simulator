use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 2,
    cycles: [12, 0],
    flags: [FlagEffect::Reset(Flag::Z), FlagEffect::Reset(Flag::N), FlagEffect::Fun(Flag::H), FlagEffect::Fun(Flag::C)],
    addr: 0xF8,
    group: "x16/lsm",
    parameters: [Some("HL"), Some("SP+r8")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | HL,SP+r8 | 0xF8 | 12
pub struct _0x00F8 {
    meta: &'static OpcodeMeta,
}

pub static _0x00F8_: _0x00F8 = _0x00F8 {
    meta: &META,
};

impl Opcode for _0x00F8 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        
let flag_effect_l = cpu.register.get_u16(Register::SP);
let flag_effect_r = cpu.imm_u8() as i8 as i16 as u16;
let right = flag_effect_l.wrapping_add(flag_effect_r);
cpu.register.set_u16(Register::HL, right);


self.meta.flags[0].effect(cpu, 0, 0);
self.meta.flags[1].effect(cpu, 0, 0);
self.meta.flags[2].effect(cpu, flag_effect_l, flag_effect_r);
self.meta.flags[3].effect(cpu, flag_effect_l, flag_effect_r);
cpu.register.set_u16(Register::HL, right);

    }
}
