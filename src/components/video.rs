use std::collections::HashMap;

use crate::constants;

///
/// Video Memory
///
#[derive(Debug)]
pub struct VideoMemory {
    data: [[u8; constants::SCREEN_WIDTH]; constants::SCREEN_HEIGHT],
    sprites: HashMap<(u16, u8, u8), (usize, usize)>,
}

impl VideoMemory {
    pub fn new() -> Self {
        Self {
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

    pub fn read_sprite(&self, key: (u16, u8, u8)) -> Option<(usize, usize)> {
        self.sprites.get(&key).copied()
    }

    pub fn write_sprite(&mut self, key: (u16, u8, u8), x: usize, y: usize) {
        self.sprites.insert(key, (x, y));
    }

    pub fn clear(&mut self) {
        self.data = [[0; constants::SCREEN_WIDTH]; constants::SCREEN_HEIGHT];
        self.sprites = HashMap::new();
    }
}
