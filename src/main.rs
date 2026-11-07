mod chip8;
mod chippy;
mod constants;
mod fonts;
mod platform;
mod utils;

use env_logger;
use log::debug;

use platform::Platform;
use std::{env, time::Duration};

use crate::chippy::*;

///
/// Main
///
fn main() {
    env_logger::try_init().expect("Couldn't load env_logger");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Must include path to ROM file!");
    }

    println!("There's my Chippy.");

    // Set up SDL context
    let platform = Platform::new();

    // Load resources
    let font = platform
        .get_ttf_context()
        .load_font(constants::FONT_PATH, constants::FONT_SIZE)
        .expect("Font does not exist");

    let mut chippy = Chippy::new(&platform, &font);

    // Interpreter timing
    let emulator_timestep = 1_000 / constants::TARGET_CLOCK_SPEED;

    let mut prev_frame_tick: u32;
    let mut should_execute: bool;

    // Load program
    chippy.settings.rom_filename = args[1].to_owned();
    chippy.devices.tape.read(&chippy.settings.rom_filename);
    chippy.devices.cpu.load_program(&chippy.devices.tape.data);

    println!("Running program...");

    // Main loop
    'emulate: loop {
        prev_frame_tick = chippy.resources.frame_timer.ticks();
        should_execute = false;

        // Clear screen
        chippy.devices.display.reset();
        chippy.devices.display.debug_panel.reset();
        chippy.devices.cpu.vram.vram_changed = false;

        // Check inputs
        match chippy.devices.keypad.read_host_keypad() {
            Actions::Running => {}
            Actions::Quit => break 'emulate,
            Actions::Reset => Actions::handle_reset_action(&mut chippy),
            Actions::ToggleDebugMode => Actions::handle_toggle_debug_mode_action(&mut chippy),
            Actions::Step => should_execute = true,
            Actions::ToggleDebugPanel => Actions::handle_toggle_debug_panel_action(&mut chippy),
            Actions::CycleColorPalette => Actions::handle_cycle_color_palette_action(&mut chippy),
            Actions::ToggleCrtFade => Actions::handle_toggle_crt_fade_action(&mut chippy),
            Actions::SetCrtFadeSpeed(fade_rate) => {
                Actions::handle_set_crt_fade_rate_action(&mut chippy, fade_rate)
            }
        }

        // Refill input buffer
        chippy
            .devices
            .keypad
            .read_keypad(chippy.devices.cpu.keys.get_buffer());

        // Handle debug mode
        if chippy.settings.debug_mode {
            if !should_execute {
                chippy
                    .devices
                    .display
                    .debug_panel
                    .update(&chippy.devices.cpu);
                chippy.devices.display.update(
                    &mut chippy.devices.cpu.vram,
                    chippy.settings.crt_fade,
                    chippy.settings.crt_fade_rate,
                );
                continue;
            }
            debug!("Executing frame {}...", chippy.devices.cpu.cycle);
        }

        // Execute cycle
        chippy.devices.cpu.tick();

        if chippy.settings.schip_mode && chippy.devices.cpu.quit_flag {
            break 'emulate;
        }

        if chippy.devices.cpu.sound_timer > 0 {
            chippy.devices.audio.start_beep();
        } else {
            chippy.devices.audio.stop_beep()
        }

        // Draw screen
        chippy
            .devices
            .display
            .debug_panel
            .update(&chippy.devices.cpu);
        chippy.devices.display.update(
            &mut chippy.devices.cpu.vram,
            chippy.settings.crt_fade,
            chippy.settings.crt_fade_rate,
        );

        let dt = chippy.resources.frame_timer.ticks() - prev_frame_tick;
        if dt < emulator_timestep {
            std::thread::sleep(Duration::from_millis((emulator_timestep - dt).into()));
        }
    }

    println!("Thanks for playing!");
}
