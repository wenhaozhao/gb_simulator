use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 2,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x0E,
    group: "x8/lsm",
    parameters: [Some("C"), Some("d8")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | C,d8 | 0x0E | 8
pub struct _0x000E {
    meta: &'static OpcodeMeta,
}

pub static _0x000E_: _0x000E = _0x000E {
    meta: &META,
};

impl Opcode for _0x000E {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) -> u8 {
        
let right = cpu.imm_u8();
// no flag effect
cpu.register.set_u8(Register::C, right);
self.meta.cycles[0]
    }
}
