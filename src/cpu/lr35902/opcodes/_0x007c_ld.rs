use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x7C,
    group: "x8/lsm",
    parameters: [Some("A"), Some("H")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | A,H | 0x7C | 4
pub struct _0x007C {
    meta: &'static OpcodeMeta,
}

pub static _0x007C_: _0x007C = _0x007C {
    meta: &META,
};

impl Opcode for _0x007C {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) -> u8 {
        
let right = cpu.register.get_u8(Register::H);
// no flag effect
cpu.register.set_u8(Register::A, right);
self.meta.cycles[0]
    }
}
