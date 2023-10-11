use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "RRC",
    length: 2,
    cycles: [16, 0],
    flags: [FlagEffect::Fun(Flag::Z), FlagEffect::Reset(Flag::N), FlagEffect::Reset(Flag::H), FlagEffect::Fun(Flag::C)],
    addr: 0x0E,
    group: "x8/rsb",
    parameters: [Some("(HL)"), None],
    cb_prefixed: true,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// RRC | (HL) | 0x0E | 16
pub struct _0xCB0E {
    meta: &'static OpcodeMeta,
}

pub static _0xCB0E_: _0xCB0E = _0xCB0E {
    meta: &META,
};

impl Opcode for _0xCB0E {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
