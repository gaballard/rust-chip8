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
pub enum EmulatorState {
    Quit,
    Reset,
    Running,
    DebugMode,
    Step,
    ToggleCrtFade,
}

///
/// Emulator Settings
///
pub struct EmulatorSettings {
    pub crt_fade: bool,
    pub debug_mode: bool,
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
        debug_mode: false,
        hires_mode: true,
        schip_mode: constants::SCHIP_MODE,
    };

    // Set up SDL context
    let platform = Platform::new();

    // Init hardware
    let mut tape = Tape::new();
    let mut display = Display::new(&platform);
    let audio = Audio::new(&platform.get_sdl_context());
    let mut keypad = Keypad::new(&platform.get_sdl_context());
    let mut cpu = Cpu::new(settings.schip_mode);

    let cartridge_filename = &args[1];
    tape.read(cartridge_filename);
    cpu.load_program(tape.rom);

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

        display.canvas.clear();
        display.canvas.set_draw_color(display.background_color);
        cpu.vram.vram_changed = false;

        // Check inputs
        match keypad.read_host_keypad() {
            EmulatorState::Quit => break 'emulate,
            EmulatorState::Reset => cpu.reset(),
            EmulatorState::Running => {}
            EmulatorState::DebugMode => settings.debug_mode = !settings.debug_mode,
            EmulatorState::Step => should_execute = true,
            EmulatorState::ToggleCrtFade => settings.crt_fade = !settings.crt_fade,
        }
        keypad.read_keypad(cpu.keys.get_buffer());

        if settings.debug_mode {
            if !should_execute {
                display.update(&mut cpu.vram, settings.crt_fade);
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

        display.update(&mut cpu.vram, settings.crt_fade);

        let dt = frame_timer.ticks() - prev_frame_tick;
        if dt < emulator_timestep {
            std::thread::sleep(Duration::from_millis((emulator_timestep - dt).into()));
        }
    }

    println!("Thanks for playing!");
}
