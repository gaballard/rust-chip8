use dotenv::dotenv;
use env_logger;
use log::debug;
use peripherals::Tape;
use platform::Platform;
use std::env;
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

        // Emulate cycle
        cpu.tick();

        if cpu.quit_flag {
            break 'emulate;
        }

        // Update display
        display.canvas.set_draw_color(display.background_color);
        display.canvas.clear();

        display.draw_window(
            0,
            0,
            (cpu.vram.get_screen_width() * display.display_scale_factor + 2) as u32,
            (cpu.vram.get_screen_height() * display.display_scale_factor + 2) as u32,
        );

        display.draw_text(
            cartridge_filename.as_str(),
            0,
            cpu.vram.get_screen_height() * constants::VIDEO_SCALE + 22,
        );

        // let log_x = (cpu.vram.get_screen_width() * display.display_scale_factor + 42) as i32;

        // display.draw_window(
        //     (screen_width * display.display_scale_factor + 42) as i32,
        //     10,
        //     (screen_width * display.display_scale_factor - 32) as u32,
        //     (screen_height * display.display_scale_factor + 2) as u32,
        // );

        // display.draw_text(
        //     frame_count.to_string().as_str(),
        //     log_x as usize,
        //     screen_height * constants::VIDEO_SCALE + 22,
        // );

        display.draw(&cpu.vram);

        display.canvas.present();

        // Increment frame count
        frame_count = frame_count.wrapping_add(1);

        // Get delta time
        let dt = frame_timer.ticks() - prev_frame_tick;

        // Delay execution to match target FPS
        if dt < target_timestep {
            std::thread::sleep(Duration::from_millis((target_timestep - dt).into()));
        }
    }

    debug!("Exiting emulator...");
}
