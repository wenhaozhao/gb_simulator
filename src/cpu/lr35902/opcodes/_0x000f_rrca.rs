use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "RRCA",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::Reset(Flag::Z), FlagEffect::Reset(Flag::N), FlagEffect::Reset(Flag::H), FlagEffect::Fun(Flag::C)],
    addr: 0x0F,
    group: "x8/rsb",
    parameters: [None, None],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// RRCA |  | 0x0F | 4
pub struct _0x000F {
    meta: &'static OpcodeMeta,
}

pub static _0x000F_: _0x000F = _0x000F {
    meta: &META,
};

impl Opcode for _0x000F {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
