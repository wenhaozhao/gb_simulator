use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "SET",
    length: 2,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xC1,
    group: "x8/rsb",
    parameters: [Some("0"), Some("C")],
    cb_prefixed: true,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// SET | 0,C | 0xC1 | 8
pub struct _0xCBC1 {
    meta: &'static OpcodeMeta,
}

pub static _0xCBC1_: _0xCBC1 = _0xCBC1 {
    meta: &META,
};

impl Opcode for _0xCBC1 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
