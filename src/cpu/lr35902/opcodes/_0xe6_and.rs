use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "AND",
    length: 2,
    cycles: [8, 0],
    flags: [FlagEffect::Fun(Flag::Z), FlagEffect::Reset(Flag::N), FlagEffect::Set(Flag::H), FlagEffect::Reset(Flag::C)],
    addr: 0xE6,
    group: "x8/alu",
    parameters: [Some("d8"), None],
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// AND | d8 | 0xE6 | 8
pub struct _0xE6 {
    meta: &'static OpcodeMeta,
}

pub static _0xE6_: _0xE6 = _0xE6 {
    meta: &META,
};

impl Opcode for _0xE6 {

    fn get_meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
