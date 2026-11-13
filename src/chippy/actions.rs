use log::debug;

use crate::{chip8::ScrollDir, chippy::Chippy, constants};

///
/// Program Actions
///
pub enum Actions {
    Quit,
    Reset,
    Running,
    ToggleDebugMode,
    ToggleDebugPanel,
    CycleColorPalette,
    Step,
    ToggleHiresMode,
    ToggleCrtFade,
    SetCrtFadeSpeed(u32),
    IncVideoScale,
    DecVideoScale,
    ScrollScreen(ScrollDir, usize),
}

impl Actions {
    pub fn handle_reset_action(chippy: &mut Chippy) {
        chippy.devices.cpu.reset();
        chippy.devices.cpu.load_program(&chippy.devices.tape.data);
        chippy.state.debug_mode = false;
    }

    pub fn handle_toggle_debug_panel_action(chippy: &mut Chippy) {
        chippy.state.show_debug_panel = !chippy.state.show_debug_panel;
        chippy
            .devices
            .display
            .debug_panel
            .set_visibility(chippy.state.show_debug_panel);
    }

    pub fn handle_cycle_color_palette_action(chippy: &mut Chippy) {
        chippy.settings.color_palette = chippy.settings.color_palette.wrapping_add(1);
        if chippy.settings.color_palette as usize >= constants::PALETTE_COLORS.len() {
            chippy.settings.color_palette = 0;
        }
        chippy
            .devices
            .display
            .set_color_palette(chippy.settings.color_palette);
        chippy
            .devices
            .display
            .debug_panel
            .set_color_palette(chippy.settings.color_palette);
    }

    pub fn handle_set_crt_fade_rate_action(chippy: &mut Chippy, fade_rate: u32) {
        chippy.state.crt_fade_rate = fade_rate;
    }

    pub fn handle_toggle_debug_mode_action(chippy: &mut Chippy) {
        chippy.state.debug_mode = !chippy.state.debug_mode
    }

    pub fn handle_toggle_crt_fade_action(chippy: &mut Chippy) {
        chippy.state.crt_fade = !chippy.state.crt_fade
    }

    pub fn handle_toggle_hires_mode(chippy: &mut Chippy) {
        let hires_mode = !chippy.devices.cpu.vram.hires_mode;

        // SCHIP-1.x defines clock speed changes on lo/hi res modes
        let clock_speed = if chippy.settings.instruction_set.is_schip() {
            if chippy.devices.cpu.vram.hires_mode {
                chippy.settings.schip_hires_clock_speed_hz
            } else {
                chippy.settings.schip_lores_clock_speed_hz
            }
        } else {
            chippy.settings.default_clock_speed_hz
        };
        if clock_speed != chippy.state.clock_speed {
            debug!("Setting clock speed to: {} hz", clock_speed);
            chippy.state.set_clock_speed(clock_speed);
        }

        chippy.devices.cpu.vram.hires_mode = hires_mode;
        chippy.devices.display.set_hires_mode(hires_mode);
    }

    pub fn handle_inc_video_scale(chippy: &mut Chippy) {
        chippy.devices.display.video_scale = chippy.devices.display.video_scale.saturating_add(1);
        chippy
            .devices
            .display
            .set_hires_mode(chippy.devices.display.get_hires_mode());
    }

    pub fn handle_dec_video_scale(chippy: &mut Chippy) {
        chippy.devices.display.video_scale = chippy.devices.display.video_scale.saturating_sub(1);
        chippy
            .devices
            .display
            .set_hires_mode(chippy.devices.display.get_hires_mode());
    }

    pub fn handle_scroll(chippy: &mut Chippy, dir: ScrollDir, amount: usize) {
        chippy.devices.cpu.vram.scroll(dir, amount);
    }
}
