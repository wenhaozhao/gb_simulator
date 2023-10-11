use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 2,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x16,
    group: "x8/lsm",
    parameters: [Some("D"), Some("d8")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | D,d8 | 0x16 | 8
pub struct _0x0016 {
    meta: &'static OpcodeMeta,
}

pub static _0x0016_: _0x0016 = _0x0016 {
    meta: &META,
};

impl Opcode for _0x0016 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) -> u8 {
        
let right = cpu.imm_u8();
// no flag effect
cpu.register.set_u8(Register::D, right);
self.meta.cycles[0]
    }
}
