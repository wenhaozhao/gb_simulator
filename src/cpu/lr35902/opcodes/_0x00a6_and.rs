use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "AND",
    length: 1,
    cycles: [8, 0],
    flags: [FlagEffect::Fun(Flag::Z), FlagEffect::Reset(Flag::N), FlagEffect::Set(Flag::H), FlagEffect::Reset(Flag::C)],
    addr: 0xA6,
    group: "x8/alu",
    parameters: [Some("(HL)"), None],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// AND | (HL) | 0xA6 | 8
pub struct _0x00A6 {
    meta: &'static OpcodeMeta,
}

pub static _0x00A6_: _0x00A6 = _0x00A6 {
    meta: &META,
};

impl Opcode for _0x00A6 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) -> u8 {
        todo!()
    }
}
