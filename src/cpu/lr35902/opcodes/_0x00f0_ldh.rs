use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LDH",
    length: 2,
    cycles: [12, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xF0,
    group: "x8/lsm",
    parameters: [Some("A"), Some("(a8)")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LDH | A,(a8) | 0xF0 | 12
pub struct _0x00F0 {
    meta: &'static OpcodeMeta,
}

pub static _0x00F0_: _0x00F0 = _0x00F0 {
    meta: &META,
};

impl Opcode for _0x00F0 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) -> u8 {
        todo!()
    }
}
