use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "AND",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::Fun(Flag::Z), FlagEffect::Reset(Flag::N), FlagEffect::Set(Flag::H), FlagEffect::Reset(Flag::C)],
    addr: 0xA2,
    group: "x8/alu",
    parameters: [Some("D"), None],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// AND | D | 0xA2 | 4
pub struct _0x00A2 {
    meta: &'static OpcodeMeta,
}

pub static _0x00A2_: _0x00A2 = _0x00A2 {
    meta: &META,
};

impl Opcode for _0x00A2 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) -> u8 {
        todo!()
    }
}
