extern crate sdl2;
use std::{fs, time::Duration};

use crate::constants;

///
/// Tape
///
pub struct Tape {
    pub rom: Vec<u8>,
    pub size: usize,
    pub baud: usize,
    pub sim_timing: bool,
}

impl Tape {
    pub fn new() -> Self {
        Tape {
            rom: Vec::new(),
            size: 0,
            baud: 750,
            sim_timing: false,
        }
    }

    pub fn load_rom(&mut self, filename: &String) {
        let program_data = fs::read(format!("{}/{}", constants::ROM_FOLDER, filename).as_str())
            .unwrap_or(Vec::new());
        let size = program_data.len();
        if size >= constants::MAX_ROM_SIZE as usize {
            panic!(
                "ROM is too big! {}B is greater than the {}B max size",
                size,
                constants::MAX_ROM_SIZE,
            );
        }

        if self.sim_timing && self.baud > 0 {
            let wait_len = Duration::from_millis(((size / self.baud) * 1_000).try_into().unwrap());
            std::thread::sleep(wait_len);
        }

        self.size = size;
        self.rom = program_data;
    }
}
