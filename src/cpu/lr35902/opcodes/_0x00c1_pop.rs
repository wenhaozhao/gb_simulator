use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "POP",
    length: 1,
    cycles: [12, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xC1,
    group: "x16/lsm",
    parameters: [Some("BC"), None],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// POP | BC | 0xC1 | 12
pub struct _0x00C1 {
    meta: &'static OpcodeMeta,
}

pub static _0x00C1_: _0x00C1 = _0x00C1 {
    meta: &META,
};

impl Opcode for _0x00C1 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
