///
/// Input Buffer
///
#[derive(Debug)]
pub struct InputBuffer {
    buffer: Box<[bool; 16]>,
}

impl InputBuffer {
    #[inline]
    pub fn new() -> Self {
        Self {
            buffer: Box::new([false; 16]),
        }
    }

    #[inline]
    pub fn get(&self, key: usize) -> &bool {
        &self.buffer[key]
    }

    #[inline]
    pub fn set(&mut self, key: usize, is_pressed: bool) {
        self.buffer[key] = is_pressed;
    }

    #[inline]
    pub fn clear(&mut self) {
        self.buffer = Box::new([false; 16]);
    }
}
