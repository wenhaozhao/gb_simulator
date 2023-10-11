use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "RES",
    length: 2,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xA4,
    group: "x8/rsb",
    parameters: [Some("4"), Some("H")],
    cb_prefixed: true,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// RES | 4,H | 0xA4 | 8
pub struct _0xCBA4 {
    meta: &'static OpcodeMeta,
}

pub static _0xCBA4_: _0xCBA4 = _0xCBA4 {
    meta: &META,
};

impl Opcode for _0xCBA4 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
