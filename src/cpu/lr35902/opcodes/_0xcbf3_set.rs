use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "SET",
    length: 2,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0xF3,
    group: "x8/rsb",
    parameters: [Some("6"), Some("E")],
    cb_prefixed: true,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// SET | 6,E | 0xF3 | 8
pub struct _0xCBF3 {
    meta: &'static OpcodeMeta,
}

pub static _0xCBF3_: _0xCBF3 = _0xCBF3 {
    meta: &META,
};

impl Opcode for _0xCBF3 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        todo!()
    }
}
