use crate::cpu::CPU;
use crate::cpu::lr35902::registers::Registers;

mod registers;


pub struct LR35902 {
    register: Registers,
}

impl CPU for LR35902 {}