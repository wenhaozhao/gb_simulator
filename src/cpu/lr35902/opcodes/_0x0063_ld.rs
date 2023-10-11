use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x63,
    group: "x8/lsm",
    parameters: [Some("H"), Some("E")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | H,E | 0x63 | 4
pub struct _0x0063 {
    meta: &'static OpcodeMeta,
}

pub static _0x0063_: _0x0063 = _0x0063 {
    meta: &META,
};

impl Opcode for _0x0063 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) -> u8 {
        
let right = cpu.register.get_u8(Register::E);
// no flag effect
cpu.register.set_u8(Register::H, right);
self.meta.cycles[0]
    }
}
