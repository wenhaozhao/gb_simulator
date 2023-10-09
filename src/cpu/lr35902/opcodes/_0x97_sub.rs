use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "SUB",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::Fun(Flag::Z), FlagEffect::Set(Flag::N), FlagEffect::Fun(Flag::H), FlagEffect::Fun(Flag::C)],
    addr: 0x97,
    group: "x8/alu",
    parameters: [Some("A"), None],
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// SUB | A | 0x97 | 4
pub struct _0x97 {
    meta: &'static OpcodeMeta,
}

pub static _0x97_: _0x97 = _0x97 {
    meta: &META,
};

impl Opcode for _0x97 {

    fn get_meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
