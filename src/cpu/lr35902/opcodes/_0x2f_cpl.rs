use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "CPL",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::None, FlagEffect::Set(Flag::N), FlagEffect::Set(Flag::H), FlagEffect::None],
    addr: 0x2F,
    group: "x8/alu",
    parameters: [None, None],
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// CPL |  | 0x2F | 4
pub struct _0x2F {
    meta: &'static OpcodeMeta,
}

pub static _0x2F_: _0x2F = _0x2F {
    meta: &META,
};

impl Opcode for _0x2F {

    fn get_meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
