use std::{collections::HashMap, hash::Hash};

use crate::constants;

///
/// Video Memory
///
#[derive(Debug)]
pub struct VideoMemory {
    data: [[u8; constants::SCREEN_WIDTH]; constants::SCREEN_HEIGHT],
    prev_data: [[u8; constants::SCREEN_WIDTH]; constants::SCREEN_HEIGHT],
    sprites: HashMap<(u16, u8, u8), (usize, usize)>,
}

impl VideoMemory {
    pub fn new() -> Self {
        Self {
            data: [[0; constants::SCREEN_WIDTH]; constants::SCREEN_HEIGHT],
            prev_data: [[0; constants::SCREEN_WIDTH]; constants::SCREEN_HEIGHT],
            sprites: HashMap::new(),
        }
    }

    pub fn read(&self, x: usize, y: usize) -> &u8 {
        // &self.data[y * x]
        &self.data[y][x]
    }

    pub fn read_prev(&self, x: usize, y: usize) -> &u8 {
        &self.prev_data[y][x]
    }

    pub fn read_sprite(&self, key: (u16, u8, u8)) -> Option<(usize, usize)> {
        self.sprites.get(&key).copied()
    }

    pub fn write(&mut self, x: usize, y: usize, value: u8) {
        // self.prev_data[y][x] = self.data[y][x];
        self.data[y][x] = value;
        // self.data[y * x] = value;
    }

    pub fn write_prev(&mut self, x: usize, y: usize, value: u8) {
        self.prev_data[y][x] = self.data[y][x];
    }

    pub fn write_data(&mut self, data: [[u8; constants::SCREEN_WIDTH]; constants::SCREEN_HEIGHT]) {
        self.prev_data = data;
        self.data = data;
    }

    pub fn write_sprite(&mut self, key: (u16, u8, u8), x: usize, y: usize) {
        self.sprites.insert(key, (x, y));
    }

    pub fn clear(&mut self) {
        // self.data = [0; constants::SCREEN_WIDTH * constants::SCREEN_HEIGHT];
        self.data = [[0; constants::SCREEN_WIDTH]; constants::SCREEN_HEIGHT];
        self.prev_data = [[0; constants::SCREEN_WIDTH]; constants::SCREEN_HEIGHT];
        self.sprites = HashMap::new();
        // self.data = Box::new([[0; constants::SCREEN_WIDTH]; constants::SCREEN_HEIGHT]);
    }

    pub fn get_pixel_changed(&self, x: usize, y: usize) -> bool {
        return if self.read(x, y) != self.read_prev(x, y) {
            true
        } else {
            false
        };
    }
}
