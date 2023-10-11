use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "POP",
    length: 1,
    cycles: [12, 0],
    flags: [FlagEffect::Fun(Flag::Z), FlagEffect::Fun(Flag::N), FlagEffect::Fun(Flag::H), FlagEffect::Fun(Flag::C)],
    addr: 0xF1,
    group: "x16/lsm",
    parameters: [Some("AF"), None],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// POP | AF | 0xF1 | 12
pub struct _0x00F1 {
    meta: &'static OpcodeMeta,
}

pub static _0x00F1_: _0x00F1 = _0x00F1 {
    meta: &META,
};

impl Opcode for _0x00F1 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) -> u8 {
        todo!()
    }
}
