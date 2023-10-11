use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x49,
    group: "x8/lsm",
    parameters: [Some("C"), Some("C")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | C,C | 0x49 | 4
pub struct _0x0049 {
    meta: &'static OpcodeMeta,
}

pub static _0x0049_: _0x0049 = _0x0049 {
    meta: &META,
};

impl Opcode for _0x0049 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        
let right = cpu.register.get_u8(Register::C);
// no flag effect
cpu.register.set_u8(Register::C, right);

    }
}
