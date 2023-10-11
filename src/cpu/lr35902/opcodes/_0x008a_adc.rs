use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "ADC",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::Fun(Flag::Z), FlagEffect::Reset(Flag::N), FlagEffect::Fun(Flag::H), FlagEffect::Fun(Flag::C)],
    addr: 0x8A,
    group: "x8/alu",
    parameters: [Some("A"), Some("D")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// ADC | A,D | 0x8A | 4
pub struct _0x008A {
    meta: &'static OpcodeMeta,
}

pub static _0x008A_: _0x008A = _0x008A {
    meta: &META,
};

impl Opcode for _0x008A {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
