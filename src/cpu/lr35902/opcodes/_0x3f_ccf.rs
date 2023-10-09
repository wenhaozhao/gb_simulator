use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "CCF",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::None, FlagEffect::Reset(Flag::N), FlagEffect::Reset(Flag::H), FlagEffect::Fun(Flag::C)],
    addr: 0x3F,
    group: "x8/alu",
    parameters: [None, None],
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// CCF |  | 0x3F | 4
pub struct _0x3F {
    meta: &'static OpcodeMeta,
}

pub static _0x3F_: _0x3F = _0x3F {
    meta: &META,
};

impl Opcode for _0x3F {

    fn get_meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
