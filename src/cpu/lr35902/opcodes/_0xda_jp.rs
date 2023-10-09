use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "JP",
    length: 3,
    cycles: [16, 12],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xDA,
    group: "control/br",
    parameters: [Some("C"), Some("a16")],
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// JP | C,a16 | 0xDA | 16/12
pub struct _0xDA {
    meta: &'static OpcodeMeta,
}

pub static _0xDA_: _0xDA = _0xDA {
    meta: &META,
};

impl Opcode for _0xDA {

    fn get_meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
