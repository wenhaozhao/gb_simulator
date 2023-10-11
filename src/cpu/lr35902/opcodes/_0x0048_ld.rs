use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x48,
    group: "x8/lsm",
    parameters: [Some("C"), Some("B")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | C,B | 0x48 | 4
pub struct _0x0048 {
    meta: &'static OpcodeMeta,
}

pub static _0x0048_: _0x0048 = _0x0048 {
    meta: &META,
};

impl Opcode for _0x0048 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        
let right = cpu.register.get_u8(Register::B);

cpu.register.set_u8(Register::C, right);

    }
}
