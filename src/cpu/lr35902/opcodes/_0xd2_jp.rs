use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "JP",
    length: 3,
    cycles: [16, 12],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xD2,
    group: "control/br",
    parameters: [Some("NC"), Some("a16")],
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// JP | NC,a16 | 0xD2 | 16/12
pub struct _0xD2 {
    meta: &'static OpcodeMeta,
}

pub static _0xD2_: _0xD2 = _0xD2 {
    meta: &META,
};

impl Opcode for _0xD2 {

    fn get_meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
