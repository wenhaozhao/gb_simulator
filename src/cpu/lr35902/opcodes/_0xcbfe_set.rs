use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "SET",
    length: 2,
    cycles: [16, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xFE,
    group: "x8/rsb",
    parameters: [Some("7"), Some("(HL)")],
    cb_prefixed: true,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// SET | 7,(HL) | 0xFE | 16
pub struct _0xCBFE {
    meta: &'static OpcodeMeta,
}

pub static _0xCBFE_: _0xCBFE = _0xCBFE {
    meta: &META,
};

impl Opcode for _0xCBFE {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
