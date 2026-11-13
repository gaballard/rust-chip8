mod waveforms;

use sdl2::{
    Sdl,
    audio::{AudioDevice, AudioSpecDesired},
};
use waveforms::SquareWave;

///
/// Audio
///
/// Credit: https://github.com/starrhorne/chip8-rust/blob/master/src/drivers/audio_driver.rs
///
pub struct Audio {
    device: AudioDevice<SquareWave>,
}

impl Audio {
    pub fn new(sdl_context: &Sdl, beep_freq_hz: u32) -> Self {
        let audio_subsystem = sdl_context.audio().unwrap();

        let audio_spec = AudioSpecDesired {
            freq: Some(beep_freq_hz as i32),
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
