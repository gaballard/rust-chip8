use std::collections::HashMap;

use sdl2::rect::Rect;

use crate::{constants, models::Sprite};

///
/// Video Buffer
///
#[derive(Debug)]
pub struct VideoBuffer {
    pub data: Box<HashMap<(usize, usize), u8>>,
    pub rects: Box<Vec<Rect>>,
}

impl VideoBuffer {
    pub fn new() -> Self {
        Self {
            data: Box::new(HashMap::new()),
            rects: Box::new(Vec::new()),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    // pub fn data(&self) -> &Box<HashMap<(usize, usize), u8>> {
    //     &self.data
    // }

    #[allow(dead_code)]
    pub fn read(&self, x: usize, y: usize) -> Option<&u8> {
        self.data.get(&(x, y))
    }

    pub fn write(&mut self, x: usize, y: usize, value: u8) {
        self.data.insert((x, y), value);
        self.rects.push(Rect::new(
            (x * constants::VIDEO_SCALE) as i32,
            (y * constants::VIDEO_SCALE) as i32,
            constants::VIDEO_SCALE as u32,
            constants::VIDEO_SCALE as u32,
        ));
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

///
/// Video Memory
///
#[derive(Debug)]
pub struct VideoMemory<'a> {
    data: Box<[[u8; constants::SCREEN_WIDTH]; constants::SCREEN_HEIGHT]>,
    pub buffer: VideoBuffer,
    pub sprites: Vec<Sprite<'a>>,
}

impl<'a> VideoMemory<'a> {
    pub fn new() -> Self {
        Self {
            data: Box::new([[0; constants::SCREEN_WIDTH]; constants::SCREEN_HEIGHT]),
            buffer: VideoBuffer::new(),
            sprites: Vec::new(),
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
