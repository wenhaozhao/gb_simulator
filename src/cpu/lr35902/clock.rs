use std::thread;
use std::time::{Duration, SystemTime};

use crate::cpu::lr35902::FREQ;
use crate::FPS;

/// frame duration in millis 16ms
const FRAME_DURATION: u64 = 1000 / FPS;
/// Cycles per frame 67108
const CYCLES_PER_FRAME: u64 = (FREQ as f64 / 1000f64 * (FRAME_DURATION as f64)) as u64;

#[derive(Debug)]
pub struct Clock {
    current_time: SystemTime,
    step_cycles: u64,
}

pub struct ClockStep {}

impl Clock {
    pub fn new() -> Self {
        Clock {
            current_time: SystemTime::now(),
            step_cycles: 0,
        }
    }

    pub fn step(&mut self, cycles: u8) -> crate::Result<()> {
        let step_cycles = self.step_cycles + cycles as u64;
        if step_cycles >= CYCLES_PER_FRAME {
            // padding to a frame duration
            let elapse = SystemTime::now().duration_since(self.current_time)
                .map_err(|err| format!("clock err: {:?}", err))?.as_millis() as u64;
            let padding = FRAME_DURATION.saturating_sub(elapse);
            thread::sleep(Duration::from_millis(padding));
            self.step_cycles -= CYCLES_PER_FRAME;
            self.current_time = self.current_time.checked_add(Duration::from_millis(FRAME_DURATION)).unwrap();
        }
        Ok(())
    }

    pub fn now(&self) -> SystemTime {
        self.current_time
    }
}
