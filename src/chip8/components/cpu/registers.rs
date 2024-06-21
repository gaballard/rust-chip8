///
/// Registers
///
#[derive(Debug)]
pub struct Registers {
    data: Box<[u8; 16]>,
}

impl Registers {
    #[inline]
    pub fn new() -> Self {
        Registers {
            data: Box::new([0; 16]),
        }
    }

    #[inline]
    pub fn read(&self, register: u8) -> &u8 {
        &self.data[register as usize]
    }

    #[inline]
    pub fn write(&mut self, register: u8, value: u8) {
        self.data[register as usize] = value;
    }

    #[inline]
    pub fn clear(&mut self) {
        self.data = Box::new([0; 16]);
    }
}
