use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 2,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x1E,
    group: "x8/lsm",
    parameters: [Some("E"), Some("d8")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | E,d8 | 0x1E | 8
pub struct _0x001E {
    meta: &'static OpcodeMeta,
}

pub static _0x001E_: _0x001E = _0x001E {
    meta: &META,
};

impl Opcode for _0x001E {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        let right = cpu.imm_u8();
cpu.register.set_e(right);
    }
}