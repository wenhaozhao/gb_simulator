use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x7E,
    group: "x8/lsm",
    parameters: [Some("A"), Some("(HL)")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | A,(HL) | 0x7E | 8
pub struct _0x007E {
    meta: &'static OpcodeMeta,
}

pub static _0x007E_: _0x007E = _0x007E {
    meta: &META,
};

impl Opcode for _0x007E {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        
let right = cpu.memory.borrow().get(cpu.register.get_u16(Register::HL));
// no flag effect
cpu.register.set_u8(Register::A, right);

    }
}
