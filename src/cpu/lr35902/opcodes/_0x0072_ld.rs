use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x72,
    group: "x8/lsm",
    parameters: [Some("(HL)"), Some("D")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | (HL),D | 0x72 | 8
pub struct _0x0072 {
    meta: &'static OpcodeMeta,
}

pub static _0x0072_: _0x0072 = _0x0072 {
    meta: &META,
};

impl Opcode for _0x0072 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        let left = cpu.register.get_u16(Register::HL);
let right = cpu.register.get_u8(Register::D);

cpu.memory.borrow_mut().set_u8(left, right);

    }
}
