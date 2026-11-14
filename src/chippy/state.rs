use crate::chippy::Settings;

///
/// Program State
///
pub struct State {
    pub rom_filename: String,
    pub debug_mode: bool,
    pub show_debug_panel: bool,
    pub _hires_mode: bool,
    pub crt_fade: bool,
    pub crt_fade_rate: u32,
    pub clock_speed: u32,
    target_timestep: u32,
    pub color_palette: usize,
}

impl State {
    pub fn new(settings: &Settings) -> Self {
        let clock_speed = if settings.instruction_set.is_schip() {
            settings.schip_hires_clock_speed_hz
        } else {
            settings.default_clock_speed_hz
        };

        State {
            rom_filename: String::new(),
            debug_mode: false,
            show_debug_panel: false,
            _hires_mode: settings.instruction_set.has_hires_mode(),
            crt_fade: true,
            crt_fade_rate: settings.crt_fade_rate_medium,
            clock_speed,
            target_timestep: 1_000 / clock_speed,
            color_palette: 2,
        }
    }

    pub fn set_clock_speed(&mut self, clock_speed: u32) {
        self.clock_speed = clock_speed;
        self.target_timestep = 1_000 / clock_speed;
    }

    pub fn get_target_timestep(&self) -> u32 {
        self.target_timestep
    }
}
