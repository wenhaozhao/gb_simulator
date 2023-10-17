use crate::cpu::lr35902::LR35902;

impl LR35902 {
    pub fn cbprefixed_exec_opcode(&mut self, opcode: u8) -> u8 {
        match opcode {
            0x00 => {

            }
            other => panic!(format!("Unsupported opcode:{}", other)),
        }
    }
}
