use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x55,
    group: "x8/lsm",
    parameters: [Some("D"), Some("L")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | D,L | 0x55 | 4
pub struct _0x0055 {
    meta: &'static OpcodeMeta,
}

pub static _0x0055_: _0x0055 = _0x0055 {
    meta: &META,
};

impl Opcode for _0x0055 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        let right = cpu.register.get_l();
cpu.register.set_d(right);
    }
}