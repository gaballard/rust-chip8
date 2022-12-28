use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH, SYSTEM_RAM};

///
/// Memory
///
#[derive(Debug)]
pub struct Memory {
    data: Box<[u8; SYSTEM_RAM as usize]>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: Box::new([0; SYSTEM_RAM as usize]),
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
        self.data = Box::new([0; SYSTEM_RAM as usize]);
    }
}

///
/// Video Memory
///
#[derive(Debug)]
pub struct VideoMemory {
    data: Box<[[u8; SCREEN_WIDTH]; SCREEN_HEIGHT]>,
}

impl VideoMemory {
    pub fn new() -> Self {
        VideoMemory {
            data: Box::new([[0; SCREEN_WIDTH]; SCREEN_HEIGHT]),
        }
    }

    pub fn read(&self, x: usize, y: usize) -> &u8 {
        &self.data[y][x]
    }

    pub fn write(&mut self, x: usize, y: usize, value: u8) {
        self.data[y][x] = value;
    }

    pub fn clear(&mut self) {
        self.data = Box::new([[0; SCREEN_WIDTH]; SCREEN_HEIGHT]);
    }
}

///
/// Program Stack
///
#[derive(Debug)]
pub struct Stack {
    data: [u16; 16],
    pointer: u8,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            data: [0; 16],
            pointer: 0,
        }
    }

    pub fn push(&mut self, value: u16) {
        if self.data.len() < 16 {
            self.data[self.pointer as usize] = value;
            self.pointer += 1;
        }
    }

    pub fn pop(&mut self) -> u16 {
        if self.data.len() > 0 && self.pointer > 0 {
            self.pointer -= 1;
            self.data[self.pointer as usize]
        } else {
            self.pointer = 0;
            self.data[0]
        }
    }

    pub fn clear(&mut self) {
        self.data = [0; 16];
        self.pointer = 0;
    }
}

///
/// Input Buffer
///
#[derive(Debug)]
pub struct InputBuffer {
    keys: Box<[bool; 16]>,
}

impl InputBuffer {
    pub fn new() -> Self {
        Self {
            keys: Box::new([false; 16]),
        }
    }

    pub fn get_all(&self) -> &Box<[bool; 16]> {
        &self.keys
    }

    pub fn get(&self, key: usize) -> &bool {
        &self.keys[key]
    }

    #[allow(dead_code)]
    pub fn set(&mut self, key: usize, is_pressed: bool) {
        self.keys[key] = is_pressed;
    }

    #[allow(dead_code)]
    pub fn toggle(&mut self, key: usize) {
        self.keys[key] = !self.keys[key];
    }

    pub fn clear(&mut self) {
        self.keys = Box::new([false; 16]);
    }
}
