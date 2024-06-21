mod chip8;
mod constants;
mod fonts;
mod platform;
mod utils;

use dotenv::dotenv;
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
}

fn main() {
    env_logger::try_init().expect("Couldn't load env_logger");
    dotenv().expect("Couldn't load settings from `.env` file");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Must include path to ROM file!");
    }
    let cartridge_filename = &args[1];

    // Emulator settings
    let mut debug_mode = false;

    // Set up SDL context
    let platform = Platform::new();

    let mut tape = Tape::new();
    let mut display = Display::new(&platform);
    let audio = Audio::new(&platform.get_sdl_context());
    let mut keypad = Keypad::new(&platform.get_sdl_context());
    let mut cpu = Cpu::new(constants::SCHIP_MODE);

    tape.read(cartridge_filename);

    cpu.load_program(tape.rom);

    // Emulator timing
    let target_timestep = 1_000 / constants::TARGET_CLOCK_SPEED as u32;

    let frame_timer = platform
        .get_sdl_context()
        .timer()
        .expect("FPS delay timer failed to initialize in main");

    let mut prev_frame_tick: u32;

    // Main loop
    'emulate: loop {
        prev_frame_tick = frame_timer.ticks();

        let mut should_execute = false;

        // Check inputs
        match keypad.read_host_keypad() {
            EmulatorState::Quit => break 'emulate,
            EmulatorState::Reset => cpu.reset(),
            EmulatorState::Running => {}
            EmulatorState::DebugMode => debug_mode = !debug_mode,
            EmulatorState::Step => should_execute = true,
        }
        keypad.read_keypad(cpu.keys.get_buffer());

        if debug_mode {
            if !should_execute {
                continue;
            }
            debug!("Executing frame {}...", cpu.cycle);
        }

        cpu.tick();

        if cpu.schip_mode && cpu.quit_flag {
            break 'emulate;
        }

        if cpu.sound_timer > 0 {
            audio.start_beep();
        } else {
            audio.stop_beep()
        }

        if cpu.vram_changed {
            display.draw(&cpu.vram);
        }

        let dt = frame_timer.ticks() - prev_frame_tick;
        if dt < target_timestep {
            std::thread::sleep(Duration::from_millis((target_timestep - dt).into()));
        }
    }

    debug!("Exiting emulator...");
}
