use log::debug;

use crate::{
    chippy::{Chippy, PixelFadeRate},
    constants,
};

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
    SetCrtFadeSpeed(PixelFadeRate),
    IncVideoScale,
    DecVideoScale,
}

impl Actions {
    pub fn handle_reset_action(chippy: &mut Chippy) {
        chippy.devices.cpu.reset();
        chippy.devices.cpu.load_program(&chippy.devices.tape.data);
        chippy.settings.debug_mode = false;
    }

    pub fn handle_toggle_debug_panel_action(chippy: &mut Chippy) {
        chippy.settings.show_debug_panel = !chippy.settings.show_debug_panel;
        chippy
            .devices
            .display
            .debug_panel
            .set_visibility(chippy.settings.show_debug_panel);
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

    pub fn handle_set_crt_fade_rate_action(chippy: &mut Chippy, fade_rate: PixelFadeRate) {
        chippy.settings.crt_fade_rate = fade_rate;
    }

    pub fn handle_toggle_debug_mode_action(chippy: &mut Chippy) {
        chippy.settings.debug_mode = !chippy.settings.debug_mode
    }

    pub fn handle_toggle_crt_fade_action(chippy: &mut Chippy) {
        chippy.settings.crt_fade = !chippy.settings.crt_fade
    }

    pub fn handle_toggle_hires_mode(chippy: &mut Chippy) {
        let hires_mode = !chippy.devices.cpu.vram.hires_mode;

        // SCHIP-1.x defines clock speed changes on lo/hi res modes
        let clock_speed = if chippy.settings.instruction_set.is_schip() {
            if chippy.devices.cpu.vram.hires_mode {
                constants::HIRES_CLOCK_SPEED
            } else {
                constants::LORES_CLOCK_SPEED
            }
        } else {
            constants::DEFAULT_CLOCK_SPEED
        };
        if clock_speed != chippy.settings.clock_speed {
            debug!("Setting clock speed to: {} hz", clock_speed);
            chippy.settings.set_clock_speed(clock_speed);
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
}
