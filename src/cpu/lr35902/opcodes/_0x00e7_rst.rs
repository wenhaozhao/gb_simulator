use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "RST",
    length: 1,
    cycles: [16, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xE7,
    group: "control/br",
    parameters: [Some("20H"), None],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// RST | 20H | 0xE7 | 16
pub struct _0x00E7 {
    meta: &'static OpcodeMeta,
}

pub static _0x00E7_: _0x00E7 = _0x00E7 {
    meta: &META,
};

impl Opcode for _0x00E7 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
