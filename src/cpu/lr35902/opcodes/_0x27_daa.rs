use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "DAA",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::Fun(Flag::Z), FlagEffect::None, FlagEffect::Reset(Flag::H), FlagEffect::Fun(Flag::C)],
    addr: 0x27,
    group: "x8/alu",
    parameters: [None, None],
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// DAA |  | 0x27 | 4
pub struct _0x27 {
    meta: &'static OpcodeMeta,
}

pub static _0x27_: _0x27 = _0x27 {
    meta: &META,
};

impl Opcode for _0x27 {

    fn get_meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
