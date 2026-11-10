use sdl2::pixels::Color;

use crate::InstructionSet;

// Interpreter
pub const EMULATOR_NAME: &str = "Chippy";
pub const MAX_ROM_SIZE: u32 = 3585;
pub const DEFAULT_INSTRUCTION_SET: InstructionSet = InstructionSet::Schip1point1;

// Display
pub const HIRES_SCREEN_WIDTH: usize = 128;
pub const HIRES_SCREEN_HEIGHT: usize = 64;
pub const LORES_SCREEN_WIDTH: usize = 64;
pub const LORES_SCREEN_HEIGHT: usize = 32;
pub const VIDEO_SCALE: u32 = 12;

// Fonts
pub const FONT_SIZE: u16 = 16;
pub const FONT_PATH: &str = "./assets/fonts/SperryPC_CGA.ttf";

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
pub const DEFAULT_PALETTE: usize = 2;

// Audio
pub const BEEP_FREQ_HZ: i32 = 440 * 8;

// CPU
pub const DEFAULT_CLOCK_SPEED: u32 = 120;
pub const LORES_CLOCK_SPEED: u32 = 60; // SCHIP mode
pub const HIRES_CLOCK_SPEED: u32 = 240; // SCHIP mode
pub const SYSTEM_RAM: u16 = 4096;
pub const OPCODE_SIZE: u16 = 2;

// Memory
pub const FONT_START_ADDR: u16 = 0x50;
pub const LARGE_FONT_START_ADDR: u16 = 0x50;
pub const PROGRAM_START_ADDR: u16 = 0x200;
