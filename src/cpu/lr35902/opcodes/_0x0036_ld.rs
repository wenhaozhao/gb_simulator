use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 2,
    cycles: [12, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x36,
    group: "x8/lsm",
    parameters: [Some("(HL)"), Some("d8")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | (HL),d8 | 0x36 | 12
pub struct _0x0036 {
    meta: &'static OpcodeMeta,
}

pub static _0x0036_: _0x0036 = _0x0036 {
    meta: &META,
};

impl Opcode for _0x0036 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        let left = cpu.register.get_u16(Register::HL);
let right = cpu.imm_u8();
// no flag effect
cpu.memory.borrow_mut().set_u8(left, right);

    }
}
