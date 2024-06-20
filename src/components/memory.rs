use crate::constants;

///
/// Memory
///
#[derive(Debug)]
pub struct Memory {
    data: [u8; constants::SYSTEM_RAM as usize],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: [0; constants::SYSTEM_RAM as usize],
        }
    }

    #[inline]
    pub fn read(&self, addr: u16) -> &u8 {
        &self.data[addr as usize]
    }

    #[inline]
    pub fn read_slice(&self, addr: u16, len: u16) -> &[u8] {
        &self.data[addr as usize..(addr + len) as usize]
    }

    #[inline]
    pub fn write(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }

    #[inline]
    pub fn clear(&mut self) {
        self.data = [0; constants::SYSTEM_RAM as usize];
    }
}
