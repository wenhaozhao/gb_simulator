use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x41,
    group: "x8/lsm",
    parameters: [Some("B"), Some("C")],
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | B,C | 0x41 | 4
pub struct _0x41 {
    meta: &'static OpcodeMeta,
}

pub static _0x41_: _0x41 = _0x41 {
    meta: &META,
};

impl Opcode for _0x41 {

    fn get_meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
