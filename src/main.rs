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

    let instruction_set = constants::DEFAULT_INSTRUCTION_SET;
    let mut chippy = Chippy::new(&platform, instruction_set, &font);

    // Interpreter timing
    let mut prev_frame_tick: u32;
    let mut should_execute: bool;

    // Load program
    chippy.settings.rom_filename = args[1].to_owned();
    chippy.devices.tape.read(&chippy.settings.rom_filename);
    chippy.devices.cpu.load_program(&chippy.devices.tape.data);

    println!(
        "Running program using the {} instruction set.",
        instruction_set.get_title()
    );

    // Main loop
    'emulate: loop {
        prev_frame_tick = chippy.resources.frame_timer.ticks();
        should_execute = false;

        // Clear frame buffer
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
            Actions::ToggleHiresMode => Actions::handle_toggle_hires_mode(&mut chippy),
            Actions::SetCrtFadeSpeed(fade_rate) => {
                Actions::handle_set_crt_fade_rate_action(&mut chippy, fade_rate)
            }
            Actions::IncVideoScale => Actions::handle_inc_video_scale(&mut chippy),
            Actions::DecVideoScale => Actions::handle_dec_video_scale(&mut chippy),
        }

        // Refill input buffer
        chippy
            .devices
            .keypad
            .read_keypad(chippy.devices.cpu.keys.get_buffer());

        // Handle debug mode
        if chippy.settings.debug_mode {
            if !should_execute {
                draw_screen(&mut chippy);
                continue;
            }
            debug!("Executing frame {}...", chippy.devices.cpu.cycle);
        }

        // Execute cycle
        chippy.devices.cpu.tick();

        if chippy.settings.instruction_set.is_schip() && chippy.devices.cpu.quit_flag {
            break 'emulate;
        }

        if chippy.devices.cpu.sound_timer > 0 {
            chippy.devices.audio.start_beep();
        } else {
            chippy.devices.audio.stop_beep()
        }

        if chippy.devices.cpu.vram.hires_mode != chippy.devices.display.get_hires_mode() {
            chippy
                .devices
                .display
                .set_hires_mode(chippy.devices.cpu.vram.hires_mode);
        }

        // Draw screen
        draw_screen(&mut chippy);

        let dt = chippy.resources.frame_timer.ticks() - prev_frame_tick;
        if dt < chippy.settings.target_timestep {
            std::thread::sleep(Duration::from_millis(
                (chippy.settings.target_timestep - dt).into(),
            ));
        }
    }

    println!("Thanks for playing!");
}

fn draw_screen(chippy: &mut Chippy) {
    chippy
        .devices
        .display
        .debug_panel
        .update(&chippy.devices.cpu);
    chippy.devices.display.update(
        &mut chippy.devices.cpu.vram,
        chippy.settings.crt_fade,
        chippy.settings.crt_fade_rate as u8,
    );
}
