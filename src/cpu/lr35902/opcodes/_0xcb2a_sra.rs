use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "SRA",
    length: 2,
    cycles: [8, 0],
    flags: [FlagEffect::Fun(Flag::Z), FlagEffect::Reset(Flag::N), FlagEffect::Reset(Flag::H), FlagEffect::Reset(Flag::C)],
    addr: 0x2A,
    group: "x8/rsb",
    parameters: [Some("D"), None],
    cb_prefixed: true,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// SRA | D | 0x2A | 8
pub struct _0xCB2A {
    meta: &'static OpcodeMeta,
}

pub static _0xCB2A_: _0xCB2A = _0xCB2A {
    meta: &META,
};

impl Opcode for _0xCB2A {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
