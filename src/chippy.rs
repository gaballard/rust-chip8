use sdl2::{ttf::Font, TimerSubsystem};

use crate::{
    chip8::Cpu,
    constants,
    platform::{Audio, Display, Keypad, Platform, Tape},
};

pub enum PixelFadeRate {
    Slow,
    Medium,
    Fast,
}

#[allow(dead_code)]
pub enum ColorPalette {
    Amber = 0,
    Green = 1,
}

pub enum Actions {
    Quit,
    Reset,
    Running,
    ToggleDebugMode,
    ToggleDebugPanel,
    CycleColorPalette,
    Step,
    ToggleCrtFade,
    SetCrtFadeSpeed(PixelFadeRate),
}

impl Actions {
    pub fn handle_reset_action(chippy: &mut Chippy) {
        chippy.devices.cpu.reset();
        chippy.devices.cpu.load_program(&chippy.devices.tape.data);
        chippy.settings.debug_mode = false;
    }

    pub fn handle_toggle_debug_panel_action(chippy: &mut Chippy) {
        chippy.settings.show_debug_panel = !chippy.settings.show_debug_panel;
        chippy
            .devices
            .display
            .debug_panel
            .set_visibility(chippy.settings.show_debug_panel);
    }

    pub fn handle_cycle_color_palette_action(chippy: &mut Chippy) {
        chippy.settings.color_palette = chippy.settings.color_palette.wrapping_add(1);
        if chippy.settings.color_palette as usize >= constants::PALETTE_COLORS.len() {
            chippy.settings.color_palette = 0;
        }
        chippy
            .devices
            .display
            .set_color_palette(chippy.settings.color_palette);
        chippy
            .devices
            .display
            .debug_panel
            .set_color_palette(chippy.settings.color_palette);
    }

    pub fn handle_set_crt_fade_rate_action(chippy: &mut Chippy, fade_rate: PixelFadeRate) {
        chippy.settings.crt_fade_rate = match fade_rate {
            PixelFadeRate::Slow => constants::PIXEL_FADE_RATE_SLOW,
            PixelFadeRate::Medium => constants::PIXEL_FADE_RATE_MED,
            PixelFadeRate::Fast => constants::PIXEL_FADE_RATE_FAST,
        };
    }

    pub fn handle_toggle_debug_mode_action(chippy: &mut Chippy) {
        chippy.settings.debug_mode = !chippy.settings.debug_mode
    }

    pub fn handle_toggle_crt_fade_action(chippy: &mut Chippy) {
        chippy.settings.crt_fade = !chippy.settings.crt_fade
    }
}

pub struct Resources {
    pub frame_timer: TimerSubsystem,
}

#[allow(dead_code)]
pub struct Settings {
    pub rom_filename: String,
    pub crt_fade: bool,
    pub crt_fade_rate: u8,
    pub debug_mode: bool,
    pub show_debug_panel: bool,
    pub color_palette: usize,
    pub hires_mode: bool,
    pub schip_mode: bool,
}

pub struct HardwareDevices<'a> {
    pub tape: Tape,
    pub display: Display<'a>,
    pub audio: Audio,
    pub keypad: Keypad,
    pub cpu: Cpu,
}

pub struct Chippy<'a> {
    pub settings: Settings,
    pub resources: Resources,
    pub devices: HardwareDevices<'a>,
}

///
/// Chippy The Program
///
impl<'a> Chippy<'a> {
    pub fn new(platform: &'a Platform, font: &'a Font<'a, 'static>) -> Self {
        // Program settings
        let settings = Settings {
            rom_filename: String::new(),
            crt_fade: true,
            crt_fade_rate: constants::PIXEL_FADE_RATE_SLOW,
            debug_mode: false,
            show_debug_panel: false,
            color_palette: ColorPalette::Green as usize,
            hires_mode: true,
            schip_mode: constants::SCHIP_MODE,
        };

        // Init resources
        let frame_timer = platform
            .get_sdl_context()
            .timer()
            .expect("FPS delay timer failed to initialize in main");

        let resources = Resources { frame_timer };

        // Init hardware
        let devices = HardwareDevices {
            tape: Tape::new(),
            display: Display::new(&platform, &font, settings.color_palette),
            audio: Audio::new(&platform.get_sdl_context()),
            keypad: Keypad::new(&platform.get_sdl_context()),
            cpu: Cpu::new(settings.schip_mode),
        };

        Chippy {
            settings,
            resources,
            devices,
        }
    }
}
