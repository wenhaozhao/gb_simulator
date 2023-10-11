use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "RET",
    length: 1,
    cycles: [20, 8],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xD8,
    group: "control/br",
    parameters: [Some("C"), None],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// RET | C | 0xD8 | 20/8
pub struct _0x00D8 {
    meta: &'static OpcodeMeta,
}

pub static _0x00D8_: _0x00D8 = _0x00D8 {
    meta: &META,
};

impl Opcode for _0x00D8 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) -> u8 {
        todo!()
    }
}
