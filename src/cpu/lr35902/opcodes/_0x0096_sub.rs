use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "SUB",
    length: 1,
    cycles: [8, 0],
    flags: [FlagEffect::Fun(Flag::Z), FlagEffect::Set(Flag::N), FlagEffect::Fun(Flag::H), FlagEffect::Fun(Flag::C)],
    addr: 0x96,
    group: "x8/alu",
    parameters: [Some("(HL)"), None],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// SUB | (HL) | 0x96 | 8
pub struct _0x0096 {
    meta: &'static OpcodeMeta,
}

pub static _0x0096_: _0x0096 = _0x0096 {
    meta: &META,
};

impl Opcode for _0x0096 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
