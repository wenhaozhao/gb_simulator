use crate::cpu::CPU;

mod unprefixed_opcode;
mod cbprefixed_opcode;

pub const CB_PREFIXED: u16 = 0xCB00;
