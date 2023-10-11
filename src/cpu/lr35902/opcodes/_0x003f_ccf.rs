use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "CCF",
    length: 1,
    cycles: [4, 0],
    flags: [FlagEffect::None, FlagEffect::Reset(Flag::N), FlagEffect::Reset(Flag::H), FlagEffect::Fun(Flag::C)],
    addr: 0x3F,
    group: "x8/alu",
    parameters: [None, None],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// CCF |  | 0x3F | 4
pub struct _0x003F {
    meta: &'static OpcodeMeta,
}

pub static _0x003F_: _0x003F = _0x003F {
    meta: &META,
};

impl Opcode for _0x003F {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
