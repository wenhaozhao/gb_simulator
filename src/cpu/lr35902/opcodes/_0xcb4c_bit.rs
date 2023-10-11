use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "BIT",
    length: 2,
    cycles: [8, 0],
    flags: [FlagEffect::Fun(Flag::Z), FlagEffect::Reset(Flag::N), FlagEffect::Set(Flag::H), FlagEffect::None],
    addr: 0x4C,
    group: "x8/rsb",
    parameters: [Some("1"), Some("H")],
    cb_prefixed: true,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// BIT | 1,H | 0x4C | 8
pub struct _0xCB4C {
    meta: &'static OpcodeMeta,
}

pub static _0xCB4C_: _0xCB4C = _0xCB4C {
    meta: &META,
};

impl Opcode for _0xCB4C {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
