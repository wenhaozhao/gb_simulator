use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x46,
    group: "x8/lsm",
    parameters: [Some("B"), Some("(HL)")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | B,(HL) | 0x46 | 8
pub struct _0x0046 {
    meta: &'static OpcodeMeta,
}

pub static _0x0046_: _0x0046 = _0x0046 {
    meta: &META,
};

impl Opcode for _0x0046 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        
let right = cpu.memory.borrow().get(cpu.register.get_u16(Register::HL));

cpu.register.set_u8(Register::B, right);

    }
}
