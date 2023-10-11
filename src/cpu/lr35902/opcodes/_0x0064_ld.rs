use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x64,
    group: "x8/lsm",
    parameters: [Some("H"), Some("H")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | H,H | 0x64 | 4
pub struct _0x0064 {
    meta: &'static OpcodeMeta,
}

pub static _0x0064_: _0x0064 = _0x0064 {
    meta: &META,
};

impl Opcode for _0x0064 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) -> u8 {
        
let right = cpu.register.get_u8(Register::H);
// no flag effect
cpu.register.set_u8(Register::H, right);
self.meta.cycles[0]
    }
}
