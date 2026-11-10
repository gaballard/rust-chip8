mod actions;
mod instructions;

use sdl2::{TimerSubsystem, ttf::Font};

pub use actions::*;
pub use instructions::*;

use crate::{
    chip8::Cpu,
    constants,
    platform::{Audio, Display, Keypad, Platform, Tape},
};

#[derive(Clone, Copy)]
pub enum PixelFadeRate {
    Slow = 3,
    Medium = 5,
    Fast = 7,
}

pub struct Resources {
    pub frame_timer: TimerSubsystem,
}

pub struct Settings {
    pub rom_filename: String,
    pub crt_fade: bool,
    pub crt_fade_rate: PixelFadeRate,
    pub debug_mode: bool,
    pub show_debug_panel: bool,
    pub color_palette: usize,
    pub instruction_set: InstructionSet,
    pub clock_speed: u32,
    pub target_timestep: u32,
}

impl Settings {
    pub fn set_clock_speed(&mut self, clock_speed: u32) {
        self.clock_speed = clock_speed;
        self.target_timestep = 1_000 / clock_speed;
    }
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
    pub fn new(
        platform: &'a Platform,
        instruction_set: InstructionSet,
        font: &'a Font<'a, 'static>,
    ) -> Self {
        let clock_speed = if instruction_set.is_schip() {
            constants::HIRES_CLOCK_SPEED
        } else {
            constants::DEFAULT_CLOCK_SPEED
        };

        // Program settings
        let settings = Settings {
            rom_filename: String::new(),
            crt_fade: true,
            crt_fade_rate: PixelFadeRate::Slow,
            debug_mode: false,
            show_debug_panel: false,
            color_palette: constants::DEFAULT_PALETTE,
            instruction_set,
            clock_speed,
            target_timestep: 1_000 / clock_speed,
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
            display: Display::new(
                &platform,
                instruction_set.has_hires_mode(),
                &font,
                settings.color_palette,
            ),
            audio: Audio::new(&platform.get_sdl_context()),
            keypad: Keypad::new(&platform.get_sdl_context()),
            cpu: Cpu::new(settings.instruction_set),
        };

        Chippy {
            settings,
            resources,
            devices,
        }
    }
}
