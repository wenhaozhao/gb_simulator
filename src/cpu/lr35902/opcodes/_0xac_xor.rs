use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "XOR",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::Fun(Flag::Z), FlagEffect::Reset(Flag::N), FlagEffect::Reset(Flag::H), FlagEffect::Reset(Flag::C)],
    addr: 0xAC,
    group: "x8/alu",
    parameters: [Some("H"), None],
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// XOR | H | 0xAC | 4
pub struct _0xAC {
    meta: &'static OpcodeMeta,
}

pub static _0xAC_: _0xAC = _0xAC {
    meta: &META,
};

impl Opcode for _0xAC {

    fn get_meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
