use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "PUSH",
    length: 1,
    cycles: [16, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xE5,
    group: "x16/lsm",
    parameters: [Some("HL"), None],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// PUSH | HL | 0xE5 | 16
pub struct _0x00E5 {
    meta: &'static OpcodeMeta,
}

pub static _0x00E5_: _0x00E5 = _0x00E5 {
    meta: &META,
};

impl Opcode for _0x00E5 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
