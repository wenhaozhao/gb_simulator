use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "RES",
    length: 2,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xB3,
    group: "x8/rsb",
    parameters: [Some("6"), Some("E")],
    cb_prefixed: true,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// RES | 6,E | 0xB3 | 8
pub struct _0xCBB3 {
    meta: &'static OpcodeMeta,
}

pub static _0xCBB3_: _0xCBB3 = _0xCBB3 {
    meta: &META,
};

impl Opcode for _0xCBB3 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
