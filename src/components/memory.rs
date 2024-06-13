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
            // data: Box::new([0; constants::SYSTEM_RAM as usize]),
            data: [0; constants::SYSTEM_RAM as usize]
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

    #[allow(dead_code)]
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

    #[inline]
    pub fn write(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }

    #[inline]
    pub fn clear(&mut self) {
        self.data = [0; constants::SYSTEM_RAM as usize];
        // self.data = Box::new([0; constants::SYSTEM_RAM as usize]);
    }
}
