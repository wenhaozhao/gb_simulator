use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "ADD",
    length: 2,
    cycles: [16, 0],
    flags: [FlagEffect::Reset(Flag::Z), FlagEffect::Reset(Flag::N), FlagEffect::Fun(Flag::H), FlagEffect::Fun(Flag::C)],
    addr: 0xE8,
    group: "x16/alu",
    parameters: [Some("SP"), Some("r8")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// ADD | SP,r8 | 0xE8 | 16
pub struct _0x00E8 {
    meta: &'static OpcodeMeta,
}

pub static _0x00E8_: _0x00E8 = _0x00E8 {
    meta: &META,
};

impl Opcode for _0x00E8 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
