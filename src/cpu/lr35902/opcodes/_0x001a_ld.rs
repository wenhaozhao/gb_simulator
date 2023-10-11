use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x1A,
    group: "x8/lsm",
    parameters: [Some("A"), Some("(DE)")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | A,(DE) | 0x1A | 8
pub struct _0x001A {
    meta: &'static OpcodeMeta,
}

pub static _0x001A_: _0x001A = _0x001A {
    meta: &META,
};

impl Opcode for _0x001A {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        
let right = cpu.register.get_de();
let right = cpu.memory.borrow().get(right);
cpu.register.set_a(right);

    }
}
