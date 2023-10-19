use std::cell::RefCell;
use std::rc::Rc;
use std::time::SystemTime;

use crate::mmu::{Memory, RefMemory};

pub mod lr35902;

pub type RefCPU = Rc<RefCell<Box<dyn CPU>>>;

pub trait CPU {
    fn memory(&self) -> RefMemory;

    fn run(&mut self);

    fn info(&self) -> &CPUInfo;
}

pub struct CPUInfo {
    start_at: SystemTime,
    freq: u64,
}

impl CPUInfo {
    pub fn new(freq: u64) -> Self {
        CPUInfo {
            start_at: SystemTime::now(),
            freq,
        }
    }
}


#[inline]
pub fn get_hi(src: &u16) -> u8 {
    (*src >> 8) as u8
}

#[inline]
pub fn set_hi(src: &mut u16, val: u8) {
    *src = (*src & 0x00FF) | ((val as u16) << 8);
}

#[inline]
pub fn get_lo(src: &u16) -> u8 {
    *src as u8
}

#[inline]
pub fn set_lo(src: &mut u16, val: u8) {
    *src = (*src & 0xFF00) | (val as u16);
}

pub fn get_bit(src: &u16, i: u8) -> u8 {
    assert!(i < 8);
    ((src >> i) & 0x01) as u8
}

pub fn set_bit(src: &mut u16, i: u8, val: u8) {
    assert!(i < 8);
    if val > 0 {
        *src = *src | (0x01 << i);
    } else {
        *src = *src & (!(0x01 << i));
    }
}
