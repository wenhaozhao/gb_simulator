pub trait Memory {
    fn read(&self, i: u16) -> u8;

    fn write(&mut self, i: u16, v: u8);

    fn writes(&mut self,i: u16, bytes: &Vec<u8>) {
        let mut i = i;
        for v in bytes {
            self.write(i, *v);
            i+=1;
        }
    }
}

#[cfg(test)]
mod memory_tests {}