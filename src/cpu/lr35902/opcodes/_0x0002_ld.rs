use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x02,
    group: "x8/lsm",
    parameters: [Some("(BC)"), Some("A")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | (BC),A | 0x02 | 8
pub struct _0x0002 {
    meta: &'static OpcodeMeta,
}

pub static _0x0002_: _0x0002 = _0x0002 {
    meta: &META,
};

impl Opcode for _0x0002 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        let left = cpu.register.get_u16(Register::BC);
let right = cpu.register.get_u8(Register::A);
// no flag effect
cpu.memory.borrow_mut().set_u8(left, right);

    }
}
