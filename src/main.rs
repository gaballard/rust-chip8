mod chip8;
mod constants;
mod fonts;
mod platform;
mod utils;

use env_logger;
use log::debug;

use std::env;
use std::time::Duration;

use chip8::Cpu;
use platform::{Audio, Display, Keypad, Platform, Tape};

///
/// Emulator State
///
pub enum PixelFadeRate {
    Slow,
    Medium,
    Fast,
}

pub enum ColorPalette {
    Amber = 0,
    Green = 1,
}

pub enum EmulatorActions {
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

///
/// Emulator Settings
///
pub struct EmulatorSettings {
    pub crt_fade: bool,
    pub crt_fade_rate: u8,
    pub debug_mode: bool,
    pub show_debug_panel: bool,
    pub color_palette: usize,
    pub hires_mode: bool,
    pub schip_mode: bool,
}

///
/// Main
///
fn main() {
    env_logger::try_init().expect("Couldn't load env_logger");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Must include path to ROM file!");
    }

    // Emulator settings
    let mut settings = EmulatorSettings {
        crt_fade: true,
        crt_fade_rate: constants::PIXEL_FADE_RATE_SLOW,
        debug_mode: false,
        show_debug_panel: false,
        color_palette: ColorPalette::Green as usize,
        hires_mode: true,
        schip_mode: constants::SCHIP_MODE,
    };

    // Set up SDL context
    let platform = Platform::new();

    // Load resources
    let font = platform
        .get_ttf_context()
        .load_font(constants::FONT_PATH, constants::FONT_SIZE)
        .expect("Font does not exist");

    // Init hardware
    let mut tape = Tape::new();
    let mut display = Display::new(&platform, &font, settings.color_palette);
    let audio = Audio::new(&platform.get_sdl_context());
    let mut keypad = Keypad::new(&platform.get_sdl_context());
    let mut cpu = Cpu::new(settings.schip_mode);

    let cartridge_filename = &args[1];
    tape.read(cartridge_filename);
    cpu.load_program(&tape.rom);

    // Emulator timing
    let emulator_timestep = 1_000 / constants::TARGET_CLOCK_SPEED;

    let frame_timer = platform
        .get_sdl_context()
        .timer()
        .expect("FPS delay timer failed to initialize in main");

    let mut prev_frame_tick: u32;
    let mut should_execute: bool;

    println!("Running program...");

    // Main loop
    'emulate: loop {
        prev_frame_tick = frame_timer.ticks();

        should_execute = false;

        display.reset();
        display.debug_panel.reset();
        cpu.vram.vram_changed = false;

        // Check inputs
        match keypad.read_host_keypad() {
            EmulatorActions::Quit => break 'emulate,
            EmulatorActions::Reset => {
                cpu.reset();
                cpu.load_program(&tape.rom);
                settings.debug_mode = false;
            }
            EmulatorActions::Running => {}
            EmulatorActions::ToggleDebugMode => settings.debug_mode = !settings.debug_mode,
            EmulatorActions::ToggleDebugPanel => {
                settings.show_debug_panel = !settings.show_debug_panel;
                display
                    .debug_panel
                    .set_visibility(settings.show_debug_panel);
            }
            EmulatorActions::CycleColorPalette => {
                settings.color_palette = settings.color_palette.wrapping_add(1);
                if settings.color_palette as usize >= constants::PALETTE_COLORS.len() {
                    settings.color_palette = 0;
                }
                display.set_color_palette(settings.color_palette);
                display
                    .debug_panel
                    .set_color_palette(settings.color_palette);
            }
            EmulatorActions::Step => should_execute = true,
            EmulatorActions::ToggleCrtFade => settings.crt_fade = !settings.crt_fade,
            EmulatorActions::SetCrtFadeSpeed(fade_rate) => {
                settings.crt_fade_rate = match fade_rate {
                    PixelFadeRate::Slow => constants::PIXEL_FADE_RATE_SLOW,
                    PixelFadeRate::Medium => constants::PIXEL_FADE_RATE_MED,
                    PixelFadeRate::Fast => constants::PIXEL_FADE_RATE_FAST,
                };
            }
        }
        keypad.read_keypad(cpu.keys.get_buffer());

        if settings.debug_mode {
            if !should_execute {
                display.debug_panel.update(&cpu);
                display.update(&mut cpu.vram, settings.crt_fade, settings.crt_fade_rate);
                continue;
            }
            debug!("Executing frame {}...", cpu.cycle);
        }

        cpu.tick();

        if settings.schip_mode && cpu.quit_flag {
            break 'emulate;
        }

        if cpu.sound_timer > 0 {
            audio.start_beep();
        } else {
            audio.stop_beep()
        }

        display.debug_panel.update(&cpu);
        display.update(&mut cpu.vram, settings.crt_fade, settings.crt_fade_rate);

        let dt = frame_timer.ticks() - prev_frame_tick;
        if dt < emulator_timestep {
            std::thread::sleep(Duration::from_millis((emulator_timestep - dt).into()));
        }
    }

    println!("Thanks for playing!");
}
