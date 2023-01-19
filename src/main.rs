use dotenv::dotenv;
use env_logger;
use eyre::Result;
use log::{debug, info};
use std::env;
use std::fs;

mod components;
mod constants;
mod fonts;
mod models;
mod peripherals;
mod utils;

use components::Cpu;
use peripherals::{Display, Keypad};

pub enum EmulatorState {
    Quit,
    Reset,
    Running,
}

#[tokio::main]
async fn main() -> Result<()> {
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

    // Set up SDL context
    let sdl_context = sdl2::init().expect("SDL2 context failed to initialize in main");
    let mut timer = sdl_context
        .timer()
        .expect("SDL2 context timer failed to initialize in main");

    // Emulator timing
    let mut prev_tick = timer.ticks();
    let mut prev_second = timer.ticks();
    let target_timestep = 1_000 / constants::TARGET_CLOCK_SPEED;
    let mut fps: u32 = 0;

    let mut display = Display::new(&sdl_context);
    let mut keypad = Keypad::new(&sdl_context);
    let mut cpu = Cpu::new(&mut display);

    cpu.load_program(program_data);

    // Main loop
    'emulate: loop {
        // Check inputs
        match keypad.read_host_keypad() {
            EmulatorState::Quit => break 'emulate,
            EmulatorState::Reset => cpu.reset(),
            EmulatorState::Running => {}
        }
        keypad.read_keypad(&mut cpu.keys);

        // Emulate cycle
        cpu.emulate_cycle();

        // Refresh screen
        debug!("Refreshing screen...");
        cpu.display.refresh(&cpu.vram).await?;
        debug!("Refresh done! Refreshing screen...");

        // Get delta time
        let tick = timer.ticks();
        let dt = tick - prev_tick;

        // Delay execution to match target FPS
        if dt < target_timestep as u32 {
            timer.delay(target_timestep as u32 - dt);
            continue;
        }

        cpu.update_timers(dt as f32);

        fps += 1;

        prev_tick = tick;

        if tick - prev_second > 1_000 {
            debug!("{} FPS", fps);
            prev_second = tick;
            fps = 0;
        }
    }

    debug!("Exiting emulator...");

    Ok(())
}
