use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "BIT",
    length: 2,
    cycles: [8, 0],
    flags: [FlagEffect::Fun(Flag::Z), FlagEffect::Reset(Flag::N), FlagEffect::Set(Flag::H), FlagEffect::None],
    addr: 0x43,
    group: "x8/rsb",
    parameters: [Some("0"), Some("E")],
    cb_prefixed: true,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// BIT | 0,E | 0x43 | 8
pub struct _0xCB43 {
    meta: &'static OpcodeMeta,
}

pub static _0xCB43_: _0xCB43 = _0xCB43 {
    meta: &META,
};

impl Opcode for _0xCB43 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) -> u8 {
        todo!()
    }
}
