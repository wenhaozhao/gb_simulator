use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x78,
    group: "x8/lsm",
    parameters: [Some("A"), Some("B")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | A,B | 0x78 | 4
pub struct _0x0078 {
    meta: &'static OpcodeMeta,
}

pub static _0x0078_: _0x0078 = _0x0078 {
    meta: &META,
};

impl Opcode for _0x0078 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) -> u8 {
        
let right = cpu.register.get_u8(Register::B);
// no flag effect
cpu.register.set_u8(Register::A, right);
self.meta.cycles[0]
    }
}
