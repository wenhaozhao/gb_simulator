use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x6A,
    group: "x8/lsm",
    parameters: [Some("L"), Some("D")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | L,D | 0x6A | 4
pub struct _0x006A {
    meta: &'static OpcodeMeta,
}

pub static _0x006A_: _0x006A = _0x006A {
    meta: &META,
};

impl Opcode for _0x006A {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        
let right = cpu.register.get_u8(Register::D);

cpu.register.set_u8(Register::L, right);

    }
}
