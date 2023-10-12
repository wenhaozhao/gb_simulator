/// MMU

pub trait Memory {
    fn get(&self, i: u16) -> u8;

    fn get_u8(&self, i: u16) -> u8 {
        self.get(i)
    }

    fn get_u16(&self, i: u16) -> u16 {
        let vec = self.gets(i, 2);
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(&vec);
        u16::from_le_bytes(bytes)
    }

    fn gets(&self, i: u16, size: u16) -> Vec<u8> {
        let mut vec = Vec::with_capacity(size as usize);
        for j in 0..size {
            vec.push(self.get(i + j));
        }
        vec
    }

    fn set(&mut self, i: u16, v: u8);

    fn set_u8(&mut self, i: u16, v: u8) {
        self.set(i, v);
    }

    fn set_u16(&mut self, i: u16, val: u16) {
        let bytes = val.to_le_bytes();
        let vec = bytes.to_vec();
        self.sets(i, &vec);
    }

    fn sets(&mut self, i: u16, bytes: &Vec<u8>) {
        let mut i = i;
        for v in bytes {
            self.set(i, *v);
            i += 1;
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::mmu::Memory;

    pub struct TestMemory {
        mem: [u8; 0xFFFF],
    }

    impl TestMemory {
        pub fn new() -> Self {
            Self {
                mem: [0u8; 0xFFFF],
            }
        }
    }

    impl Memory for TestMemory {
        fn get(&self, i: u16) -> u8 {
            self.mem[i as usize]
        }

        fn set(&mut self, i: u16, v: u8) {
            self.mem[i as usize] = v;
        }
    }
}