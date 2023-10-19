use std::cell::RefCell;
use std::rc::Rc;

use crate::{cpu, GBTerm};
use crate::cpu::CPU;
use crate::io_device::cartridge;
use crate::mmu::MMU;
use crate::mother_board::MotherBoard;

#[test]
fn test_cpu_instrs() -> ! {
    let rom_path = String::from("resources/cartridge/cpu_instrs.gb");
    let ram_path = String::from("target/_ram");
    let rtc_path = String::from("target/_rtc");
    let mut mb = MotherBoard::new(rom_path, ram_path, rtc_path).unwrap();
    mb.start()
}