use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x4C,
    group: "x8/lsm",
    parameters: [Some("C"), Some("H")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | C,H | 0x4C | 4
pub struct _0x004C {
    meta: &'static OpcodeMeta,
}

pub static _0x004C_: _0x004C = _0x004C {
    meta: &META,
};

impl Opcode for _0x004C {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        
let right = cpu.register.get_u8(Register::H);

cpu.register.set_u8(Register::C, right);

    }
}
