pub trait Memory {
    fn read(&self, i: u16) -> u8;

    fn write(&mut self, i: u16, v: u8);
}

#[cfg(test)]
mod memory_tests {}