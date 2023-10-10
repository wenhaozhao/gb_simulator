use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "STOP",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x10,
    group: "control/misc",
    parameters: [Some("0"), None],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// STOP | 0 | 0x10 | 4
pub struct _0x0010 {
    meta: &'static OpcodeMeta,
}

pub static _0x0010_: _0x0010 = _0x0010 {
    meta: &META,
};

impl Opcode for _0x0010 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}