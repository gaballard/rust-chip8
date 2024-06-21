mod waveforms;

use egui_sdl2_platform::sdl2;
use sdl2::{
    audio::{AudioDevice, AudioSpecDesired},
    Sdl,
};
use waveforms::SquareWave;

use crate::constants;

///
/// Audio
///
/// Credit: https://github.com/starrhorne/chip8-rust/blob/master/src/drivers/audio_driver.rs
///
pub struct Audio {
    device: AudioDevice<SquareWave>,
}

impl Audio {
    pub fn new(sdl_context: &Sdl) -> Self {
        let audio_subsystem = sdl_context.audio().unwrap();

        let audio_spec = AudioSpecDesired {
            freq: Some(constants::BEEP_FREQ_HZ),
            channels: Some(1),
            samples: None,
        };

        let device = audio_subsystem
            .open_playback(None, &audio_spec, |spec| SquareWave {
                phase_inc: 240.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            })
            .expect("Error creating audio device");

        Self { device }
    }

    pub fn start_beep(&self) {
        self.device.resume();
    }

    pub fn stop_beep(&self) {
        self.device.pause();
    }
}
