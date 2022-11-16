// use crate::fonts::FONT_SET;

// const SYSTEM_RAM: usize = 4092;

// pub struct Memory {
//   ram: [usize; SYSTEM_RAM],
// }

// impl Memory {
//   pub fn new() -> Memory {
//     let mut memory = Memory {
//       ram: [0; SYSTEM_RAM],
//     };

//     let mut i = 0;
//     for font_char in &FONT_SET {
//       memory.ram[i] = *font_char;
//       i += 1;
//     }

//     memory
//   }

//   pub fn write_ram_byte(&mut self, address: usize, value: usize) {
//     self.ram[address] = value;
//   }

//   pub fn read_ram_byte(&self, address: usize) -> usize {
//     self.ram[address]
//   }
// }
