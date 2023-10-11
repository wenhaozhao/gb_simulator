use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::opcode::{FlagEffect, Opcode, OpcodeMeta};
use crate::cpu::lr35902::registers::Flag;

static META: OpcodeMeta = OpcodeMeta {
    mnemonic: "LD",
    length: 1,
    cycles: [8, 0],
    flags: [FlagEffect::None, FlagEffect::None, FlagEffect::None, FlagEffect::None],
    addr: 0x73,
    group: "x8/lsm",
    parameters: [Some("(HL)"), Some("E")],
    cb_prefixed: false,
};

/// Instruction | Parameters | Opcode | Cycles
/// ----------- | ---------- | ------ | ------
/// LD | (HL),E | 0x73 | 8
pub struct _0x0073 {
    meta: &'static OpcodeMeta,
}

pub static _0x0073_: _0x0073 = _0x0073 {
    meta: &META,
};

impl Opcode for _0x0073 {

    fn meta(&self) -> &'static OpcodeMeta {
        self.meta
    }

    fn exec(&self, cpu: &mut LR35902) {
        let left = 0xFF00 | (cpu.memory.borrow().get(cpu.register.get_e() as u16) as u16);
let right = cpu.register.get_e();
cpu.memory.borrow_mut().set_u8(left, right);

    }
}
