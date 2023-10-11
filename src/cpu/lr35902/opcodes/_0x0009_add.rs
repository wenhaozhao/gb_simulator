use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "ADD",
    length: 1,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::Reset(Flag::N), FlagEffect::Fun(Flag::H), FlagEffect::Fun(Flag::C)],
    addr: 0x09,
    group: "x16/alu",
    parameters: [Some("HL"), Some("BC")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// ADD | HL,BC | 0x09 | 8
pub struct _0x0009 {
    meta: &'static OpcodeMeta,
}

pub static _0x0009_: _0x0009 = _0x0009 {
    meta: &META,
};

impl Opcode for _0x0009 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
