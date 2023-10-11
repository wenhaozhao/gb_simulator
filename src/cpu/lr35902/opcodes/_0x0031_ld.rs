use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 3,
    cycles: [12, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x31,
    group: "x16/lsm",
    parameters: [Some("SP"), Some("d16")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | SP,d16 | 0x31 | 12
pub struct _0x0031 {
    meta: &'static OpcodeMeta,
}

pub static _0x0031_: _0x0031 = _0x0031 {
    meta: &META,
};

impl Opcode for _0x0031 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        let left = 0xFF00 | (cpu.memory.borrow().get(cpu.register.get_d16() ) as u16);
let right = cpu.imm_u16();
cpu.memory.borrow_mut().set_u16(left, right);

    }
}
