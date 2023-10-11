use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "XOR",
    length: 2,
    cycles: [8, 0],
    flags: [FlagEffect::Fun(Flag::Z), FlagEffect::Reset(Flag::N), FlagEffect::Reset(Flag::H), FlagEffect::Reset(Flag::C)],
    addr: 0xEE,
    group: "x8/alu",
    parameters: [Some("d8"), None],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// XOR | d8 | 0xEE | 8
pub struct _0x00EE {
    meta: &'static OpcodeMeta,
}

pub static _0x00EE_: _0x00EE = _0x00EE {
    meta: &META,
};

impl Opcode for _0x00EE {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
