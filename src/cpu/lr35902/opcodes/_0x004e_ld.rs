use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x4E,
    group: "x8/lsm",
    parameters: [Some("C"), Some("(HL)")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | C,(HL) | 0x4E | 8
pub struct _0x004E {
    meta: &'static OpcodeMeta,
}

pub static _0x004E_: _0x004E = _0x004E {
    meta: &META,
};

impl Opcode for _0x004E {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        
let right = cpu.register.get_hl();
let right = cpu.memory.borrow().get(right);
cpu.register.set_c(right);

    }
}
