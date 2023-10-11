use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "CALL",
    length: 3,
    cycles: [24, 12],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xCC,
    group: "control/br",
    parameters: [Some("ZZ"), Some("a16")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// CALL | ZZ,a16 | 0xCC | 24/12
pub struct _0x00CC {
    meta: &'static OpcodeMeta,
}

pub static _0x00CC_: _0x00CC = _0x00CC {
    meta: &META,
};

impl Opcode for _0x00CC {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) -> u8 {
        todo!()
    }
}
