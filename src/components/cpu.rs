use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::thread;
use std::time::Duration;

use crate::fonts::FONT_SET;
use crate::EMULATOR_NAME;
use crate::SCREEN_HEIGHT;
use crate::SCREEN_WIDTH;

const SYSTEM_SPEED_HZ: u64 = 60;
const SYSTEM_RAM: usize = 4092;
const PROGRAM_START: usize = 0x200;
const OPCODE_SIZE: usize = 2;

pub struct Cpu {
  tick_duration: Duration,
  tick: u16,
  ram: [usize; SYSTEM_RAM],
  vram: [[usize; SCREEN_WIDTH]; SCREEN_HEIGHT],
  vram_changed: bool,
  stack: [usize; 16],
  v: [usize; 16],
  i: usize,
  pc: usize,
  sc: usize,
  rng: ThreadRng,
  dt: usize,
  st: usize,
  keypad: [bool; 16],
  event_pump: EventPump,
  canvas: Canvas<Window>,
}

impl Cpu {
  pub fn new() -> Cpu {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
      .window(&EMULATOR_NAME, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
      .position_centered()
      .build()
      .unwrap();
    let canvas = window.into_canvas().build().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();

    let mut cpu = Cpu {
      tick_duration: Duration::from_millis(1000 / SYSTEM_SPEED_HZ),
      tick: 0,
      ram: [0; SYSTEM_RAM],
      vram: [[0; SCREEN_WIDTH]; SCREEN_HEIGHT],
      vram_changed: false,
      v: [0; 16],
      pc: PROGRAM_START,
      sc: 0,
      i: 0,
      stack: [0; 16],
      rng: thread_rng(),
      dt: 0,
      st: 0,
      keypad: [false; 16],
      event_pump,
      canvas,
    };

    let mut i = 0;
    for font_char in &FONT_SET {
      cpu.ram[i] = *font_char;
      i += 1;
    }

    cpu
  }

  pub fn load_program(&mut self, data: Vec<u8>) {
    let mut addr = PROGRAM_START;
    for v in &data {
      self.write_ram_byte(addr, *v as usize);
      addr += 1;
    }
  }

  pub fn init_video(&mut self) {
    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
    self.canvas.clear();
    self.canvas.present();
  }

  pub fn emulate(&mut self) {
    'running: loop {
      for event in self.event_pump.poll_iter() {
        match event {
          Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
          } => {
            break 'running;
          }
          Event::KeyDown {
            keycode: Some(Keycode::Num1),
            ..
          } => {
            self.keypad[0] = true;
          }
          Event::KeyDown {
            keycode: Some(Keycode::Num2),
            ..
          } => {
            self.keypad[1] = true;
          }
          Event::KeyDown {
            keycode: Some(Keycode::Num3),
            ..
          } => {
            self.keypad[2] = true;
          }
          Event::KeyDown {
            keycode: Some(Keycode::Num4),
            ..
          } => {
            self.keypad[3] = true;
          }
          Event::KeyDown {
            keycode: Some(Keycode::Q),
            ..
          } => {
            self.keypad[4] = true;
          }
          Event::KeyDown {
            keycode: Some(Keycode::W),
            ..
          } => {
            self.keypad[5] = true;
          }
          Event::KeyDown {
            keycode: Some(Keycode::E),
            ..
          } => {
            self.keypad[6] = true;
          }
          Event::KeyDown {
            keycode: Some(Keycode::R),
            ..
          } => {
            self.keypad[7] = true;
          }
          Event::KeyDown {
            keycode: Some(Keycode::A),
            ..
          } => {
            self.keypad[8] = true;
          }
          Event::KeyDown {
            keycode: Some(Keycode::S),
            ..
          } => {
            self.keypad[9] = true;
          }
          Event::KeyDown {
            keycode: Some(Keycode::F),
            ..
          } => {
            self.keypad[10] = true;
          }
          Event::KeyDown {
            keycode: Some(Keycode::D),
            ..
          } => {
            self.keypad[11] = true;
          }
          Event::KeyDown {
            keycode: Some(Keycode::Z),
            ..
          } => {
            self.keypad[12] = true;
          }
          Event::KeyDown {
            keycode: Some(Keycode::X),
            ..
          } => {
            self.keypad[13] = true;
          }
          Event::KeyDown {
            keycode: Some(Keycode::C),
            ..
          } => {
            self.keypad[14] = true;
          }
          Event::KeyDown {
            keycode: Some(Keycode::V),
            ..
          } => {
            self.keypad[15] = true;
          }
          _ => {}
        }
      }

      self.emulate_cycle();
    }
  }

  fn draw_vram_to_display(&mut self) {
    let mut x = 0;
    let mut y = 0;
    let mut points: Vec<Point> = Vec::new();
    while x < SCREEN_WIDTH {
      while y < SCREEN_HEIGHT {
        if self.vram[x][y] == 1 {
          let p = Point::new(x as i32, y as i32);
          points.push(p);
        }
        y += 1;
      }
      x += 1;
    }
    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
    self.canvas.clear();
    self.canvas.set_draw_color(Color::RGB(255, 255, 255));
    self.canvas.draw_points(&points[..]).unwrap();
    self.canvas.present();
  }

  fn emulate_cycle(&mut self) {
    self.process_next_instruction();
    self.draw_vram_to_display();
    self.tick();
    thread::sleep(self.tick_duration);
  }

  fn tick(&mut self) {
    self.tick += 1;
    self.tick_delay_timer();
    self.tick_sound_timer();
  }

  fn set_delay_timer(&mut self, duration: usize) {
    self.dt = duration;
  }

  fn tick_delay_timer(&mut self) {
    if self.dt > 0 {
      self.dt -= 1;
    }
  }

  fn set_sound_timer(&mut self, duration: usize) {
    self.st = duration;
  }

  fn tick_sound_timer(&mut self) {
    if self.st > 0 {
      self.st -= 1;
    }
  }

  fn write_register(&mut self, index: usize, value: usize) {
    self.v[index] = value;
  }

  fn read_register(&self, index: usize) -> usize {
    self.v[index]
  }

  fn write_ram_byte(&mut self, address: usize, value: usize) {
    self.ram[address] = value;
  }

  fn read_ram_byte(&self, address: usize) -> usize {
    self.ram[address]
  }

  // fn read_ram_bytes(&self, address: usize, length: usize) -> &[usize] {
  //   &self.ram[address..address + length]
  // }

  fn write_vram_byte(&mut self, x: usize, y: usize, value: usize) {
    self.vram[x][y] = value;
    self.vram_changed = true;
  }

  // fn read_vram_byte(&self, x: usize, y: usize) -> &usize {
  //   &self.vram[x][y]
  // }

  fn read_bit_from_byte(&self, byte: &usize, bit_position: u8) -> &u8 {
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

  fn stack_push(&mut self, value: usize) {
    self.sc += 1;
    self.stack[self.sc - 1] = value;
  }

  fn stack_pop(&mut self) -> usize {
    let value = self.stack[self.sc - 1];
    self.stack[self.sc - 1] = 0;
    self.sc -= 1;
    value
  }

  fn pc_next(&mut self) {
    self.pc += OPCODE_SIZE;
  }

  fn process_next_instruction(&mut self) {
    let instruction = self.read_ram_byte(self.pc) + self.read_ram_byte(self.pc + 2);

    println!("Processing instruction: {}", instruction);

    match instruction & 0xF000 {
      0x0 => match instruction & 0x00FF {
        0xE0 => {
          println!("00E0 - CLS");
          // Clear the display.
          let mut x = 0;
          let mut y = 0;
          for _ in self.vram {
            for _ in self.vram[x] {
              self.write_vram_byte(x, y, 0);
              y += 1;
            }
            x += 1;
          }
        }
        0xEE => {
          println!("00EE - RET");
          // The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
          self.pc = self.stack_pop();
        }
        _ => println!("Invalid instruction: {}", instruction),
      },
      0x1000 => {
        println!("1nnn - JP addr: {}", (instruction & 0x0FFF));
        // The interpreter sets the program counter to nnn.
        self.pc = instruction & 0x0FFF;
      }
      0x2000 => {
        println!("2nnn - CALL addr: {}", (instruction & 0x0FFF));
        // The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
        self.stack_push(self.pc);
        self.pc = instruction & 0x0FFF;
      }
      0x3000 => {
        println!(
          "3xkk - SE V{} byte: {}",
          (instruction & 0x0F00),
          (instruction & 0x00FF)
        );
        // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
        let x = instruction & 0x0F00;
        let kk = instruction & 0x00FF;
        if self.read_register(x) == kk {
          self.pc_next();
        }
      }
      0x4000 => {
        println!(
          "4xkk - SNE V{} byte: {}",
          (instruction & 0x0F00),
          (instruction & 0x00FF)
        );
        // The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
        let x = instruction & 0x0F00;
        let kk = instruction & 0x00FF;
        if self.read_register(x) != kk {
          self.pc_next();
        }
      }
      0x5000 => match instruction & 0x000F {
        0x0 => {
          println!(
            "5xy0 - SE V{}, V{}",
            (instruction & 0x0F00),
            (instruction & 0x00F0)
          );
          // The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
          let x = instruction & 0x0F00;
          let y = instruction & 0x00F0;
          if self.read_register(x) == self.read_register(y) {
            self.pc_next();
          }
        }
        _ => println!("Invalid instruction: {}", instruction),
      },
      0x6000 => {
        println!(
          "6xkk - LD V{} byte: {}",
          (instruction & 0x0F00),
          (instruction & 0x00FF)
        );
        let x = instruction & 0x0F00;
        let kk = instruction & 0x00FF;
        // The interpreter puts the value kk into register Vx.
        self.write_register(x, kk);
      }
      0x7000 => {
        println!(
          "7xkk - ADD V{} byte: {}",
          (instruction & 0x0F00),
          (instruction & 0x00FF)
        );
        let x = instruction & 0x0F00;
        let kk = instruction & 0x00FF;
        // Adds the value kk to the value of register Vx, then stores the result in Vx.
        self.write_register(x, self.read_register(x) + kk);
      }
      0x8000 => match instruction & 0x000F {
        0x0 => {
          println!(
            "8xy0 - LD V{}, V{}",
            (instruction & 0x0F00),
            (instruction & 0x00F0)
          );
          // Stores the value of register Vy in register Vx.
          let x = instruction & 0x0F00;
          let y = instruction & 0x00F0;
          self.write_register(x, self.read_register(y));
        }
        0x1 => {
          println!(
            "8xy1 - OR V{}, V{}",
            (instruction & 0x0F00),
            (instruction & 0x00F0)
          );
          // Set Vx = Vx OR Vy.
          let x = instruction & 0x0F00;
          let y = instruction & 0x00F0;
          self.write_register(x, self.read_register(x) | self.read_register(y));
        }
        0x2 => {
          println!(
            "8xy2 - AND V{}, V{}",
            (instruction & 0x0F00),
            (instruction & 0x00F0)
          );
          // Set Vx = Vx AND Vy.
          let x = instruction & 0x0F00;
          let y = instruction & 0x00F0;
          self.write_register(x, self.read_register(x) & self.read_register(y));
        }
        0x3 => {
          println!(
            "8xy3 - XOR V{}, V{}",
            (instruction & 0x0F00),
            (instruction & 0x00F0)
          );
          // Set Vx = Vx XOR Vy.
          let x = instruction & 0x0F00;
          let y = instruction & 0x00F0;
          self.write_register(x, self.read_register(x) ^ self.read_register(y));
        }
        0x4 => {
          println!(
            "8xy4 - ADD V{}, V{}",
            (instruction & 0x0F00),
            (instruction & 0x00F0)
          );
          // Set Vx = Vx + Vy, set VF = carry.
          let x = instruction & 0x0F00;
          let y = instruction & 0x00F0;
          let sum: usize = self.read_register(x) + self.read_register(y);
          if sum > 255 {
            self.write_register(x, 255);
            self.write_register(0xF, 255);
          } else {
            self.write_register(x, sum);
            self.write_register(0xF, 0);
          }
        }
        0x5 => {
          println!(
            "8xy5 - SUB V{}, V{}",
            (instruction & 0x0F00),
            (instruction & 0x00F0)
          );
          // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
          let x = instruction & 0x0F00;
          let y = instruction & 0x00F0;
          if self.read_register(x) > self.read_register(y) {
            self.write_register(0xF, 1);
          } else {
            self.write_register(0xF, 0);
          }
          self.write_register(x, self.read_register(x) - self.read_register(y));
        }
        0x6 => {
          println!(
            "8xy6 - SHR V{}, V{}",
            (instruction & 0x0F00),
            (instruction & 0x00F0)
          );
          // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
          let x = instruction & 0x0F00;
          if self.read_register(x) & 0b0000000000000001 == 1 {
            self.write_register(0xF, 1);
          } else {
            self.write_register(0xF, 0);
          }
          self.write_register(x, self.read_register(x) / 2);
        }
        0x7 => {
          println!(
            "8xy7 - SUBN V{}, V{}",
            (instruction & 0x0F00),
            (instruction & 0x00F0)
          );
          // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
          let x = instruction & 0x0F00;
          let y = instruction & 0x00F0;
          if self.read_register(y) > self.read_register(x) {
            self.write_register(0xF, 1);
          } else {
            self.write_register(0xF, 0);
          }
          self.write_register(x, self.read_register(y) - self.read_register(x));
        }
        0xE => {
          println!(
            "8xyE - SHL V{}, V{}",
            (instruction & 0x0F00),
            (instruction & 0x00F0)
          );
          // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
          let x = instruction & 0x0F00;
          if self.read_register(x) & 0b1 == 1 {
            self.write_register(0xF, 1);
          } else {
            self.write_register(0xF, 0);
          }
          self.write_register(x, self.read_register(x) * 2);
        }
        _ => println!("Invalid instruction: {}", instruction),
      },
      0x9000 => match instruction & 0x000F {
        0x0 => {
          println!(
            "9xy0 - SNE V{}, V{}",
            (instruction & 0x0F00),
            (instruction & 0x00F0)
          );
          // The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
          let x = instruction & 0x0F00;
          let y = instruction & 0x00F0;
          if self.read_register(x) != self.read_register(y) {
            self.pc_next();
          }
        }
        _ => println!("Invalid instruction: {}", instruction),
      },
      0xA000 => {
        println!("Annn - LD I addr: {}", (instruction & 0x0FFF));
        // The value of register I is set to nnn.
        self.i = instruction & 0x0FFF;
      }
      0xB000 => {
        println!("Bnnn - JP V0 addr: {}", (instruction & 0x0FFF));
        // The program counter is set to nnn plus the value of V0.
        self.pc = self.read_register(0) + instruction & 0x0FFF;
      }
      0xC000 => {
        println!(
          "Cxkk - RND V{}, byte: {}",
          (instruction & 0x0F00),
          (instruction & 0x00FF)
        );
        // The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx.
        let x = instruction & 0x0F00;
        let kk = instruction & 0x00FF;
        let rnum: usize = self.rng.gen();
        self.write_register(x, rnum & kk);
      }
      0xD000 => {
        println!(
          "Dxyn - DRW V{}, V{}, nibble: {}",
          (instruction & 0x0F00),
          (instruction & 0x00F0),
          (instruction & 0x000F)
        );
        // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
        let sx = instruction & 0x0F00;
        let sy = instruction & 0x00F0;
        let len = instruction & 0x000F;
        // let sprite = self.read_ram_bytes(self.i, len);
        let sprite = &self.ram[self.i..self.i + len];

        let mut y = 0;
        let mut collision = false;

        for byte in sprite {
          let mut x = 8;

          while x > 0 {
            let bit = self.read_bit_from_byte(byte, x - 1);
            let xor_res = self.vram[sx % SCREEN_WIDTH][sy % SCREEN_HEIGHT] as u8 ^ bit;
            if xor_res == 1 && !collision {
              collision = true;
            }
            self.vram[(sx % SCREEN_WIDTH) + x as usize][(sy % SCREEN_HEIGHT) + y as usize] =
              xor_res as usize;
            x -= 1;
          }
          y += 1;
        }

        if collision {
          self.v[0xF] = 0x1;
        }
      }
      0xE000 => match instruction & 0x00FF {
        0x9E => {
          println!("Ex9E - SKP V{}", (instruction & 0x0F00));
          // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
          if self.keypad[self.read_register(instruction & 0x0F00)] == true {
            self.pc_next();
          }
        }
        0xA1 => {
          println!("ExA1 - SKNP V{}", (instruction & 0x0F00));
          // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
          if self.keypad[self.read_register(instruction & 0x0F00)] == false {
            self.pc_next();
          }
        }
        _ => println!("Invalid instruction: {}", instruction),
      },
      0xF000 => match instruction & 0x00FF {
        0x07 => {
          println!("Fx07 - LD V{}, DT", (instruction & 0x0F00));
          // The value of DT is placed into Vx.
          self.write_register(instruction & 0x0F00, self.dt);
        }
        0x0A => {
          println!("Fx0A - LD V{}, K", (instruction & 0x0F00));
          // All execution stops until a key is pressed, then the value of that key is stored in Vx.
          let x = instruction & 0x0F00;

          'waiting_for_input: loop {
            for event in self.event_pump.poll_iter() {
              match event {
                Event::KeyDown {
                  keycode: Some(Keycode::Num1),
                  ..
                } => {
                  self.v[x] = 0;
                  break 'waiting_for_input;
                }
                Event::KeyDown {
                  keycode: Some(Keycode::Num2),
                  ..
                } => {
                  self.v[x] = 1;
                  break 'waiting_for_input;
                }
                Event::KeyDown {
                  keycode: Some(Keycode::Num3),
                  ..
                } => {
                  self.v[x] = 2;
                  break 'waiting_for_input;
                }
                Event::KeyDown {
                  keycode: Some(Keycode::Num4),
                  ..
                } => {
                  self.v[x] = 3;
                  break 'waiting_for_input;
                }
                Event::KeyDown {
                  keycode: Some(Keycode::Q),
                  ..
                } => {
                  self.v[x] = 4;
                  break 'waiting_for_input;
                }
                Event::KeyDown {
                  keycode: Some(Keycode::W),
                  ..
                } => {
                  self.v[x] = 5;
                  break 'waiting_for_input;
                }
                Event::KeyDown {
                  keycode: Some(Keycode::E),
                  ..
                } => {
                  self.v[x] = 6;
                  break 'waiting_for_input;
                }
                Event::KeyDown {
                  keycode: Some(Keycode::R),
                  ..
                } => {
                  self.v[x] = 7;
                  break 'waiting_for_input;
                }
                Event::KeyDown {
                  keycode: Some(Keycode::A),
                  ..
                } => {
                  self.v[x] = 8;
                  break 'waiting_for_input;
                }
                Event::KeyDown {
                  keycode: Some(Keycode::S),
                  ..
                } => {
                  self.v[x] = 9;
                  break 'waiting_for_input;
                }
                Event::KeyDown {
                  keycode: Some(Keycode::F),
                  ..
                } => {
                  self.v[x] = 10;
                  break 'waiting_for_input;
                }
                Event::KeyDown {
                  keycode: Some(Keycode::D),
                  ..
                } => {
                  self.v[x] = 11;
                  break 'waiting_for_input;
                }
                Event::KeyDown {
                  keycode: Some(Keycode::Z),
                  ..
                } => {
                  self.v[x] = 12;
                  break 'waiting_for_input;
                }
                Event::KeyDown {
                  keycode: Some(Keycode::X),
                  ..
                } => {
                  self.v[x] = 13;
                  break 'waiting_for_input;
                }
                Event::KeyDown {
                  keycode: Some(Keycode::C),
                  ..
                } => {
                  self.v[x] = 14;
                  break 'waiting_for_input;
                }
                Event::KeyDown {
                  keycode: Some(Keycode::V),
                  ..
                } => {
                  self.v[x] = 15;
                  break 'waiting_for_input;
                }
                _ => {}
              }
            }
          }
        }
        0x15 => {
          println!("Fx15 - LD DT V{}", (instruction & 0x0F00));
          // DT is set equal to the value of Vx.
          self.set_delay_timer(self.read_register(instruction & 0x0F00));
        }
        0x18 => {
          println!("Fx18 - LD ST V{}", (instruction & 0x0F00));
          // ST is set equal to the value of Vx.
          self.set_sound_timer(self.read_register(instruction & 0x0F00));
        }
        0x1E => {
          println!("Fx1E - ADD I V{}", (instruction & 0x0F00));
          // The values of I and Vx are added, and the results are stored in I.
          self.i += self.read_register(instruction & 0x0F00);
        }
        0x29 => {
          println!("Fx29 - LD F V{}", (instruction & 0x0F00));
          // The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx.
          self.i = 0x0 + (self.read_register(instruction & 0x0F00) * 5);
        }
        0x33 => {
          println!("Fx33 - LD B V{}", (instruction & 0x0F00));
          // The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
          let xs = self.read_register(instruction & 0x0F00).to_string();
          self.write_ram_byte(self.i, (&xs[2..2]).parse::<usize>().unwrap());
          self.write_ram_byte(self.i + 1, (&xs[1..1]).parse::<usize>().unwrap());
          self.write_ram_byte(self.i + 2, (&xs[..0]).parse::<usize>().unwrap());
        }
        0x55 => {
          println!("Fx55 - LD [I] V{}", (instruction & 0x0F00));
          // The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
          let mut i = 0;
          while i < instruction & 0x0F00 {
            self.write_ram_byte(self.i + (i * 2), self.read_register(i));
            i += 1;
          }
        }
        0x65 => {
          println!("Fx65 - LD V{}, [I]", (instruction & 0x0F00));
          // The interpreter reads values from memory starting at location I into registers V0 through Vx.
          let mut i = 0;
          while i < instruction & 0x0F00 {
            self.write_register(i, self.read_ram_byte(self.i + (i * 2)));
            i += 1;
          }
        }
        _ => println!("Invalid instruction: {}", instruction),
      },
      _ => println!("Invalid instruction: {}", instruction),
    }
  }
}
