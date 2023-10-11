use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x54,
    group: "x8/lsm",
    parameters: [Some("D"), Some("H")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | D,H | 0x54 | 4
pub struct _0x0054 {
    meta: &'static OpcodeMeta,
}

pub static _0x0054_: _0x0054 = _0x0054 {
    meta: &META,
};

impl Opcode for _0x0054 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) -> u8 {
        
let right = cpu.register.get_u8(Register::H);
// no flag effect
cpu.register.set_u8(Register::D, right);
self.meta.cycles[0]
    }
}
