use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "SLA",
    length: 2,
    cycles: [16, 0],
    flags: [FlagEffect::Fun(Flag::Z), FlagEffect::Reset(Flag::N), FlagEffect::Reset(Flag::H), FlagEffect::Fun(Flag::C)],
    addr: 0x26,
    group: "x8/rsb",
    parameters: [Some("(HL)"), None],
    cb_prefixed: true,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// SLA | (HL) | 0x26 | 16
pub struct _0xCB26 {
    meta: &'static OpcodeMeta,
}

pub static _0xCB26_: _0xCB26 = _0xCB26 {
    meta: &META,
};

impl Opcode for _0xCB26 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}