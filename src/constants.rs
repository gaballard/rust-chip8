use sdl2::pixels::Color;

// Emulator
pub const EMULATOR_NAME: &str = "Wow! Another CHIP-8 Emulator!";
pub const MAX_ROM_SIZE: u32 = 3585;

// Display
pub const _HIRES_SCREEN_WIDTH: usize = 128;
pub const _HIRES_SCREEN_HEIGHT: usize = 64;
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
pub const VIDEO_SCALE: u32 = 14;
pub const PIXEL_FADE_RATE_SLOW: u8 = 3;
pub const PIXEL_FADE_RATE_MED: u8 = 5;
pub const PIXEL_FADE_RATE_FAST: u8 = 7;

// Fonts
pub const FONT_SIZE: u16 = 16;
pub const FONT_PATH: &str = "./fonts/SperryPC_CGA.ttf";

// Colors
pub const FOREGROUND_COLOR_AMBER: Color = Color::RGB(255, 176, 0); // Amber (P3)
pub const FOREGROUND_COLOR_GREEN: Color = Color::RGB(51, 255, 51); // Green (P1)
pub const BACKGROUND_COLOR: Color = Color::RGB(40, 40, 40);
pub const PALETTE_COLORS: [(Color, Color); 2] = [
    (FOREGROUND_COLOR_AMBER, BACKGROUND_COLOR),
    (FOREGROUND_COLOR_GREEN, BACKGROUND_COLOR),
];

// Audio
pub const BEEP_FREQ_HZ: i32 = 440 * 8;

// CPU
pub const SCHIP_MODE: bool = false;
pub const TARGET_CLOCK_SPEED: u32 = 120;
pub const SYSTEM_RAM: u16 = 4096;
pub const OPCODE_SIZE: u16 = 2;

// Memory
pub const FONT_START_ADDR: u16 = 0x50;
pub const LARGE_FONT_START_ADDR: u16 = 0x50;
pub const PROGRAM_START_ADDR: u16 = 0x200;
