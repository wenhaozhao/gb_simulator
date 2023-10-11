use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x2A,
    group: "x8/lsm",
    parameters: [Some("A"), Some("(HL+)")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | A,(HL+) | 0x2A | 8
pub struct _0x002A {
    meta: &'static OpcodeMeta,
}

pub static _0x002A_: _0x002A = _0x002A {
    meta: &META,
};

impl Opcode for _0x002A {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        

cpu.register.set_a(right);

    }
}
