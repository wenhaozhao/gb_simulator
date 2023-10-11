use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "BIT",
    length: 2,
    cycles: [16, 0],
    flags: [FlagEffect::Fun(Flag::Z), FlagEffect::Reset(Flag::N), FlagEffect::Set(Flag::H), FlagEffect::None],
    addr: 0x6E,
    group: "x8/rsb",
    parameters: [Some("5"), Some("(HL)")],
    cb_prefixed: true,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// BIT | 5,(HL) | 0x6E | 16
pub struct _0xCB6E {
    meta: &'static OpcodeMeta,
}

pub static _0xCB6E_: _0xCB6E = _0xCB6E {
    meta: &META,
};

impl Opcode for _0xCB6E {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
