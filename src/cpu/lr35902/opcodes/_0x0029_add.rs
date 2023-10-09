use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "ADD",
    length: 1,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::Reset(Flag::N), FlagEffect::Fun(Flag::H), FlagEffect::Fun(Flag::C)],
    addr: 0x29,
    group: "x16/alu",
    parameters: [Some("HL"), Some("HL")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// ADD | HL,HL | 0x29 | 8
pub struct _0x0029 {
    meta: &'static OpcodeMeta,
}

pub static _0x0029_: _0x0029 = _0x0029 {
    meta: &META,
};

impl Opcode for _0x0029 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
