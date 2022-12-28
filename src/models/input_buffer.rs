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
