use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "JP",
    length: 3,
    cycles: [16, 12],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xC2,
    group: "control/br",
    parameters: [Some("NZ"), Some("a16")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// JP | NZ,a16 | 0xC2 | 16/12
pub struct _0x00C2 {
    meta: &'static OpcodeMeta,
}

pub static _0x00C2_: _0x00C2 = _0x00C2 {
    meta: &META,
};

impl Opcode for _0x00C2 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) -> u8 {
        todo!()
    }
}
