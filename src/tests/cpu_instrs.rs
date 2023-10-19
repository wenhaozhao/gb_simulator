use std::cell::RefCell;
use std::rc::Rc;

use crate::{cpu, GBTerm};
use crate::cpu::CPU;
use crate::io_device::cartridge;
use crate::mmu::MMU;

#[test]
fn test_cpu_instrs() -> ! {
    let rom_path = String::from("resources/cartridge/cpu_instrs.gb");
    let ram_path = String::from("target/_ram");
    let rtc_path = String::from("target/_rtc");
    let cart = cartridge::power_up(rom_path, ram_path, rtc_path).unwrap();
    let info = cart.info();
    println!("{}", info);
    let mut cpu = cpu::lr35902::LR35902::new(
        GBTerm::GB,
        MMU::new(Rc::new(RefCell::new(cart))),
    );
    loop {
        cpu.borrow_mut().run();
    }
}