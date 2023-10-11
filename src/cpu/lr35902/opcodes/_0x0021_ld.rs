use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 3,
    cycles: [12, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x21,
    group: "x16/lsm",
    parameters: [Some("HL"), Some("d16")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | HL,d16 | 0x21 | 12
pub struct _0x0021 {
    meta: &'static OpcodeMeta,
}

pub static _0x0021_: _0x0021 = _0x0021 {
    meta: &META,
};

impl Opcode for _0x0021 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        
let right = cpu.imm_u16();
cpu.register.set_hl(right);

    }
}
