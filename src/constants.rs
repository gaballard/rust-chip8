use sdl2::pixels::Color;

// Emulator
pub const EMULATOR_NAME: &str = "Wow! Another CHIP-8 Emulator!";
pub const MAX_ROM_SIZE: usize = 3585;
pub const ROM_FOLDER: &str = "./roms";

// Display
pub const _HIRES_SCREEN_WIDTH: usize = 128;
pub const _HIRES_SCREEN_HEIGHT: usize = 64;
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const VIDEO_SCALE: usize = 12;
pub const FOREGROUND_COLOR: Color = Color::RGB(255, 176, 0); // Amber
pub const BACKGROUND_COLOR: Color = Color::BLACK;

// Audio
pub const BEEP_FREQ_HZ: i32 = 440 * 8;

// CPU
pub const SCHIP_MODE: bool = false;
pub const QUIRKS_MODE: bool = false;
pub const TARGET_CLOCK_SPEED: usize = 120;
pub const SYSTEM_RAM: u16 = 4096;
pub const OPCODE_SIZE: u16 = 2;

// Memory
pub const FONT_START_ADDR: u16 = 0x50;
pub const LARGE_FONT_START_ADDR: u16 = 0x50;
pub const PROGRAM_START_ADDR: u16 = 0x200;
