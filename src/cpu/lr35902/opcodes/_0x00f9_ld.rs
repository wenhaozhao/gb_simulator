use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xF9,
    group: "x16/lsm",
    parameters: [Some("SP"), Some("HL")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | SP,HL | 0xF9 | 8
pub struct _0x00F9 {
    meta: &'static OpcodeMeta,
}

pub static _0x00F9_: _0x00F9 = _0x00F9 {
    meta: &META,
};

impl Opcode for _0x00F9 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) -> u8 {
        
let right = cpu.register.get_u16(Register::HL);
// no flag effect
cpu.register.set_u16(Register::SP, right);
self.meta.cycles[0]
    }
}
