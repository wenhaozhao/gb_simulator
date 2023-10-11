use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "AND",
    length: 2,
    cycles: [8, 0],
    flags: [FlagEffect::Fun(Flag::Z), FlagEffect::Reset(Flag::N), FlagEffect::Set(Flag::H), FlagEffect::Reset(Flag::C)],
    addr: 0xE6,
    group: "x8/alu",
    parameters: [Some("d8"), None],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// AND | d8 | 0xE6 | 8
pub struct _0x00E6 {
    meta: &'static OpcodeMeta,
}

pub static _0x00E6_: _0x00E6 = _0x00E6 {
    meta: &META,
};

impl Opcode for _0x00E6 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
