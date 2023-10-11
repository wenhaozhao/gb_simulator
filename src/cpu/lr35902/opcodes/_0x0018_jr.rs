use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "JR",
    length: 2,
    cycles: [12, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x18,
    group: "control/br",
    parameters: [Some("r8"), None],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// JR | r8 | 0x18 | 12
pub struct _0x0018 {
    meta: &'static OpcodeMeta,
}

pub static _0x0018_: _0x0018 = _0x0018 {
    meta: &META,
};

impl Opcode for _0x0018 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) -> u8 {
        let left = cpu.imm_u8() as i8;
cpu.register.pc_incr_by_i8(left);
self.meta.cycles[0]
    }
}
