use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x57,
    group: "x8/lsm",
    parameters: [Some("D"), Some("A")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | D,A | 0x57 | 4
pub struct _0x0057 {
    meta: &'static OpcodeMeta,
}

pub static _0x0057_: _0x0057 = _0x0057 {
    meta: &META,
};

impl Opcode for _0x0057 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) -> u8 {
        
let right = cpu.register.get_u8(Register::A);
// no flag effect
cpu.register.set_u8(Register::D, right);
self.meta.cycles[0]
    }
}
