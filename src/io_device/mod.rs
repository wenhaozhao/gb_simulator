use crate::mmu::Memory;

pub mod joypad;
pub mod serial;
pub mod divider;
pub mod interrupt;
pub mod video;
pub mod infrared;
pub mod wram;
pub mod cartridge;


/// ## [P1/JOYP](https://gbdev.io/pandocs/Joypad_Input.html#ff00--p1joyp-joypad)
/// - $FF00	P1/JOYP	Joypad	Mixed	All
/// - [Joypad](joypad)
pub const IO_ADDR_JOYPAD: u16 = 0xFF00;

/// ## [VBK](https://gbdev.io/pandocs/CGB_Registers.html#ff4f--vbk-cgb-mode-only-vram-bank)
/// - 0xFF4F	VBK	VRAM bank	R/W	CGB
/// - [Video](video)
pub const IO_ADDR_VBK: u16 = 0xFF4F;

/// ## [IF](https://gbdev.io/pandocs/Interrupts.html#ff0f--if-interrupt-flag)
/// - 0xFF0F	IF	Interrupt flag	R/W	All
/// - [Interrupt](interrupt)
pub const IO_ADDR_INT_F: u16 = 0xFF0F;

/// ## [SVBK](https://gbdev.io/pandocs/CGB_Registers.html#ff70--svbk-cgb-mode-only-wram-bank)
/// - 0xFF70 SVBK	WRAM bank	R/W	CGB
/// - [WorkRam](wram)
pub const IO_ADDR_SVBK: u16 = 0xFF70;

/// ## [IE](https://gbdev.io/pandocs/Interrupts.html#ffff--ie-interrupt-enable)
/// - 0xFFFF	IE	Interrupt enable	R/W	All
/// - [Interrupt](interrupt)
pub const IO_ADDR_INT_E: u16 = 0xFFFF;
