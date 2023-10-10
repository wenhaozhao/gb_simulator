use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x6E,
    group: "x8/lsm",
    parameters: [Some("L"), Some("(HL)")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | L,(HL) | 0x6E | 8
pub struct _0x006E {
    meta: &'static OpcodeMeta,
}

pub static _0x006E_: _0x006E = _0x006E {
    meta: &META,
};

impl Opcode for _0x006E {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        let right = cpu.register.get_hl();
let right = cpu.memory.borrow().get(right);
cpu.register.set_l(right);
    }
}
