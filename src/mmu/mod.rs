/// MMU

pub trait Memory {
    fn get(&self, i: u16) -> u8;

    fn gets(&self, i: u16, size: u16) -> Vec<u8> {
        let mut vec = Vec::with_capacity(size as usize);
        for j in 0..size {
            vec.push(self.get(i + j));
        }
        vec
    }

    fn set(&mut self, i: u16, v: u8);

    fn sets(&mut self, i: u16, bytes: &Vec<u8>) {
        let mut i = i;
        for v in bytes {
            self.set(i, *v);
            i += 1;
        }
    }
}

#[cfg(test)]
mod memory_tests {}