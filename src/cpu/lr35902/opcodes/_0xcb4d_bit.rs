use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "BIT",
    length: 2,
    cycles: [8, 0],
    flags: [FlagEffect::Fun(Flag::Z), FlagEffect::Reset(Flag::N), FlagEffect::Set(Flag::H), FlagEffect::None],
    addr: 0x4D,
    group: "x8/rsb",
    parameters: [Some("1"), Some("L")],
    cb_prefixed: true,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// BIT | 1,L | 0x4D | 8
pub struct _0xCB4D {
    meta: &'static OpcodeMeta,
}

pub static _0xCB4D_: _0xCB4D = _0xCB4D {
    meta: &META,
};

impl Opcode for _0xCB4D {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
