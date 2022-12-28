pub const EMULATOR_NAME: &str = "gEMU Chip-8 Emulator";
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const VIDEO_SCALE: usize = 4;

pub const TARGET_CLOCK_SPEED: usize = 60;
pub const MAX_ROM_SIZE: usize = 3585;
pub const SYSTEM_RAM: u16 = 4096;

pub const FONT_START_ADDR: u16 = 0x50;
pub const PROGRAM_START_ADDR: u16 = 0x200;
pub const OPCODE_SIZE: u16 = 2;
