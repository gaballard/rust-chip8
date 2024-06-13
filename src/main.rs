use dotenv::dotenv;
use env_logger;
use log::debug;
use std::env;
use std::fs;
use std::time::Duration;

mod components;
mod constants;
mod fonts;
mod models;
mod peripherals;
mod utils;

use components::Cpu;
use peripherals::{Display, Keypad};

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
    let program_data = fs::read(cartridge_filename).unwrap_or(Vec::new());
    if program_data.len() > constants::MAX_ROM_SIZE {
        panic!(
            "ROM is too big! {}b is greater than the {}b max size",
            program_data.len(),
            constants::MAX_ROM_SIZE,
        );
    }

    // Emulator settings
    let mut debug_mode = false;

    // Set up SDL context
    let sdl_context = sdl2::init().expect("SDL2 context failed to initialize in main");

    let mut display = Display::new(&sdl_context);
    let mut keypad = Keypad::new(&sdl_context);
    let mut cpu = Cpu::new(&mut display);

    cpu.load_program(program_data);

    // Emulator timing
    let target_timestep = 1_000 / constants::TARGET_CLOCK_SPEED as u32;
    let mut frame_count: u32 = 0;

    let frame_timer = sdl_context
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

        // Emulate cycle
        cpu.emulate_cycle();

        // Update display
        cpu.display.refresh(&cpu.vram);

        frame_count = frame_count.wrapping_add(1);

        // Get delta time
        let dt = frame_timer.ticks() - prev_frame_tick;

        cpu.update_timers(dt as f32);

        // Delay execution to match target FPS
        if dt < target_timestep {
            std::thread::sleep(Duration::from_millis((target_timestep - dt).into()));
        }
    }

    debug!("Exiting emulator...");
}
