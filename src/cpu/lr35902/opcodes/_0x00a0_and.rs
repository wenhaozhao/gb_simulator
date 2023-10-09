use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "AND",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::Fun(Flag::Z), FlagEffect::Reset(Flag::N), FlagEffect::Set(Flag::H), FlagEffect::Reset(Flag::C)],
    addr: 0xA0,
    group: "x8/alu",
    parameters: [Some("B"), None],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// AND | B | 0xA0 | 4
pub struct _0x00A0 {
    meta: &'static OpcodeMeta,
}

pub static _0x00A0_: _0x00A0 = _0x00A0 {
    meta: &META,
};

impl Opcode for _0x00A0 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
