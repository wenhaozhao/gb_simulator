use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "RET",
    length: 1,
    cycles: [20, 8],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xC8,
    group: "control/br",
    parameters: [Some("Z"), None],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// RET | Z | 0xC8 | 20/8
pub struct _0x00C8 {
    meta: &'static OpcodeMeta,
}

pub static _0x00C8_: _0x00C8 = _0x00C8 {
    meta: &META,
};

impl Opcode for _0x00C8 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) -> u8 {
        todo!()
    }
}
