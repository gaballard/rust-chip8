pub mod sprite;

use std::collections::HashMap;

use log::debug;

use crate::{
    chip8::sprite::{Sprite, SpriteKey},
    constants,
    utils::read_bit_from_byte,
};

#[derive(Debug)]
pub enum ScrollDir {
    Up,
    Down,
    Left,
    Right,
}

///
/// Video Memory
///
#[derive(Debug)]
#[must_use]
pub struct VideoMemory {
    pub hires_mode: bool,
    pub vram_changed: bool,
    data: [[u8; constants::HIRES_SCREEN_WIDTH]; constants::HIRES_SCREEN_HEIGHT],
    sprites: HashMap<SpriteKey, Sprite>,
    collisions: u8,
}

impl VideoMemory {
    pub fn new() -> Self {
        Self {
            hires_mode: false,
            vram_changed: false,
            data: [[0; constants::HIRES_SCREEN_WIDTH]; constants::HIRES_SCREEN_HEIGHT],
            sprites: HashMap::new(),
            collisions: 0,
        }
    }

    ///
    /// Memory
    ///

    pub fn read(&self, x: usize, y: usize) -> &u8 {
        if self.hires_mode {
            &self.data[y][x]
        } else {
            &self.data[if y > constants::LORES_SCREEN_HEIGHT {
                constants::LORES_SCREEN_HEIGHT
            } else {
                y
            }][if x > constants::LORES_SCREEN_WIDTH {
                constants::LORES_SCREEN_WIDTH
            } else {
                x
            }]
        }
    }

    pub fn write(&mut self, x: usize, y: usize, value: u8) {
        if self.hires_mode {
            self.data[y][x] = value;
        } else {
            self.data[if y > constants::LORES_SCREEN_HEIGHT {
                constants::LORES_SCREEN_HEIGHT
            } else {
                y
            }][if x > constants::LORES_SCREEN_WIDTH {
                constants::LORES_SCREEN_WIDTH
            } else {
                x
            }] = value;
        }
        self.vram_changed = true;
    }

    pub fn clear(&mut self) {
        self.data = [[0; constants::HIRES_SCREEN_WIDTH]; constants::HIRES_SCREEN_HEIGHT];
        self.vram_changed = true;
    }

    ///
    /// Screen
    ///

    pub fn get_screen_width(&self) -> usize {
        if self.hires_mode {
            constants::HIRES_SCREEN_WIDTH
        } else {
            constants::LORES_SCREEN_WIDTH
        }
    }

    pub fn get_screen_height(&self) -> usize {
        if self.hires_mode {
            constants::HIRES_SCREEN_HEIGHT
        } else {
            constants::LORES_SCREEN_HEIGHT
        }
    }

    ///
    /// Scrolling
    ///

    pub fn scroll(&mut self, dir: ScrollDir, amount: usize) {
        let mut new_data = self.data.clone();

        for y in 0..self.get_screen_height() {
            for x in 0..self.get_screen_width() {
                new_data[y][x] = match dir {
                    ScrollDir::Down => self.data[y.saturating_sub(amount)][x],
                    ScrollDir::Up => self.data[y.saturating_add(amount)][x],
                    ScrollDir::Right => self.data[y][x.saturating_sub(amount)],
                    ScrollDir::Left => self.data[y][x.saturating_add(amount)],
                };
            }
        }

        self.data = new_data;
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

    pub fn add_sprite(&mut self, key: &SpriteKey, width: u8, height: u8, sprite_data: &[u8]) {
        self.sprites
            .insert(*key, Sprite::new(*key, width, height, sprite_data.to_vec()));
    }

    pub fn place_sprite(&mut self, key: &SpriteKey, x: usize, y: usize) {
        match self.get_sprite_mut(key) {
            Some(sprite) => sprite.set_coords(x, y),
            _ => return,
        }
    }

    pub fn draw_sprite(&mut self, key: &SpriteKey, _clipping: bool) -> u8 {
        let mut collisions: u8 = 0;

        let sprite = match self.get_sprite(key) {
            Some(sprite) => sprite.to_owned(),
            _ => {
                debug!("Couldn't find sprite with key {:?}", key);
                return 0;
            }
        };

        // TODO: what's the role of clipping again between CHIP-8 and SCHIP?

        let pos_y = if sprite.get_y() > self.get_screen_height() {
            sprite.get_y() % self.get_screen_height()
        } else {
            sprite.get_y()
        };

        let pos_x = if sprite.get_x() > self.get_screen_width() {
            sprite.get_x() % self.get_screen_width()
        } else {
            sprite.get_x()
        };

        for sy in 0..sprite.get_height() {
            let vy = pos_y.saturating_add(sy as usize);
            for sx in 0..sprite.get_width() {
                let vx = pos_x.saturating_add(sx as usize);

                let pixel = read_bit_from_byte(
                    sprite.get_data()[sy as usize],
                    (7 as u8).wrapping_sub(sx as u8),
                );

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
