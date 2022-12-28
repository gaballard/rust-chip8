extern crate sdl2;

use rand::*;

use crate::components::{InputBuffer, Memory, Stack, VideoMemory};
use crate::constants;
use crate::fonts::CHIP8_FONTS;
use crate::peripherals::Display;

const DEFAULT_CLOCK_SPEED: u8 = 60;
const FONT_START_ADDR: u16 = 0x50;
const PROGRAM_START_ADDR: u16 = 0x200;
const OPCODE_SIZE: u16 = 2;

///
/// Program Counter
///
// #[derive(Debug)]
// pub struct ProgramCounter {
//     pub addr: u16,
// }

// impl ProgramCounter {
//     pub fn new() -> Self {
//         ProgramCounter {
//             addr: PROGRAM_START_ADDR,
//         }
//     }

//     pub fn next(&mut self) {
//         self = self.wrapping_add(OPCODE_SIZE);
//     }

//     pub fn jump(&mut self, jump_addr: u16) {
//         self = jump_addr;
//     }

//     pub fn clear(&mut self) {
//         self = PROGRAM_START_ADDR;
//     }
// }

///
/// Registers
///
#[derive(Debug)]
pub struct Registers {
    data: [u8; 16],
}

impl Registers {
    pub fn new() -> Self {
        Registers { data: [0; 16] }
    }

    pub fn read(&self, register: u8) -> &u8 {
        &self.data[register as usize]
    }

    pub fn write(&mut self, register: u8, value: u8) {
        self.data[register as usize] = value;
    }

    pub fn clear(&mut self) {
        self.data = [0; 16];
    }
}

///
/// CPU
///
pub struct Cpu {
    instruction: u16,
    clock_speed: u8,
    ram: Memory,
    // ram: Box<[u8; SYSTEM_RAM as usize]>,
    vram: VideoMemory,
    v: Registers,
    i: u16,
    pc: u16,
    stack: Stack,
    delay_timer: u8,
    delay_tick: f32,
    sound_timer: u8,
    sound_tick: f32,
    rng: rand::rngs::ThreadRng,
    keys: InputBuffer,
    display: Display,
}

impl Cpu {
    pub fn new(display: Display) -> Cpu {
        let mut cpu = Cpu {
            instruction: 0,
            clock_speed: DEFAULT_CLOCK_SPEED,
            ram: Memory::new(),
            // ram: Box::new([0; SYSTEM_RAM as usize]),
            vram: VideoMemory::new(),
            v: Registers::new(),
            i: PROGRAM_START_ADDR,
            pc: PROGRAM_START_ADDR,
            stack: Stack::new(),
            rng: rand::thread_rng(),
            delay_timer: 0,
            delay_tick: 0.0,
            sound_timer: 0,
            sound_tick: 0.0,
            keys: InputBuffer::new(),
            display,
        };

        let mut i = FONT_START_ADDR;
        for font_char in &CHIP8_FONTS {
            cpu.ram.write(i, *font_char);
            // cpu.ram[i as usize] = *font_char;
            i += 1;
        }

        cpu
    }

    pub fn reset(&mut self) {
        self.instruction = 0;
        self.clock_speed = DEFAULT_CLOCK_SPEED;
        self.ram.clear();
        // self.ram = Box::new([0; SYSTEM_RAM as usize]);
        self.vram.clear();
        self.v.clear();
        self.i = PROGRAM_START_ADDR;
        self.pc = PROGRAM_START_ADDR;
        self.stack.clear();
        self.rng = rand::thread_rng();
        self.delay_timer = 0;
        self.delay_tick = 0.0;
        self.sound_timer = 0;
        self.sound_tick = 0.0;
        self.keys.clear();
    }

    pub fn load_program(&mut self, data: Vec<u8>) {
        let mut addr = PROGRAM_START_ADDR;
        for v in &data {
            self.ram.write(addr, *v);
            // self.ram[addr] = *v;
            addr += 1;
        }
    }

