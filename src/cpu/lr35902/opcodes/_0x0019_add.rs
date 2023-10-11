use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "ADD",
    length: 1,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::Reset(Flag::N), FlagEffect::Fun(Flag::H), FlagEffect::Fun(Flag::C)],
    addr: 0x19,
    group: "x16/alu",
    parameters: [Some("HL"), Some("DE")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// ADD | HL,DE | 0x19 | 8
pub struct _0x0019 {
    meta: &'static OpcodeMeta,
}

pub static _0x0019_: _0x0019 = _0x0019 {
    meta: &META,
};

impl Opcode for _0x0019 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
