use crate::constants;

///
/// Memory
///
#[derive(Debug)]
pub struct Memory {
    data: Box<[u8; constants::SYSTEM_RAM as usize]>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: Box::new([0; constants::SYSTEM_RAM as usize]),
        }
    }

    pub fn read(&self, addr: u16) -> &u8 {
        &self.data[addr as usize]
    }

    pub fn read_slice(&self, addr: u16, len: u16) -> &[u8] {
        &self.data[addr as usize..(addr + len) as usize]
    }

    pub fn read_bit_from_byte(&self, byte: &u8, bit_position: u8) -> &u8 {
        if bit_position < 8 {
            if byte & (1 << bit_position) != 0 {
                &1
            } else {
                &0
            }
        } else {
            &0
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }

    pub fn clear(&mut self) {
        self.data = Box::new([0; constants::SYSTEM_RAM as usize]);
    }
}

///
/// Video Memory
///
#[derive(Debug)]
pub struct VideoMemory {
    data: Box<[[u8; constants::SCREEN_WIDTH]; constants::SCREEN_HEIGHT]>,
}

impl VideoMemory {
    pub fn new() -> Self {
        VideoMemory {
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

///
/// Input Buffer
///
#[derive(Debug)]
pub struct InputBuffer {
    buffer: Box<[bool; 16]>,
}

impl InputBuffer {
    pub fn new() -> Self {
        Self {
            buffer: Box::new([false; 16]),
        }
    }

    pub fn get_all(&self) -> &Box<[bool; 16]> {
        &self.buffer
    }

    pub fn get(&self, key: usize) -> &bool {
        &self.buffer[key]
    }

    pub fn set(&mut self, key: usize, is_pressed: bool) {
        self.buffer[key] = is_pressed;
    }

    pub fn clear(&mut self) {
        self.buffer = Box::new([false; 16]);
    }
}
