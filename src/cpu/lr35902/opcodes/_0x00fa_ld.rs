use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 3,
    cycles: [16, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xFA,
    group: "x8/lsm",
    parameters: [Some("A"), Some("(a16)")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | A,(a16) | 0xFA | 16
pub struct _0x00FA {
    meta: &'static OpcodeMeta,
}

pub static _0x00FA_: _0x00FA = _0x00FA {
    meta: &META,
};

impl Opcode for _0x00FA {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        let right = cpu.imm_u16();
let right = cpu.memory.borrow().get((a16));
cpu.register.set_a(right);
    }
}