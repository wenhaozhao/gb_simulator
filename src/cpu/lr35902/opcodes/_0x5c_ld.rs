use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x5C,
    group: "x8/lsm",
    parameters: [Some("E"), Some("H")],
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | E,H | 0x5C | 4
pub struct _0x5C {
    meta: &'static OpcodeMeta,
}

pub static _0x5C_: _0x5C = _0x5C {
    meta: &META,
};

impl Opcode for _0x5C {

    fn get_meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
