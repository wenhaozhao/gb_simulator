use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "RR",
    length: 2,
    cycles: [16, 0],
    flags: [FlagEffect::Fun(Flag::Z), FlagEffect::Reset(Flag::N), FlagEffect::Reset(Flag::H), FlagEffect::Fun(Flag::C)],
    addr: 0x1E,
    group: "x8/rsb",
    parameters: [Some("(HL)"), None],
    cb_prefixed: true,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// RR | (HL) | 0x1E | 16
pub struct _0xCB1E {
    meta: &'static OpcodeMeta,
}

pub static _0xCB1E_: _0xCB1E = _0xCB1E {
    meta: &META,
};

impl Opcode for _0xCB1E {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
