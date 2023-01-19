use sdl2::pixels::Color;

// Emulator
pub const EMULATOR_NAME: &str = "Wow! Another CHIP-8 Emulator!";
pub const MAX_ROM_SIZE: usize = 3585;

// Display
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const VIDEO_SCALE: usize = 12;
pub const FOREGROUND_COLOR: Color = Color::GREEN;
pub const BACKGROUND_COLOR: Color = Color::BLACK;

// Sound
pub const BEEP_FREQ_HZ: u16 = 440;

// CPU
pub const TARGET_CLOCK_SPEED: usize = 60;
pub const SYSTEM_RAM: u16 = 4096;
pub const OPCODE_SIZE: u16 = 2;

// Memory
pub const FONT_START_ADDR: u16 = 0x50;
pub const PROGRAM_START_ADDR: u16 = 0x200;

// Videp
pub const MAX_SPRITE_WIDTH: usize = 8;
pub const MAX_SPRITE_HEIGHT: usize = 15;
