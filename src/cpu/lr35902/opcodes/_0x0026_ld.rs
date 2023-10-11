use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::{Flag, Register};

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 2,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x26,
    group: "x8/lsm",
    parameters: [Some("H"), Some("d8")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | H,d8 | 0x26 | 8
pub struct _0x0026 {
    meta: &'static OpcodeMeta,
}

pub static _0x0026_: _0x0026 = _0x0026 {
    meta: &META,
};

impl Opcode for _0x0026 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        
let right = cpu.imm_u8();

cpu.register.set_u8(Register::H, right);

    }
}
