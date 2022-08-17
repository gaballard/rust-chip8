use fonts::FONT_SET;

use SYSTEM_RAM;
use SYSTEM_VRAM;

use SCREEN_HEIGHT;
use SCREEN_WIDTH;

pub struct Memory {
  ram: [u8; SYSTEM_RAM];
  vram: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT];
  vram_changed: bool;
}

impl Memory {
  pub fn new() -> Memory {
    let mut memory = Memory {
      ram: [0; SYSTEM_RAM]
      vram: [[0; SCREEN_WIDTH]; SCREEN_HEIGHT]
      vram_changed: false;
    }

    let mut i = 0;
    for font_char in &FONT_SET {
      memory.ram[i] = *font_char;
      i += 1;
    }

    memory
  }

  pub fn write_ram_byte(&mut self, address: u16, value: u8) {
    self.ram[address as usize] = value;
  }

  pub fn read_ram_byte(&self, address: u16) -> u8 {
    self.ram[address as usize]
  }

  pub fn write_vram_byte(&mut self, x: u16, y: u16, value: u8) {
    self.vram[x, y] = value;
  }

  pub fn read_vram_byte(&self, x: u16, value: u8) -> u8 {
    self.vram[x, y]
  }

  pub fn set_vram_changed(&mut self, value: bool) {
    self.vram_changed = value;
  }
}