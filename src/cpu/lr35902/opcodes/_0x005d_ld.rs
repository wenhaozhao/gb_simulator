use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x5D,
    group: "x8/lsm",
    parameters: [Some("E"), Some("L")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | E,L | 0x5D | 4
pub struct _0x005D {
    meta: &'static OpcodeMeta,
}

pub static _0x005D_: _0x005D = _0x005D {
    meta: &META,
};

impl Opcode for _0x005D {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        let right = cpu.register.get_l();
cpu.register.set_e(right);
    }
}