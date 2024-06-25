use crate::constants;

use super::Storage;

///
/// Memory
///
#[derive(Debug)]
#[must_use]
pub struct Memory {
    data: [u8; constants::SYSTEM_RAM as usize],
}

impl Storage for Memory {
    #[inline]
    fn read(&self, addr: u16) -> &u8 {
        &self.data[addr as usize]
    }

    #[inline]
    fn write(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }

    #[inline]
    fn clear(&mut self) {
        self.data = [0; constants::SYSTEM_RAM as usize];
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            data: [0; constants::SYSTEM_RAM as usize],
        }
    }
}

impl Memory {
    #[inline]
    pub fn read_slice(&self, addr: u16, len: u16) -> &[u8] {
        &self.data[addr as usize..(addr + len) as usize]
    }
}
