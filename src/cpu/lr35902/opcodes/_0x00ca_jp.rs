use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "JP",
    length: 3,
    cycles: [16, 12],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xCA,
    group: "control/br",
    parameters: [Some("Z"), Some("a16")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// JP | Z,a16 | 0xCA | 16/12
pub struct _0x00CA {
    meta: &'static OpcodeMeta,
}

pub static _0x00CA_: _0x00CA = _0x00CA {
    meta: &META,
};

impl Opcode for _0x00CA {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
