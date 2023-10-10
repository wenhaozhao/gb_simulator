use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "SBC",
    length: 2,
    cycles: [8, 0],
    flags: [FlagEffect::Fun(Flag::Z), FlagEffect::Set(Flag::N), FlagEffect::Fun(Flag::H), FlagEffect::Fun(Flag::C)],
    addr: 0xDE,
    group: "x8/alu",
    parameters: [Some("A"), Some("d8")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// SBC | A,d8 | 0xDE | 8
pub struct _0x00DE {
    meta: &'static OpcodeMeta,
}

pub static _0x00DE_: _0x00DE = _0x00DE {
    meta: &META,
};

impl Opcode for _0x00DE {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}