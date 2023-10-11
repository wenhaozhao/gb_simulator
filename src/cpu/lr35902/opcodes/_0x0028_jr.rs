use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "JR",
    length: 2,
    cycles: [12, 8],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x28,
    group: "control/br",
    parameters: [Some("Z"), Some("r8")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// JR | Z,r8 | 0x28 | 12/8
pub struct _0x0028 {
    meta: &'static OpcodeMeta,
}

pub static _0x0028_: _0x0028 = _0x0028 {
    meta: &META,
};

impl Opcode for _0x0028 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
