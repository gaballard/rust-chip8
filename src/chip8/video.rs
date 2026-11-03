pub mod sprite;

use std::collections::HashMap;

use log::debug;

use crate::{
    chip8::sprite::{Sprite, SpriteKey},
    constants,
    utils::read_bit_from_byte,
};

///
/// Video Memory
///
#[derive(Debug)]
#[must_use]
pub struct VideoMemory {
    pub hires_mode: bool,
    pub vram_changed: bool,
    pub data: [[u8; constants::SCREEN_WIDTH]; constants::SCREEN_HEIGHT],
    sprites: HashMap<SpriteKey, Sprite>,
    collisions: u8,
}

impl VideoMemory {
    pub fn new() -> Self {
        Self {
            hires_mode: false,
            vram_changed: false,
            data: [[0; constants::SCREEN_WIDTH]; constants::SCREEN_HEIGHT],
            sprites: HashMap::new(),
            collisions: 0,
        }
    }

    pub fn read(&self, x: usize, y: usize) -> &u8 {
        &self.data[y][x]
    }

    pub fn write(&mut self, x: usize, y: usize, value: u8) {
        self.data[y][x] = value;
        self.vram_changed = true;
    }

    pub fn clear(&mut self) {
        self.data = [[0; constants::SCREEN_WIDTH]; constants::SCREEN_HEIGHT];
        self.vram_changed = true;
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

    ///
    /// Sprite Handling
    ///

    pub fn has_sprite(&self, key: &SpriteKey) -> bool {
        match self.get_sprite(key) {
            Some(_) => true,
            _ => false,
        }
    }

    pub fn get_sprite(&self, key: &SpriteKey) -> Option<&Sprite> {
        self.sprites.get(key)
    }

    pub fn get_sprite_mut(&mut self, key: &SpriteKey) -> Option<&mut Sprite> {
        self.sprites.get_mut(key)
    }

    pub fn add_sprite(&mut self, key: &SpriteKey, sprite_data: &[u8], sprite_len: u8) {
        self.sprites
            .insert(*key, Sprite::new(*key, sprite_data.to_vec(), sprite_len));
    }

    pub fn place_sprite(&mut self, key: &SpriteKey, x: usize, y: usize) {
        match self.get_sprite_mut(key) {
            Some(sprite) => sprite.set_coords(x, y),
            _ => return,
        }
    }

    pub fn draw_sprite(&mut self, key: &SpriteKey) -> u8 {
        let mut collisions: u8 = 0;

        let sprite = match self.get_sprite(key) {
            Some(sprite) => sprite.to_owned(),
            _ => {
                debug!("Couldn't find sprite with key {:?}", key);
                return 0;
            }
        };

        for sy in 0..sprite.get_sprite_len() {
            let vy = (sprite.get_y() + sy as usize) % self.get_screen_height();
            for sx in 0..8 as u8 {
                let vx = (sprite.get_x() + sx as usize) % self.get_screen_width();

                let pixel = read_bit_from_byte(sprite.get_sprite_data()[sy as usize], 7 - sx as u8);

                if pixel == 0 {
                    continue;
                }

                if self.read(vx, vy) & pixel == 1 {
                    collisions += 1;
                }

                self.write(vx, vy, self.read(vx, vy) ^ pixel);
            }
        }

        self.collisions = collisions;

        collisions
    }

    pub fn clear_collisions(&mut self) {
        self.collisions = 0;
    }
}
