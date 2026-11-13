use std::io::Result;

use ini::Ini;

use crate::chippy::InstructionSet;

///
/// Chippy Settings
///
/// Reads from `settings.ini` in the folder specified by `settings_ini_folder`.
///

pub struct Settings {
    // Interpreter
    pub instruction_set: InstructionSet,

    // Display
    pub video_scale: u32,
    pub _display_refresh_rate_hz: u32,
    pub crt_fade_rate_slow: u32,
    pub crt_fade_rate_medium: u32,
    pub crt_fade_rate_fast: u32,

    // Fonts
    pub font_size: u16,
    pub font_path: String,

    // Colors
    pub color_palette: usize,

    // Audio
    pub beep_freq_hz: u32,

    // CPU
    pub default_clock_speed_hz: u32,
    pub schip_lores_clock_speed_hz: u32,
    pub schip_hires_clock_speed_hz: u32,
}

impl Settings {
    pub fn from(settings_ini_folder: &str) -> Result<Self> {
        let settings_ini = Ini::load_from_file(format!("{}/settings.ini", settings_ini_folder))
            .expect("Unable to open settings file");

        //
        // Interpreter
        //
        let interpreter_section = settings_ini
            .section(Some("Interpreter"))
            .expect("Settings.ini missing \"Interpreter\" section");

        let instruction_set = InstructionSet::from_title(
            interpreter_section
                .get("instruction_set")
                .expect("Settings.ini missing \"Interpreter.instruction_set\""),
        );

        //
        // Display
        //
        let display_section = settings_ini
            .section(Some("Display"))
            .expect("Settings.ini missing \"Display\" section");

        let _display_refresh_rate_hz: u32 = display_section
            .get("display_refresh_rate_hz")
            .expect("Settings.ini missing \"Display.display_refresh_rate_hz\"")
            .parse()
            .expect("Display.display_refresh_rate_hz should be a 32-bit integer");

        let video_scale: u32 = display_section
            .get("video_scale")
            .expect("Settings.ini missing \"Display.video_scale\"")
            .parse()
            .expect("Display.video_scale should be a 32-bit integer");

        let crt_fade_rate_slow: u32 = display_section
            .get("crt_fade_rate_slow")
            .expect("Settings.ini missing \"Display.crt_fade_rate_slow\"")
            .parse()
            .expect("Display.crt_fade_rate_slow should be a 32-bit integer");

        let crt_fade_rate_medium: u32 = display_section
            .get("crt_fade_rate_medium")
            .expect("Settings.ini missing \"Display.crt_fade_rate_medium\"")
            .parse()
            .expect("Display.crt_fade_rate_medium should be a 32-bit integer");

        let crt_fade_rate_fast: u32 = display_section
            .get("crt_fade_rate_fast")
            .expect("Settings.ini missing \"Display.crt_fade_rate_fast\"")
            .parse()
            .expect("Display.crt_fade_rate_fast should be a 32-bit integer");

        //
        // Fonts
        //
        let fonts_section = settings_ini
            .section(Some("Fonts"))
            .expect("Settings.ini missing \"Fonts\" section");

        let font_size: u16 = fonts_section
            .get("font_size")
            .expect("Settings.ini missing \"Fonts.font_size\"")
            .parse()
            .expect("Fonts.font_size should be a 16-bit integer");

        let font_path: String = fonts_section
            .get("font_path")
            .expect("Settings.ini missing \"Fonts.font_path\"")
            .to_string();

        //
        // Colors
        //
        let color_section = settings_ini
            .section(Some("Colors"))
            .expect("Settings.ini missing \"Colors\" section");

        let color_palette: usize = color_section
            .get("color_palette")
            .expect("Settings.ini missing \"Colors.color_palette\"")
            .parse()
            .expect(
                "Colors.color_palette should be an integer between 1-3 (1=white, 2=amber, 3=green)",
            );

        //
        // Audio
        //
        let audio_section = settings_ini
            .section(Some("Audio"))
            .expect("Settings.ini missing \"Audio\" section");

        let beep_freq_hz: u32 = audio_section
            .get("beep_freq_hz")
            .expect("Settings.ini missing \"Audio.beep_freq_hz\"")
            .parse()
            .expect("Audio.beep_freq_hz should be a 32-bit integer");

        //
        // CPU
        //
        let cpu_section = settings_ini
            .section(Some("CPU"))
            .expect("Settings.ini missing \"CPU\" section");

        let default_clock_speed_hz: u32 = cpu_section
            .get("default_clock_speed_hz")
            .expect("Settings.ini missing \"CPU.default_clock_speed_hz\"")
            .parse()
            .expect("CPU.default_clock_speed_hz should be a 32-bit integer");

        let schip_lores_clock_speed_hz: u32 = cpu_section
            .get("schip_lores_clock_speed_hz")
            .expect("Settings.ini missing \"CPU.schip_lores_clock_speed_hz\"")
            .parse()
            .expect("CPU.schip_lores_clock_speed_hz should be a 32-bit integer");

        let schip_hires_clock_speed_hz: u32 = cpu_section
            .get("schip_hires_clock_speed_hz")
            .expect("Settings.ini missing \"CPU.schip_hires_clock_speed_hz\"")
            .parse()
            .expect("CPU.schip_hires_clock_speed_hz should be a 32-bit integer");

        Ok(Settings {
            instruction_set,
            video_scale,
            _display_refresh_rate_hz,
            crt_fade_rate_slow,
            crt_fade_rate_medium,
            crt_fade_rate_fast,
            font_size,
            font_path,
            color_palette,
            beep_freq_hz,
            default_clock_speed_hz,
            schip_lores_clock_speed_hz,
            schip_hires_clock_speed_hz,
        })
    }
}
