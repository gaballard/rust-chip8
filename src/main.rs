use dotenv::dotenv;
use env_logger;
use log::debug;
use peripherals::Tape;
use platform::Platform;
use std::env;
use std::fs;
use std::time::Duration;

mod components;
mod constants;
mod fonts;
mod models;
mod peripherals;
mod platform;
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

    // Emulator settings
    let mut debug_mode = false;

    // Set up SDL context
    let platform = Platform::new();

    let mut tape = Tape::new();
    let mut display = Display::new(&platform);
    let mut keypad = Keypad::new(&platform.get_sdl_context());
    let mut cpu = Cpu::new();

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

        // Emulate cycle
        cpu.emulate_cycle();

        // Update display
        display.canvas.set_draw_color(display.background_color);
        display.canvas.clear();

        display.draw_window(
            10,
            10,
            (constants::SCREEN_WIDTH * display.display_scale_factor + 2) as u32,
            (constants::SCREEN_HEIGHT * display.display_scale_factor + 2) as u32,
            // Some(5),
            None,
        );

        display.draw_text(
            cartridge_filename.as_str(),
            10,
            constants::SCREEN_HEIGHT * constants::VIDEO_SCALE + 22,
        );

        display.draw(&cpu.vram);

        display.canvas.present();

        // Increment frame count
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
