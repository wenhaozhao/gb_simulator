use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 3,
    cycles: [16, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xEA,
    group: "x8/lsm",
    parameters: [Some("(a16)"), Some("A")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | (a16),A | 0xEA | 16
pub struct _0x00EA {
    meta: &'static OpcodeMeta,
}

pub static _0x00EA_: _0x00EA = _0x00EA {
    meta: &META,
};

impl Opcode for _0x00EA {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        let left = cpu.imm_u16();
let right = cpu.register.get_u8(Register::A);

cpu.memory.borrow_mut().set_u8(left, right);

    }
}
