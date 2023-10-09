use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "RST",
    length: 1,
    cycles: [16, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xD7,
    group: "control/br",
    parameters: [Some("10H"), None],
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// RST | 10H | 0xD7 | 16
pub struct _0xD7 {
    meta: &'static OpcodeMeta,
}

pub static _0xD7_: _0xD7 = _0xD7 {
    meta: &META,
};

impl Opcode for _0xD7 {

    fn get_meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
