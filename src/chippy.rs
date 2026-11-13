mod actions;
mod settings;
mod state;
mod variants;

use sdl2::{TimerSubsystem, ttf::Font};

pub use actions::*;
pub use settings::*;
pub use variants::*;

use crate::{
    chip8::Cpu,
    chippy::state::State,
    platform::{Audio, Display, Keypad, Platform, Tape},
};

pub struct Resources {
    pub frame_timer: TimerSubsystem,
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
    pub state: State,
    pub resources: Resources,
    pub devices: HardwareDevices<'a>,
}

///
/// Chippy The Program
///
impl<'a> Chippy<'a> {
    pub fn new(platform: &'a Platform, settings: Settings, font: &'a Font<'a, 'static>) -> Self {
        let state = State::new(&settings);

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
                settings.video_scale,
                settings.instruction_set.has_hires_mode(),
                &font,
                state.color_palette,
            ),
            audio: Audio::new(&platform.get_sdl_context(), settings.beep_freq_hz),
            keypad: Keypad::new(&platform.get_sdl_context()),
            cpu: Cpu::new(settings.instruction_set),
        };

        Chippy {
            settings,
            state,
            resources,
            devices,
        }
    }
}
