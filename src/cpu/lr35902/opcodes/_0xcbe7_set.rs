use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "SET",
    length: 2,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xE7,
    group: "x8/rsb",
    parameters: [Some("4"), Some("A")],
    cb_prefixed: true,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// SET | 4,A | 0xE7 | 8
pub struct _0xCBE7 {
    meta: &'static OpcodeMeta,
}

pub static _0xCBE7_: _0xCBE7 = _0xCBE7 {
    meta: &META,
};

impl Opcode for _0xCBE7 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
