use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "SCF",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::None, FlagEffect::Reset(Flag::N), FlagEffect::Reset(Flag::H), FlagEffect::Set(Flag::C)],
    addr: 0x37,
    group: "x8/alu",
    parameters: [None, None],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// SCF |  | 0x37 | 4
pub struct _0x0037 {
    meta: &'static OpcodeMeta,
}

pub static _0x0037_: _0x0037 = _0x0037 {
    meta: &META,
};

impl Opcode for _0x0037 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
