use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "DEC",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::Fun(Flag::Z), FlagEffect::Set(Flag::N), FlagEffect::Fun(Flag::H), FlagEffect::None],
    addr: 0x25,
    group: "x8/alu",
    parameters: [Some("H"), None],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// DEC | H | 0x25 | 4
pub struct _0x0025 {
    meta: &'static OpcodeMeta,
}

pub static _0x0025_: _0x0025 = _0x0025 {
    meta: &META,
};

impl Opcode for _0x0025 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
