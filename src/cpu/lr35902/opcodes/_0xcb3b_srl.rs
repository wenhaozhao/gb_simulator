use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "SRL",
    length: 2,
    cycles: [8, 0],
    flags: [FlagEffect::Fun(Flag::Z), FlagEffect::Reset(Flag::N), FlagEffect::Reset(Flag::H), FlagEffect::Fun(Flag::C)],
    addr: 0x3B,
    group: "x8/rsb",
    parameters: [Some("E"), None],
    cb_prefixed: true,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// SRL | E | 0x3B | 8
pub struct _0xCB3B {
    meta: &'static OpcodeMeta,
}

pub static _0xCB3B_: _0xCB3B = _0xCB3B {
    meta: &META,
};

impl Opcode for _0xCB3B {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) -> u8 {
        todo!()
    }
}
