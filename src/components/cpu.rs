use beep::beep;
use log::debug;
use rand::{rngs::ThreadRng, Rng};

use crate::{
    components::{Memory, VideoMemory},
    constants,
    fonts::CHIP8_FONTS,
    models::{InputBuffer, Registers, Stack, Timer},
    peripherals::Display,
    utils::read_bit_from_byte,
};

///
/// Program Counter
///
#[derive(Debug)]
pub struct ProgramCounter {
    pub address: u16,
}

impl ProgramCounter {
    pub fn new() -> Self {
        ProgramCounter {
            address: constants::PROGRAM_START_ADDR,
        }
    }

    pub fn next(&mut self) {
        self.address += constants::OPCODE_SIZE;
    }

    pub fn jump(&mut self, jump_addr: u16) {
        self.address = jump_addr;
    }

    pub fn clear(&mut self) {
        self.address = constants::PROGRAM_START_ADDR;
    }
}

///
/// CPU
///
pub struct Cpu<'a> {
    instruction: u16,
    ram: Memory,
    pub vram: VideoMemory,
    v: Registers,
    i: u16,
    pc: ProgramCounter,
    stack: Stack,
    delay_timer: Timer,
    sound_timer: Timer,
    rng: ThreadRng,
    pub keys: InputBuffer,
    pub display: &'a mut Display,
    pause: bool,
}

impl<'a> Cpu<'a> {
    pub fn new(display: &'a mut Display) -> Self {
        let mut cpu = Self {
            instruction: 0,
            ram: Memory::new(),
            vram: VideoMemory::new(),
            v: Registers::new(),
            i: 0,
            pc: ProgramCounter::new(),
            stack: Stack::new(),
            rng: rand::thread_rng(),
            delay_timer: Timer::new(0, 0.0, constants::TARGET_CLOCK_SPEED),
            sound_timer: Timer::new(0, 0.0, constants::TARGET_CLOCK_SPEED),
            keys: InputBuffer::new(),
            display,
            pause: false,
        };

        let mut i = constants::FONT_START_ADDR;
        for font_char in &CHIP8_FONTS {
            cpu.ram.write(i, *font_char);
            i += 1;
        }

        cpu
    }

    pub fn reset(&mut self) {
        self.instruction = 0;
        self.ram.clear();
        self.vram.clear();
        self.v.clear();
        self.i = 0;
        self.pc.clear();
        self.stack.clear();
        self.rng = rand::thread_rng();
        self.delay_timer.clear();
        self.sound_timer.clear();
        self.keys.clear();
    }

    pub fn load_program(&mut self, data: Vec<u8>) {
        let mut address = constants::PROGRAM_START_ADDR;
        for v in &data {
            self.ram.write(address, *v);
            address += 1;
        }
    }

    pub fn emulate_cycle(&mut self) {
        if self.wait_for_input() {
            return;
        }

        // CHIP-8 processes two instructions per clock cycle
        for _ in 0..1 {
            self.read_instruction();
            self.execute_instruction();
        }
    }

    pub fn update_timers(&mut self, delta_time: f32) {
        self.delay_timer.update(&delta_time);
        self.sound_timer.update(&delta_time);

        if self.sound_timer.get_value() > &0 {
            let _ = beep(constants::BEEP_FREQ_HZ);
        } else {
            let _ = beep(0);
        }
    }

    fn wait_for_input(&mut self) -> bool {
        if self.pause {
            for key in 0..15 as usize {
                if *self.keys.get(key) {
                    self.pause = false;
                }
            }
        }
        self.pause
    }

    fn read_instruction(&mut self) {
        self.instruction = (*self.ram.read(self.pc.address) as u16) << 8
            | (*self.ram.read(self.pc.address + 1) as u16);
    }

