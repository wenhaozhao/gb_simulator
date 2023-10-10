use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "INC",
    length: 1,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x33,
    group: "x16/alu",
    parameters: [Some("SP"), None],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// INC | SP | 0x33 | 8
pub struct _0x0033 {
    meta: &'static OpcodeMeta,
}

pub static _0x0033_: _0x0033 = _0x0033 {
    meta: &META,
};

impl Opcode for _0x0033 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}