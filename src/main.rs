use devices::{Audio, Tape};
use dotenv::dotenv;
use env_logger;
use log::debug;
use platform::Platform;
use std::env;
use std::time::Duration;

mod components;
mod constants;
mod devices;
mod fonts;
mod models;
mod platform;
mod utils;

use components::Cpu;
use devices::{Display, Keypad};

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

    tape.load_rom(cartridge_filename);

    cpu.load_program(tape.rom);

    // Emulator timing
    let target_timestep = 1_000 / constants::TARGET_CLOCK_SPEED as u32;
    let mut frame_count: u32 = 0;

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
        keypad.read_keypad(&mut cpu.keys);

        if debug_mode {
            if !should_execute {
                continue;
            }
            debug!("Executing frame {}...", frame_count);
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

        frame_count = frame_count.wrapping_add(1);

        let dt = frame_timer.ticks() - prev_frame_tick;
        if dt < target_timestep {
            std::thread::sleep(Duration::from_millis((target_timestep - dt).into()));
        }
    }

    debug!("Exiting emulator...");
}
