use crate::constants;

///
/// Video Memory
///
#[derive(Debug)]
pub struct VideoMemory {
    data: Box<[[u8; constants::SCREEN_WIDTH]; constants::SCREEN_HEIGHT]>,
}

impl VideoMemory {
    pub fn new() -> Self {
        Self {
            data: Box::new([[0; constants::SCREEN_WIDTH]; constants::SCREEN_HEIGHT]),
        }
    }

    pub fn read(&self, x: usize, y: usize) -> &u8 {
        &self.data[y][x]
    }

    pub fn write(&mut self, x: usize, y: usize, value: u8) {
        self.data[y][x] = value;
    }

    pub fn clear(&mut self) {
        self.data = Box::new([[0; constants::SCREEN_WIDTH]; constants::SCREEN_HEIGHT]);
    }
}
