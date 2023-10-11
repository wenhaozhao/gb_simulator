use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 3,
    cycles: [12, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x01,
    group: "x16/lsm",
    parameters: [Some("BC"), Some("d16")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | BC,d16 | 0x01 | 12
pub struct _0x0001 {
    meta: &'static OpcodeMeta,
}

pub static _0x0001_: _0x0001 = _0x0001 {
    meta: &META,
};

impl Opcode for _0x0001 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        
let right = cpu.imm_u16();

cpu.register.set_u16(Register::BC, right);

    }
}
