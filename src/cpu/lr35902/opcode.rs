use crate::cpu::lr35902::LR35902;
use crate::cpu::lr35902::registers::Flag;

pub const OPCODE_PREFIX_CB: u16 = 0x00CB;

pub struct OpcodeMeta {
    pub mnemonic: &'static str,
    pub length: u8,
    pub cycles: [u8; 2],
    pub flags: [FlagEffect; 4],
    pub addr: u8,
    pub group: &'static str,
    pub parameters: [Option<&'static str>; 2],
    pub cb_prefixed: bool,
}

impl OpcodeMeta {}

pub trait Opcode: Send + Sync {
    fn meta(&self) -> &'static OpcodeMeta;

    /// exec
    /// return Duration in cycles
    fn exec(&self, cpu: &mut LR35902);
}

pub enum FlagEffect {
    /// 无影响
    None,
    /// 指令之后它被重置
    Reset(Flag),
    /// 已设置
    Set(Flag),
    /// 相应的标志将受到其功能的预期影响
    Fun(Flag),
}

impl FlagEffect {
    pub fn effect(&self, cpu: &mut LR35902, l: isize, r: isize) {
        match self {
            FlagEffect::None => {}
            FlagEffect::Reset(flag) => cpu.register.set_flag(*flag, false),
            FlagEffect::Set(flag) => cpu.register.set_flag(*flag, true),
            FlagEffect::Fun(flag) => {
                match flag {
                    Flag::H => cpu.register.set_flag(Flag::H, (l & 0x000f) + (r & 0x000f) > 0x000f),
                    Flag::C => cpu.register.set_flag(Flag::C, (l & 0x00ff) + (r & 0x00ff) > 0x00ff),
                    _ => {}
                }
            }
        }
    }
}