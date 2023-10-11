use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 2,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x3E,
    group: "x8/lsm",
    parameters: [Some("A"), Some("d8")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | A,d8 | 0x3E | 8
pub struct _0x003E {
    meta: &'static OpcodeMeta,
}

pub static _0x003E_: _0x003E = _0x003E {
    meta: &META,
};

impl Opcode for _0x003E {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) -> u8 {
        
let right = cpu.imm_u8();
// no flag effect
cpu.register.set_u8(Register::A, right);
self.meta.cycles[0]
    }
}
