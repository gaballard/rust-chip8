use std::collections::HashMap;

use crate::constants;

///
/// Video Memory
///
#[derive(Debug)]
pub struct VideoMemory {
    pub hires_mode: bool,
    pub data: [[u8; constants::SCREEN_WIDTH]; constants::SCREEN_HEIGHT],
    sprites: HashMap<(u16, u8, u8), (usize, usize)>,
}

impl VideoMemory {
    pub fn new() -> Self {
        Self {
            hires_mode: false,
            data: [[0; constants::SCREEN_WIDTH]; constants::SCREEN_HEIGHT],
            sprites: HashMap::new(),
        }
    }

    pub fn read(&self, x: usize, y: usize) -> &u8 {
        &self.data[y][x]
    }

    pub fn write(&mut self, x: usize, y: usize, value: u8) {
        self.data[y][x] = value;
    }

    pub fn _read_sprite(&self, key: (u16, u8, u8)) -> Option<(usize, usize)> {
        self.sprites.get(&key).copied()
    }

    pub fn _write_sprite(&mut self, key: (u16, u8, u8), x: usize, y: usize) {
        self.sprites.insert(key, (x, y));
    }

    pub fn clear(&mut self) {
        self.data = [[0; constants::SCREEN_WIDTH]; constants::SCREEN_HEIGHT];
        self.sprites = HashMap::new();
    }

    pub fn get_screen_width(&self) -> usize {
        if self.hires_mode {
            constants::SCREEN_WIDTH
        } else {
            constants::SCREEN_WIDTH
        }
    }

    pub fn get_screen_height(&self) -> usize {
        if self.hires_mode {
            constants::SCREEN_HEIGHT
        } else {
            constants::SCREEN_HEIGHT
        }
    }
}
