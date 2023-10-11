use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x52,
    group: "x8/lsm",
    parameters: [Some("D"), Some("D")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | D,D | 0x52 | 4
pub struct _0x0052 {
    meta: &'static OpcodeMeta,
}

pub static _0x0052_: _0x0052 = _0x0052 {
    meta: &META,
};

impl Opcode for _0x0052 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        
let right = cpu.register.get_u8(Register::D);
// no flag effect
cpu.register.set_u8(Register::D, right);

    }
}
