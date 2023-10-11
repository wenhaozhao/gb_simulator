use std::mem;

use proc_macros::U16FieldAccessor;

use crate::cpu::{get_hi, get_lo, set_hi, set_lo};

/// ## Registers
///
/// 16-bit |Hi |Lo | Name/Function
/// -------|---|---|--------------
///    AF  | A | - | Accumulator & Flags
///    BC  | B | C | BC
///    DE  | D | E | DE
///    HL  | H | L | HL
///    SP  | - | - | Stack Pointer
///    PC  | - | - | Program Counter/Pointer
/// #### [see pandoc](https://gbdev.io/pandocs/CPU_Registers_and_Flags.html)
#[repr(C)]
#[derive(U16FieldAccessor)]
pub struct Registers {
    /// Accumulator & Flags
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    /// Stack Pointer
    sp: u16,
    /// Program Counter/Pointer
    pc: u16,
}

impl Default for Registers {
    fn default() -> Self {
        unsafe { mem::transmute::<[u16; 6], Registers>([0u16; 6]) }
    }
}

impl Registers {
    pub fn new() -> Self {
        Default::default()
    }
}

/// General registers
impl Registers {

    #[inline]
    pub fn get_a(&self) -> u8 {
        get_hi(&self.af)
    }

    #[inline]
    pub fn set_a(&mut self, val: u8) {
        set_hi(&mut self.af, val);
    }

    #[inline]
    pub fn get_f(&self) -> u8 {
        get_lo(&self.af)
    }

    #[inline]
    pub fn set_f(&mut self, val: u8) {
        set_lo(&mut self.af, val);
    }
}

impl Registers {

    #[inline]
    pub fn get_b(&self) -> u8 {
        get_hi(&self.bc)
    }

    #[inline]
    pub fn set_b(&mut self, val: u8) {
        set_hi(&mut self.bc, val);
    }

    #[inline]
    pub fn get_c(&self) -> u8 {
        get_lo(&self.bc)
    }

    #[inline]
    pub fn set_c(&mut self, val: u8) {
        set_lo(&mut self.bc, val);
    }
}

impl Registers {

    #[inline]
    pub fn get_d(&self) -> u8 {
        get_hi(&self.de)
    }

    #[inline]
    pub fn set_d(&mut self, val: u8) {
        set_hi(&mut self.de, val);
    }

    #[inline]
    pub fn get_e(&self) -> u8 {
        get_lo(&self.de)
    }

    #[inline]
    pub fn set_e(&mut self, val: u8) {
        set_lo(&mut self.de, val);
    }
}

impl Registers {

    #[inline]
    pub fn get_h(&self) -> u8 {
        get_hi(&self.hl)
    }

    #[inline]
    pub fn set_h(&mut self, val: u8) {
        set_hi(&mut self.hl, val);
    }

    #[inline]
    pub fn get_l(&self) -> u8 {
        get_lo(&self.hl)
    }

    #[inline]
    pub fn set_l(&mut self, val: u8) {
        set_lo(&mut self.hl, val);
    }
}


///
/// ## The Flags Register (lower 8 bits of AF register)
///
/// Bit | Name | Explanation
/// ----|------|-------
///   7 |   z  | Zero flag
///   6 |   n  | Subtraction flag (BCD)
///   5 |   h  | Half Carry flag (BCD)
///   4 |   c  | Carry flag
pub enum Flag {
    /// Bit | Name | Explanation
    /// ----|------|-------
    ///   7 |   z  | Zero flag
    Z = 0x0080,
    /// Bit | Name | Explanation
    /// ----|------|-------
    ///   6 |   n  | Subtraction flag (BCD)
    N = 0x0040,
    /// Bit | Name | Explanation
    /// ----|------|-------
    ///   5 |   h  | Half Carry flag (BCD)
    H = 0x0020,
    /// Bit | Name | Explanation
    /// ----|------|-------
    ///   4 |   c  | Carry flag
    C = 0x0010,
}

/// Flag Registers
impl Registers {
    #[inline]
    pub fn get_flags(&self) -> u8 {
        self.get_f()
    }

    #[inline]
    pub fn set_flags(&mut self, val: u8) {
        self.set_f(val);
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        self.af & (flag as u16) > 0
    }

    #[inline]
    pub fn set_flag(&mut self, flag: Flag, val: bool) {
        if val {
            self.af = self.af | (flag as u16);
        } else {
            self.af = self.af & (!(flag as u16));
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::cpu::lr35902::registers::Registers;

    #[test]
    pub fn test_bc() {
        let mut registers: Registers = Default::default();
        registers.set_bc(0x0001);
        let bc = registers.get_bc();
        assert_eq!(bc, 0x0001);
        registers.set_bc(0x0002);
        let bc = registers.get_bc();
        assert_eq!(bc, 0x0002);
    }

    #[test]
    pub fn test_bc_lo() {
        let mut registers: Registers = Default::default();
        registers.set_bc(0x1001);
        registers.set_bc_lo(0x02);
        let bc = registers.get_bc();
        assert_eq!(bc, 0x1002);
        let bc_lo = registers.get_bc_lo();
        assert_eq!(bc_lo, 0x02);
    }

    #[test]
    pub fn test_bc_hi() {
        let mut registers: Registers = Default::default();
        registers.set_bc(0x1001);
        registers.set_bc_hi(0x20);
        let bc = registers.get_bc();
        assert_eq!(bc, 0x2001);
        let bc_lo = registers.get_bc_hi();
        assert_eq!(bc_lo, 0x20);
    }

}