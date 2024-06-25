///
/// Registers
///
#[derive(Debug)]
pub struct Registers {
    data: [u8; 16],
}

impl Default for Registers {
    fn default() -> Self {
        Self { data: [0; 16] }
    }
}

impl Registers {
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
        self.data = [0; 16];
    }
}
