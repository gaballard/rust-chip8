use sdl2::pixels::Color;

// Interpreter
pub const EMULATOR_NAME: &str = "Chippy";

// System
pub const MAX_ROM_SIZE: u32 = 3585; // bytes
pub const OPCODE_SIZE: u16 = 2;
pub const SYSTEM_RAM: u16 = 4096; // bytes
pub const FONT_START_ADDR: u16 = 0x50;
pub const LARGE_FONT_START_ADDR: u16 = 0x50;
pub const PROGRAM_START_ADDR: u16 = 0x200;

// Display
pub const HIRES_SCREEN_WIDTH: usize = 128;
pub const HIRES_SCREEN_HEIGHT: usize = 64;
pub const LORES_SCREEN_WIDTH: usize = 64;
pub const LORES_SCREEN_HEIGHT: usize = 32;

// Colors
pub const FOREGROUND_COLOR_WHITE: Color = Color::RGB(215, 215, 215);
pub const FOREGROUND_COLOR_AMBER: Color = Color::RGB(255, 176, 0); // Amber (P3)
pub const FOREGROUND_COLOR_GREEN: Color = Color::RGB(51, 255, 51); // Green (P1)
pub const BACKGROUND_COLOR: Color = Color::RGB(40, 40, 40);
pub const PALETTE_COLORS: [(Color, Color); 3] = [
    (FOREGROUND_COLOR_WHITE, BACKGROUND_COLOR),
    (FOREGROUND_COLOR_AMBER, BACKGROUND_COLOR),
    (FOREGROUND_COLOR_GREEN, BACKGROUND_COLOR),
];
