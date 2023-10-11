use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "SET",
    length: 2,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xE5,
    group: "x8/rsb",
    parameters: [Some("4"), Some("L")],
    cb_prefixed: true,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// SET | 4,L | 0xE5 | 8
pub struct _0xCBE5 {
    meta: &'static OpcodeMeta,
}

pub static _0xCBE5_: _0xCBE5 = _0xCBE5 {
    meta: &META,
};

impl Opcode for _0xCBE5 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
