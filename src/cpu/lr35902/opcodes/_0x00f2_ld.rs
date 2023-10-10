use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xF2,
    group: "x8/lsm",
    parameters: [Some("A"), Some("(C)")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | A,(C) | 0xF2 | 8
pub struct _0x00F2 {
    meta: &'static OpcodeMeta,
}

pub static _0x00F2_: _0x00F2 = _0x00F2 {
    meta: &META,
};

impl Opcode for _0x00F2 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        let right = 0xFF00 | (cpu.register.get_c() as u16);
let right = cpu.memory.borrow().get(right);
cpu.register.set_a(right);
    }
}
