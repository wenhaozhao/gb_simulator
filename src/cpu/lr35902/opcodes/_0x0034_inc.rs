use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "INC",
    length: 1,
    cycles: [12, 0],
    flags: [FlagEffect::Fun(Flag::Z), FlagEffect::Reset(Flag::N), FlagEffect::Fun(Flag::H), FlagEffect::None],
    addr: 0x34,
    group: "x8/alu",
    parameters: [Some("(HL)"), None],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// INC | (HL) | 0x34 | 12
pub struct _0x0034 {
    meta: &'static OpcodeMeta,
}

pub static _0x0034_: _0x0034 = _0x0034 {
    meta: &META,
};

impl Opcode for _0x0034 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
