///
/// Registers
///
#[derive(Debug)]
pub struct Registers {
    data: Box<[u8; 16]>,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            data: Box::new([0; 16]),
        }
    }

    pub fn read(&self, register: u8) -> &u8 {
        &self.data[register as usize]
    }

    pub fn write(&mut self, register: u8, value: u8) {
        self.data[register as usize] = value;
    }

    pub fn clear(&mut self) {
        self.data = Box::new([0; 16]);
    }
}
