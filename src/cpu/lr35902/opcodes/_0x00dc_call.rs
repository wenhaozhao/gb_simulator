use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "CALL",
    length: 3,
    cycles: [24, 12],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xDC,
    group: "control/br",
    parameters: [Some("CC"), Some("a16")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// CALL | CC,a16 | 0xDC | 24/12
pub struct _0x00DC {
    meta: &'static OpcodeMeta,
}

pub static _0x00DC_: _0x00DC = _0x00DC {
    meta: &META,
};

impl Opcode for _0x00DC {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) -> u8 {
        todo!()
    }
}
