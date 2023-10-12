use std::cell::RefCell;
use std::ops::Sub;
use std::thread;
use std::time::{Duration, SystemTime};

use opcode::OPCODE_PREFIX_CB as PREFIX_CB;

use crate::cpu::CPU;
use crate::cpu::lr35902::opcode::Opcode;
use crate::cpu::lr35902::registers::Registers;
use crate::mmu::Memory;

mod registers;
mod opcode;
mod opcodes;

#[derive(Debug)]
pub struct Clock {
    zero_at: SystemTime,
    step_zero_at: SystemTime,
    step_cycles: u64,
}

impl Clock {
    fn new() -> Self {
        Clock {
            zero_at: SystemTime::now(),
            step_zero_at: SystemTime::now(),
            step_cycles: 0,
        }
    }
}

impl Clock {
    const FEQ: u64 = 0x0040_0000;
    const FPS: u64 = 60;
    // 60fps a good game value

    const STEP_DUR: u64 = Duration::from_secs(1).as_millis() as u64 / Clock::FPS;

    const FRAME_SPEED: u64 = Duration::from_secs(1).as_millis() as u64 / Clock::FPS; //16ms per frame

    /// duration per frame in cycles
    const FIC: u64 = Clock::FEQ / Clock::FPS;
}

pub struct LR35902 {
    clock: Clock,
    opcodes: &'static [Option<&'static dyn Opcode>; 0x0200],
    register: Registers,
    memory: RefCell<Box<dyn Memory>>,
}

impl LR35902 {
    fn imm_u8(&mut self) -> u8 {
        let addr = self.register.pc_get_and_incr();
        let val = self.memory.borrow().get(addr);
        val
    }

    fn imm_u16(&mut self) -> u16 {
        let addr = self.register.pc_get_and_decr_by(0x02);
        let vec = self.memory.borrow().gets(addr, 0x02);
        let mut bytes = [00u8; 2];
        bytes.copy_from_slice(&vec);
        u16::from_le_bytes(bytes)
    }
}

// 4MHz
impl LR35902 {
    fn sleep(&self, duration: Duration) {}
}

impl CPU for LR35902 {
    fn run(&mut self) {
        let opcode_addr = self.imm_u8() as u16;
        let actual_opcode_addr = if opcode_addr == PREFIX_CB {
            PREFIX_CB | (self.imm_u8() as u16)
        } else {
            opcode_addr
        };
        self.actual_run(actual_opcode_addr);
    }
}

impl LR35902 {
    fn actual_run(&mut self, actual_opcode_addr: u16) {
        let step_cycles = self.clock.step_cycles;
        if step_cycles >= Clock::FIC {
            // sleep
            let elapse = SystemTime::now().duration_since(self.clock.step_zero_at).unwrap().as_millis() as u64;
            let sleep = Clock::STEP_DUR.saturating_sub(elapse);
            thread::sleep(Duration::from_millis(sleep));
            self.clock.step_cycles -= Clock::FIC;
            self.clock.step_zero_at = self.clock.step_zero_at.checked_add(Duration::from_millis(Clock::STEP_DUR)).unwrap();
            #[cfg(test)]
            {
                println!("after sleep, step_cycles:{}, elapse:{}ms, sleep:{}ms, => {:?}", step_cycles, elapse, sleep, self.clock);
            }
        }
        let opcode = opcodes::get_opcode(actual_opcode_addr)
            .expect(format!("Unsupported opcode {:04X}", actual_opcode_addr).as_str());
        let cycles = opcode.exec(self);
        self.clock.step_cycles += cycles as u64;
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::cpu::lr35902::{Clock, LR35902};
    use crate::cpu::lr35902::opcodes::OPCODES;
    use crate::cpu::lr35902::registers::Registers;

    #[test]
    pub fn test_actual_run() {
        let mut cpu = LR35902 {
            clock: Clock::new(),
            opcodes: &OPCODES,
            register: Registers::default(),
            memory: RefCell::new(Box::new(crate::mmu::tests::TestMemory::new())),// RefCell<Box<dyn Memory>>
        };
        let actual_opcode_addr = 0x0020u16;
        let mut count = 100000;
        loop {
            cpu.actual_run(actual_opcode_addr);
            count -= 1;
            if count <= 0 {
                return;
            }
        }
    }
}