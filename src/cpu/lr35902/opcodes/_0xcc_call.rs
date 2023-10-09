use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "CALL",
    length: 3,
    cycles: [24, 12],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xCC,
    group: "control/br",
    parameters: [Some("Z"), Some("a16")],
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// CALL | Z,a16 | 0xCC | 24/12
pub struct _0xCC {
    meta: &'static OpcodeMeta,
}

pub static _0xCC_: _0xCC = _0xCC {
    meta: &META,
};

impl Opcode for _0xCC {

    fn get_meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
