use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x7B,
    group: "x8/lsm",
    parameters: [Some("A"), Some("E")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | A,E | 0x7B | 4
pub struct _0x007B {
    meta: &'static OpcodeMeta,
}

pub static _0x007B_: _0x007B = _0x007B {
    meta: &META,
};

impl Opcode for _0x007B {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        
let right = cpu.register.get_u8(Register::E);

cpu.register.set_u8(Register::A, right);

    }
}