    fn execute_instruction(&mut self) {
        // Increment this now instead of in each `match` branch
        self.pc.next();

        let x = ((self.instruction & 0x0F00) >> 8).try_into().unwrap();
        let y = ((self.instruction & 0x00F0) >> 4).try_into().unwrap();

        let n = (self.instruction & 0x000F) as u8;
        let kk = (self.instruction & 0x00FF) as u8;
        let nnn = self.instruction & 0x0FFF;

        match self.instruction & 0xF000 {
            0x0 => match self.instruction & 0x00FF {
                0xE0 => {
                    debug!("00E0 - CLS");
                    // Clear the display.
                    self.vram.clear();
                    self.display.refresh(&self.vram);
                }
                0xEE => {
                    debug!("00EE - RET");
                    // The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
                    self.pc.jump(self.stack.pop());
                }
                _ => debug!("Invalid instruction: {}", self.instruction),
            },
            0x1000 => {
                debug!("1nnn - JP address: {}", nnn);
                // The interpreter sets the program counter to nnn.
                self.pc.jump(nnn);
            }
            0x2000 => {
                debug!("2nnn - CALL address: {}", nnn);
                // The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
                self.stack.push(self.pc.address);
                self.pc.jump(nnn);
            }
            0x3000 => {
                debug!("3xkk - SE V{} byte: {}", x, (kk));
                // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
                if self.v.read(x) == &kk {
                    self.pc.next();
                }
            }
            0x4000 => {
                debug!("4xkk - SNE V{} byte: {}", x, (kk));
                // The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
                if self.v.read(x) != &kk {
                    self.pc.next();
                }
            }
            0x5000 => match self.instruction & 0x000F {
                0x0 => {
                    debug!("5xy0 - SE V{}, V{}", x, y);
                    // The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
                    if self.v.read(x) == self.v.read(y) {
                        self.pc.next();
                    }
                }
                _ => debug!("Invalid instruction: {}", self.instruction),
            },
            0x6000 => {
                debug!("6xkk - LD V{} byte: {}", x, (kk));

                // The interpreter puts the value kk into register Vx.
                self.v.write(x, kk);
            }
            0x7000 => {
                debug!("7xkk - ADD V{} byte: {}", x, (kk));
                // Adds the value kk to the value of register Vx, then stores the result in Vx.
                self.v.write(x, self.v.read(x).wrapping_add(kk));
            }
            0x8000 => match self.instruction & 0x000F {
                0x0 => {
                    debug!("8xy0 - LD V{}, V{}", x, y);
                    // Stores the value of register Vy in register Vx.
                    self.v.write(x, *self.v.read(y));
                }
                0x1 => {
                    debug!("8xy1 - OR V{}, V{}", x, y);
                    // Set Vx = Vx OR Vy.
                    self.v.write(x, self.v.read(x) | self.v.read(y));
                }
                0x2 => {
                    debug!("8xy2 - AND V{}, V{}", x, y);
                    // Set Vx = Vx AND Vy.
                    self.v.write(x, self.v.read(x) & self.v.read(y));
                }
                0x3 => {
                    debug!("8xy3 - XOR V{}, V{}", x, y);
                    // Set Vx = Vx XOR Vy.
                    self.v.write(x, self.v.read(x) ^ self.v.read(y));
                }
                0x4 => {
                    debug!("8xy4 - ADD V{}, V{}", x, y);
                    // Set Vx = Vx + Vy, set VF = carry.
                    let sum = (self.v.read(x).wrapping_add(*self.v.read(y))) as u16;
                    self.v.write(x, sum.try_into().unwrap());
                    self.v.write(0xF, (sum > 0xFF) as u8);
                }
                0x5 => {
                    debug!("8xy5 - SUB V{}, V{}", x, y);
                    // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
                    self.v.write(0xF, (self.v.read(x) > self.v.read(y)) as u8);
                    self.v
                        .write(x, self.v.read(x).wrapping_sub(*self.v.read(y)));
                }
                0x6 => {
                    debug!("8xy6 - SHR V{}, V{}", x, y);
                    // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
                    if self.v.read(x) & 0b0000000000000001 == 1 {
                        self.v.write(0xF, 1);
                    } else {
                        self.v.write(0xF, 0);
                    }
                    self.v.write(x, self.v.read(x) / 2);
                }
                0x7 => {
                    debug!("8xy7 - SUBN V{}, V{}", x, y);
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
                    debug!("8xyE - SHL V{}, V{}", x, y);
                    // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
                    if self.v.read(x) >> 7 == 0b1 {
                        self.v.write(0xF, 1);
                    } else {
                        self.v.write(0xF, 0)
                    }
                    self.v.write(x, self.v.read(x) << 1);
                }
                _ => debug!("Invalid instruction: {}", self.instruction),
            },
            0x9000 => match self.instruction & 0x000F {
                0x0 => {
                    debug!("9xy0 - SNE V{}, V{}", x, y);
                    // The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
                    if self.v.read(x) != self.v.read(y) {
                        // self.next_instruction();
                        self.pc.next();
                    }
                }
                _ => debug!("Invalid instruction: {}", self.instruction),
            },
            0xA000 => {
                debug!("Annn - LD I address: {}", nnn);
                // The value of register I is set to nnn.
                self.i = nnn;
            }
            0xB000 => {
                debug!("Bnnn - JP V0 address: {}", nnn);
                // The program counter is set to nnn plus the value of V0.
                self.pc
                    .jump(self.v.read(0).wrapping_add(nnn.try_into().unwrap()) as u16);
            }
            0xC000 => {
                debug!("Cxkk - RND V{}, byte: {}", x, (kk));
                // The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx.
                let rnum: u8 = self.rng.gen();
                self.v.write(x, rnum & kk);
            }
            0xD000 => {
                debug!("Dxyn - DRW V{}, V{}, nibble: {}", x, y, n);
                // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.

                let vx = *self.v.read(x) as usize;
                let vy = *self.v.read(y) as usize;
                let len = self.instruction & 0x000F;
                let sprite_data = self.ram.read_slice(self.i, len);

                debug!(
                    "Draw the following Sprite with length {} at ({},{}): {:?}",
                    len, vx, vy, sprite_data
                );

                let mut sy: usize = 0;
                let mut sx: usize = 0;
                let mut collision = false;
                let mut did_draw = false;

                self.v.write(0xF, 0);

                while sy < len.try_into().unwrap() {
                    while sx < 8 {
                        let bit = read_bit_from_byte(&sprite_data[sy as usize], 7 - sx as u8);

                        let mut x = vx + sx;
                        if x > constants::SCREEN_WIDTH - 1 {
                            x = x % constants::SCREEN_WIDTH;
                        }

                        let mut y = vy + sy;
                        if y > constants::SCREEN_HEIGHT - 1 {
                            y = y % constants::SCREEN_HEIGHT;
                        }

                        let xor_res = self.vram.read(x, y) ^ bit;
                        if !collision && xor_res == 1 {
                            collision = true;
                            self.v.write(0xF, 1);
                        }

                        if !did_draw {
                            did_draw = true;
                        }

                        self.vram.write(x, y, xor_res);

                        sx += 1;
                    }
                    sx = 0;
                    sy += 1;
                }

                if did_draw {
                    self.display.refresh(&self.vram);
                }
            }
            0xE000 => match self.instruction & 0x00FF {
                0x9E => {
                    debug!("Ex9E - SKP V{}", x);
                    // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
                    debug!("Value of V{}: {}", x, self.v.read(x));
                    debug!("Keypad state: {:?}", self.keys.get_all());
                    let key = *self.v.read(x) as usize;
                    if *self.keys.get(key) {
                        self.pc.next();
                    }
                }
                0xA1 => {
                    debug!("ExA1 - SKNP V{}", x);
                    // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
                    if !self.keys.get(*self.v.read(x) as usize) {
                        self.pc.next();
                    }
                }
                _ => debug!("Invalid instruction: {}", self.instruction),
            },
            0xF000 => match self.instruction & 0x00FF {
                0x07 => {
                    debug!("Fx07 - LD V{}, DT", x);
                    // The value of DT is placed into Vx.
                    self.v.write(x, *self.delay_timer.get_value());
                }
                0x0A => {
                    debug!("Fx0A - LD V{}, K", x);
                    // All execution stops until a key is pressed, then the value of that key is stored in Vx.
                    if !self.pause {
                        self.pause = true;
                    }
                }
                0x15 => {
                    debug!("Fx15 - LD DT V{}", x);
                    // DT is set equal to the value of Vx.
                    self.delay_timer.set_value(*self.v.read(x));
                }
                0x18 => {
                    debug!("Fx18 - LD ST V{}", x);
                    // ST is set equal to the value of Vx.
                    self.sound_timer.set_value(*self.v.read(x));
                }
                0x1E => {
                    debug!("Fx1E - ADD I V{}", x);
                    // The values of I and Vx are added, and the results are stored in I.
                    self.i = self.i.wrapping_add(*self.v.read(x) as u16);
                }
                0x29 => {
                    debug!("Fx29 - LD F V{}", x);
                    // The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx.
                    self.i = constants::FONT_START_ADDR.wrapping_add((*self.v.read(x) * 5) as u16);
                }
                0x33 => {
                    debug!("Fx33 - LD B V{}", x);
                    // The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
                    let i = self.i;
                    let val = *self.v.read(x);

                    self.ram.write(i, val / 100);
                    self.ram.write(i + 1, (val % 100) / 10);
                    self.ram.write(i + 2, val % 10);

                    debug!(
                        "Stored BCD of {}: Hundreds: {}, Tens: {}, Ones: {}",
                        val,
                        self.ram.read(i),
                        self.ram.read(i + 1),
                        self.ram.read(i + 2),
                    );
                }
                0x55 => {
                    debug!("Fx55 - LD [I] V{}", x);
                    // The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
                    for i in 0..(x + 1) as u16 {
                        self.ram.write(self.i + i, *self.v.read(i as u8));
                    }
                }
                0x65 => {
                    debug!("Fx65 - LD V{}, [I]", x);
                    // The interpreter reads values from memory starting at location I into registers V0 through Vx.
                    for i in 0..(x + 1) as u16 {
                        self.v.write(i as u8, *self.ram.read(self.i + i));
                    }
                }
                _ => debug!("Invalid instruction: {}", self.instruction),
            },
            _ => debug!("Invalid instruction: {}", self.instruction),
        }
    }
}
