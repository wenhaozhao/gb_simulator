use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "RES",
    length: 2,
    cycles: [16, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x8E,
    group: "x8/rsb",
    parameters: [Some("1"), Some("(HL)")],
    cb_prefixed: true,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// RES | 1,(HL) | 0x8E | 16
pub struct _0xCB8E {
    meta: &'static OpcodeMeta,
}

pub static _0xCB8E_: _0xCB8E = _0xCB8E {
    meta: &META,
};

impl Opcode for _0xCB8E {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
