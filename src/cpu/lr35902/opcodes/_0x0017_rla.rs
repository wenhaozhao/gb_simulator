use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "RLA",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::Reset(Flag::Z), FlagEffect::Reset(Flag::N), FlagEffect::Reset(Flag::H), FlagEffect::Fun(Flag::C)],
    addr: 0x17,
    group: "x8/rsb",
    parameters: [None, None],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// RLA |  | 0x17 | 4
pub struct _0x0017 {
    meta: &'static OpcodeMeta,
}

pub static _0x0017_: _0x0017 = _0x0017 {
    meta: &META,
};

impl Opcode for _0x0017 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