    pub fn emulate_cycle(&mut self) {
        // CHIP-8 processes two instructions per clock cycle
        for _ in 0..1 {
            self.execute_instruction();
        }
        self.display.refresh(&self.vram);
    }

    pub fn update_timers(&mut self, delta_time: f32) {
        self.update_delay_timer(delta_time);
        self.update_sound_timer(delta_time);
    }

    fn update_delay_timer(&mut self, delta_time: f32) {
        if self.delay_timer > 0 {
            self.delay_tick -= delta_time;

            if self.delay_tick <= 0.0 {
                self.delay_timer -= 1;
                self.delay_tick = 1.0 / constants::TARGET_FPS as f32;
            }
        }
    }

    fn update_sound_timer(&mut self, delta_time: f32) {
        if self.sound_timer > 0 {
            self.sound_tick -= delta_time;

            if self.sound_tick <= 0.0 {
                self.delay_timer -= 1;
                self.sound_tick = 1.0 / constants::TARGET_FPS as f32;
            }
        }
    }

    fn next_instruction(&mut self) {
        self.pc += OPCODE_SIZE;
    }

    fn read_instruction(&self) -> u16 {
        (*self.ram.read(self.pc) as u16) << 8 | (*self.ram.read(self.pc + 1) as u16)
    }

