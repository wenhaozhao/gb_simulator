use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "RRA",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::Reset(Flag::Z), FlagEffect::Reset(Flag::N), FlagEffect::Reset(Flag::H), FlagEffect::Fun(Flag::C)],
    addr: 0x1F,
    group: "x8/rsb",
    parameters: [None, None],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// RRA |  | 0x1F | 4
pub struct _0x001F {
    meta: &'static OpcodeMeta,
}

pub static _0x001F_: _0x001F = _0x001F {
    meta: &META,
};

impl Opcode for _0x001F {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) -> u8 {
        todo!()
    }
}
