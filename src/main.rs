use std::cell::RefCell;
use std::rc::Rc;

use gb_simulator::{cartridge, cpu, GBTerm};
use gb_simulator::cartridge::Cartridge;
use gb_simulator::cpu::CPU;
use gb_simulator::mmu::Memory;

fn main() -> ! {
    let rom_path = String::from("resources/cartridge/cpu_instrs.gb");
    let ram_path = String::from("target/_ram");
    let rtc_path = String::from("target/_rtc");
    let cart = cartridge::power_up(rom_path, ram_path, rtc_path).unwrap();
    let title_text = cart.title_text().unwrap();
    println!(" rom title_text => {}", title_text);
    println!(" rom manufacturer_code => {:02X?}", cart.manufacturer_code());
    let info = cart.info();
    println!("{}", info);

    let mem_ref: Rc<RefCell<Box<dyn Memory>>> = Rc::new(RefCell::new(Box::new(TestMemory { cart })));
    let mut cpu = cpu::lr35902::LR35902::new(GBTerm::GB, Rc::clone(&mem_ref));
    loop {
        cpu.run();
    }
}

struct TestMemory {
    cart: Box<dyn Cartridge>,
}

impl Memory for TestMemory {
    fn get(&self, i: u16) -> u8 {
        self.cart.get(i)
    }

    fn set(&mut self, i: u16, v: u8) {
        self.cart.set(i, v)
    }
}
