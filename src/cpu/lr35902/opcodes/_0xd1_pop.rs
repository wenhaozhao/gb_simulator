use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "POP",
    length: 1,
    cycles: [12, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xD1,
    group: "x16/lsm",
    parameters: [Some("DE"), None],
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// POP | DE | 0xD1 | 12
pub struct _0xD1 {
    meta: &'static OpcodeMeta,
}

pub static _0xD1_: _0xD1 = _0xD1 {
    meta: &META,
};

impl Opcode for _0xD1 {

    fn get_meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
