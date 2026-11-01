use crate::constants;

///
/// Video Memory
///
#[derive(Debug)]
#[must_use]
pub struct VideoMemory {
    pub hires_mode: bool,
    pub vram_changed: bool,
    pub data: [[u8; constants::SCREEN_WIDTH]; constants::SCREEN_HEIGHT],
}

impl VideoMemory {
    pub fn new() -> Self {
        Self {
            hires_mode: false,
            vram_changed: false,
            data: [[0; constants::SCREEN_WIDTH]; constants::SCREEN_HEIGHT],
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
}
