extern crate sdl2;
use std::{fs, process, time::Duration};

use crate::constants;

///
/// Tape
///
pub struct Tape {
    pub data: Vec<u8>,
    pub size: usize,
    pub baud: usize,
    pub sim_timing: bool,
}

impl Tape {
    pub fn new() -> Self {
        Tape {
            data: Vec::new(),
            size: 0,
            baud: 750,
            sim_timing: false,
        }
    }

    pub fn read(&mut self, filename: &String) {
        let path = format!("{}", filename);

        println!("Loading ROM at path \"{}\"...", path.as_str());

        let program_data = fs::read(path.as_str()).unwrap_or_else(|err| {
            println!("Error reading ROM \"{}\": {err}", path.as_str());
            process::exit(1);
        });
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
        self.data = program_data;

        println!("Done! Loaded {}B ({}KB) into memory", size, size / 8);
    }
}
