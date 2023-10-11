use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x71,
    group: "x8/lsm",
    parameters: [Some("(HL)"), Some("C")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | (HL),C | 0x71 | 8
pub struct _0x0071 {
    meta: &'static OpcodeMeta,
}

pub static _0x0071_: _0x0071 = _0x0071 {
    meta: &META,
};

impl Opcode for _0x0071 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        let left = cpu.register.get_u16(Register::HL);
let right = cpu.register.get_u8(Register::C);

cpu.memory.borrow_mut().set_u8(left, right);

    }
}
