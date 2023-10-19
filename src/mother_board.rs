use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::{cartridge, cpu, GBTerm};
use crate::cpu::CPU;
use crate::mmu::{Memory, MMU};

pub struct MotherBoard {
    cpu: Rc<RefCell<Box<dyn CPU>>>,
    mmu: Rc<RefCell<Box<dyn Memory>>>,
}

impl MotherBoard {
    pub fn new(rom_path: String, ram_path: String, rtc_path: String) -> crate::Result<MotherBoard> {
        let mmu = MMU::new(Rc::new(RefCell::new(cartridge::power_up(rom_path, ram_path, rtc_path)?)));
        let cpu = cpu::lr35902::LR35902::new(
            GBTerm::GB,
            Rc::clone(&mmu),
        );
        Ok(MotherBoard {
            cpu: cpu,
            mmu: mmu,
        })
    }

    pub fn cpu(&self) -> Ref<Box<dyn CPU>> {
        let cpu = self.cpu.borrow();
        cpu
    }

    pub fn start(&self) {
        loop {
            self.cpu.borrow_mut().run()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mother_board::MotherBoard;

    #[test]
    pub fn test_mother_board() {
        let rom_path = String::from("resources/cartridge/cpu_instrs.gb");
        let ram_path = String::from("target/_ram");
        let rtc_path = String::from("target/_rtc");
        let mb = MotherBoard::new(rom_path, ram_path, rtc_path).unwrap();
        mb.start()
    }
}