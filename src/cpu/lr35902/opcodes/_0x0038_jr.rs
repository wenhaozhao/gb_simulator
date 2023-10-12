use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "JR",
    length: 2,
    cycles: [12, 8],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x38,
    group: "control/br",
    parameters: [Some("CC"), Some("r8")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// JR | CC,r8 | 0x38 | 12/8
pub struct _0x0038 {
    meta: &'static OpcodeMeta,
}

pub static _0x0038_: _0x0038 = _0x0038 {
    meta: &META,
};

impl Opcode for _0x0038 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) -> u8 {
        let left = cpu.register.get_flag(Flag::C);
if left {
    let right = cpu.imm_u8() as i8;
    cpu.register.pc_incr_by_i8(right);
    return self.meta.cycles[0];
}
self.meta.cycles[1]
    }
}
