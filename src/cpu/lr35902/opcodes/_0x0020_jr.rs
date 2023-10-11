use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "JR",
    length: 2,
    cycles: [12, 8],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x20,
    group: "control/br",
    parameters: [Some("NZ"), Some("r8")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// JR | NZ,r8 | 0x20 | 12/8
pub struct _0x0020 {
    meta: &'static OpcodeMeta,
}

pub static _0x0020_: _0x0020 = _0x0020 {
    meta: &META,
};

impl Opcode for _0x0020 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
