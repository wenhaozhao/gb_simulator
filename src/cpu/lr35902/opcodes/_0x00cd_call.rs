use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "CALL",
    length: 3,
    cycles: [24, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xCD,
    group: "control/br",
    parameters: [Some("a16"), None],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// CALL | a16 | 0xCD | 24
pub struct _0x00CD {
    meta: &'static OpcodeMeta,
}

pub static _0x00CD_: _0x00CD = _0x00CD {
    meta: &META,
};

impl Opcode for _0x00CD {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
