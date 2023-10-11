use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 3,
    cycles: [20, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x08,
    group: "x16/lsm",
    parameters: [Some("(a16)"), Some("SP")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | (a16),SP | 0x08 | 20
pub struct _0x0008 {
    meta: &'static OpcodeMeta,
}

pub static _0x0008_: _0x0008 = _0x0008 {
    meta: &META,
};

impl Opcode for _0x0008 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) -> u8 {
        let left = cpu.imm_u16();
let right = cpu.register.get_u16(Register::SP);
// no flag effect
cpu.memory.borrow_mut().set_u16(left, right);
self.meta.cycles[0]
    }
}
