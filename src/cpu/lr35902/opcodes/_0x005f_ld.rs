use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x5F,
    group: "x8/lsm",
    parameters: [Some("E"), Some("A")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | E,A | 0x5F | 4
pub struct _0x005F {
    meta: &'static OpcodeMeta,
}

pub static _0x005F_: _0x005F = _0x005F {
    meta: &META,
};

impl Opcode for _0x005F {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        
let right = cpu.register.get_u8(Register::A);
// no flag effect
cpu.register.set_u8(Register::E, right);

    }
}