    fn execute_instruction(&mut self) {
        self.instruction = self.read_instruction();

        self.next_instruction();

        let x = ((self.instruction & 0x0F00) >> 8).try_into().unwrap();
        let y = ((self.instruction & 0x00F0) >> 4).try_into().unwrap();

        let n = (self.instruction & 0x000F) as u8;
        let kk = (self.instruction & 0x00FF) as u8;
        let nnn = self.instruction & 0x0FFF;

        match self.instruction & 0xF000 {
            0x0 => match self.instruction & 0x00FF {
                0xE0 => {
                    println!("00E0 - CLS");
                    // Clear the display.
                    self.vram.clear();
                    self.display.set_refresh_flag(true);
                }
                0xEE => {
                    println!("00EE - RET");
                    // The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
                    self.pc = self.stack.pop();
                }
                _ => println!("Invalid instruction: {}", self.instruction),
            },
            0x1000 => {
                println!("1nnn - JP addr: {}", nnn);
                // The interpreter sets the program counter to nnn.
                self.pc = nnn;
            }
            0x2000 => {
                println!("2nnn - CALL addr: {}", nnn);
                // The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
                self.stack.push(self.pc);
                self.pc = nnn;
            }
            0x3000 => {
                println!("3xkk - SE V{} byte: {}", x, (kk));
                // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
                if self.v.read(x) == &kk {
                    self.next_instruction();
                }
            }
            0x4000 => {
                println!("4xkk - SNE V{} byte: {}", x, (kk));
                // The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
                if self.v.read(x) != &kk {
                    self.next_instruction();
                }
            }
            0x5000 => match self.instruction & 0x000F {
                0x0 => {
                    println!("5xy0 - SE V{}, V{}", x, y);
                    // The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
                    if self.v.read(x) == self.v.read(y) {
                        self.next_instruction();
                    }
                }
                _ => println!("Invalid instruction: {}", self.instruction),
            },
            0x6000 => {
                println!("6xkk - LD V{} byte: {}", x, (kk));

                // The interpreter puts the value kk into register Vx.
                self.v.write(x, kk);
            }
            0x7000 => {
                println!("7xkk - ADD V{} byte: {}", x, (kk));
                // Adds the value kk to the value of register Vx, then stores the result in Vx.
                self.v.write(x, self.v.read(x).wrapping_add(kk));
            }
            0x8000 => match self.instruction & 0x000F {
                0x0 => {
                    println!("8xy0 - LD V{}, V{}", x, y);
                    // Stores the value of register Vy in register Vx.
                    self.v.write(x, *self.v.read(y));
                }
                0x1 => {
                    println!("8xy1 - OR V{}, V{}", x, y);
                    // Set Vx = Vx OR Vy.
                    self.v.write(x, self.v.read(x) | self.v.read(y));
                }
                0x2 => {
                    println!("8xy2 - AND V{}, V{}", x, y);
                    // Set Vx = Vx AND Vy.
                    self.v.write(x, self.v.read(x) & self.v.read(y));
                }
                0x3 => {
                    println!("8xy3 - XOR V{}, V{}", x, y);
                    // Set Vx = Vx XOR Vy.
                    self.v.write(x, self.v.read(x) ^ self.v.read(y));
                }
                0x4 => {
                    println!("8xy4 - ADD V{}, V{}", x, y);
                    // Set Vx = Vx + Vy, set VF = carry.
                    let sum = (self.v.read(x) + self.v.read(y)) as u16;
                    self.v.write(x, sum.try_into().unwrap());
                    self.v.write(0xF, (sum > 0xFF) as u8);
                }
                0x5 => {
                    println!("8xy5 - SUB V{}, V{}", x, y);
                    // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
                    self.v.write(0xF, (self.v.read(x) > self.v.read(y)) as u8);
                    self.v
                        .write(x, self.v.read(x).wrapping_sub(*self.v.read(y)));
                }
                0x6 => {
                    println!("8xy6 - SHR V{}, V{}", x, y);
                    // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
                    if self.v.read(x) & 0b0000000000000001 == 1 {
                        self.v.write(0xF, 1);
                    } else {
                        self.v.write(0xF, 0);
                    }
                    self.v.write(x, self.v.read(x) / 2);
                }
                0x7 => {
                    println!("8xy7 - SUBN V{}, V{}", x, y);
                    // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
                    if self.v.read(y) > self.v.read(y) {
                        self.v.write(x, self.v.read(y) - self.v.read(x));
                        self.v.write(0xF, 1);
                    } else {
                        self.v.write(x, 0);
                        self.v.write(0xF, 0)
                    }
                }
                0xE => {
                    println!("8xyE - SHL V{}, V{}", x, y);
                    // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
                    if self.v.read(x) >> 7 == 0b1 {
                        self.v.write(0xF, 1);
                    } else {
                        self.v.write(0xF, 0)
                    }
                    self.v.write(x, self.v.read(x) << 1);
                }
                _ => println!("Invalid instruction: {}", self.instruction),
            },
            0x9000 => match self.instruction & 0x000F {
                0x0 => {
                    println!("9xy0 - SNE V{}, V{}", x, y);
                    // The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
                    if self.v.read(x) != self.v.read(y) {
                        self.next_instruction();
                    }
                }
                _ => println!("Invalid instruction: {}", self.instruction),
            },
            0xA000 => {
                println!("Annn - LD I addr: {}", nnn);
                // The value of register I is set to nnn.
                self.i = nnn;
            }
            0xB000 => {
                println!("Bnnn - JP V0 addr: {}", nnn);
                // The program counter is set to nnn plus the value of V0.
                self.pc = self.v.read(0).wrapping_add(nnn.try_into().unwrap()) as u16;
            }
            0xC000 => {
                println!("Cxkk - RND V{}, byte: {}", x, (kk));
                // The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx.
                let rnum: u8 = self.rng.gen();
                self.v.write(x, rnum & kk);
            }
            0xD000 => {
                println!("Dxyn - DRW V{}, V{}, nibble: {}", x, y, n);
                // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.

                let vx = *self.v.read(x) as usize;
                let vy = *self.v.read(y) as usize;
                let len = self.instruction & 0x000F;
                let sprite = self.ram.read_slice(self.i, len);

                println!("Draw the following Sprite at {},{}: {:?}", vx, vy, sprite);

                let mut sy: usize = 0;
                let mut sx: usize = 0;
                let mut collision = false;
                self.v.write(0xF, 0);

                while sy < len.try_into().unwrap() {
                    while sx < 8 {
                        let bit = self
                            .ram
                            .read_bit_from_byte(&sprite[sy as usize], 7 - sx as u8);

                        let mut bx = vx + sx;
                        if bx as usize > constants::SCREEN_WIDTH - 1 {
                            bx = bx % constants::SCREEN_WIDTH;
                        }

                        let mut by = vy + sy;
                        if by as usize > constants::SCREEN_HEIGHT - 1 {
                            by = by % constants::SCREEN_HEIGHT;
                        }

                        let xor_res = self.vram.read(bx, by) ^ bit;
                        if xor_res == 1 && !collision {
                            collision = true;
                        }

                        self.vram.write(bx, by, xor_res);
                        sx += 1;
                    }
                    sx = 0;
                    sy += 1;
                }

                if collision {
                    self.v.write(0xF, 1);
                }
            }
            0xE000 => match self.instruction & 0x00FF {
                0x9E => {
                    println!("Ex9E - SKP V{}", x);
                    // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
                    println!("Value of V{}: {}", x, self.v.read(x));
                    println!("Keypad state: {:?}", self.keys.get_all());
                    let key = *self.v.read(x) as usize;
                    if *self.keys.get(key) {
                        self.next_instruction();
                    }
                }
                0xA1 => {
                    println!("ExA1 - SKNP V{}", x);
                    // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
                    if !self.keys.get(*self.v.read(x) as usize) {
                        self.next_instruction();
                    }
                }
                _ => println!("Invalid instruction: {}", self.instruction),
            },
            0xF000 => match self.instruction & 0x00FF {
                0x07 => {
                    println!("Fx07 - LD V{}, DT", x);
                    // The value of DT is placed into Vx.
                    self.v.write(x, self.delay_timer);
                }
                0x0A => {
                    println!("Fx0A - LD V{}, K", x);
                    // All execution stops until a key is pressed, then the value of that key is stored in Vx.
                    //
                    //
                    //
                }
                0x15 => {
                    println!("Fx15 - LD DT V{}", x);
                    // DT is set equal to the value of Vx.
                    self.delay_timer = *self.v.read(x);
                }
                0x18 => {
                    println!("Fx18 - LD ST V{}", x);
                    // ST is set equal to the value of Vx.
                    self.sound_timer = *self.v.read(x);
                }
                0x1E => {
                    println!("Fx1E - ADD I V{}", x);
                    // The values of I and Vx are added, and the results are stored in I.
                    self.i = self.i.wrapping_add(*self.v.read(x) as u16);
                }
                0x29 => {
                    println!("Fx29 - LD F V{}", x);
                    // The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx.
                    self.i = FONT_START_ADDR.wrapping_add((*self.v.read(x) * 5) as u16);
                }
                0x33 => {
                    println!("Fx33 - LD B V{}", x);
                    // The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
                    let i = self.i;

                    // let val = self.v.read(x);
                    self.ram.write(i, self.v.read(x) / 100);
                    self.ram.write(i + 1, (self.v.read(x) % 100) / 10);
                    self.ram.write(i + 2, self.v.read(x) % 10);
                    println!(
                        "Stored BCD of {}: Hundreds: {}, Tens: {}, Ones: {}",
                        self.v.read(x),
                        self.ram.read(i),
                        self.ram.read(i + 1),
                        self.ram.read(i + 2),
                    );
                }
                0x55 => {
                    println!("Fx55 - LD [I] V{}", x);
                    // The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
                    let mut i = 0;
                    while i < x {
                        self.ram
                            .write(self.i.wrapping_add((i * 2) as u16), *self.v.read(i));
                        i += 1;
                    }
                }
                0x65 => {
                    println!("Fx65 - LD V{}, [I]", x);
                    // The interpreter reads values from memory starting at location I into registers V0 through Vx.
                    let mut i = 0;
                    while i < x {
                        self.v
                            .write(i, *self.ram.read(self.i.wrapping_add((i * 2) as u16)));
                        i += 1;
                    }
                }
                _ => println!("Invalid instruction: {}", self.instruction),
            },
            _ => println!("Invalid instruction: {}", self.instruction),
        }
    }
}
