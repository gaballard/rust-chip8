use sdl2::pixels::Color;

use crate::constants;

///
/// Emulator Config
///
#[derive(Debug, Clone)]
pub struct EmulatorConfig {
    pub schip_mode: bool,
    pub quirks_mode: bool,
    pub debug_mode: bool,
    pub step_mode: bool,
    pub target_clock_speed: usize, // hz
    pub max_rom_size: usize,
}

impl Default for EmulatorConfig {
    fn default() -> Self {
        Self {
            schip_mode: constants::SCHIP_MODE,
            quirks_mode: constants::QUIRKS_MODE,
            debug_mode: false,
            step_mode: false,
            target_clock_speed: constants::TARGET_CLOCK_SPEED, // hz
            max_rom_size: constants::MAX_ROM_SIZE,
        }
    }
}

///
/// Renderer Config
///
#[derive(Debug, Clone)]
pub struct RendererConfig {
    pub video_scale: usize,
    pub foreground_color: Color,
    pub background_color: Color,
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            video_scale: constants::VIDEO_SCALE,
            foreground_color: constants::FOREGROUND_COLOR,
            background_color: constants::BACKGROUND_COLOR,
        }
    }
}
