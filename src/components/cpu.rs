use log::debug;
use rand::{rngs::ThreadRng, Rng};

use crate::{
    components::{Memory, VideoMemory},
    constants,
    fonts::CHIP8_FONTS,
    models::{InputBuffer, ProgramCounter, Registers, Stack},
    utils::read_bit_from_byte,
};

///
/// CPU
///
pub struct Cpu {
    pub instruction: u16,
    ram: Memory,
    pub vram: VideoMemory,
    pub vram_changed: bool,
    v: Registers,
    i: u16,
    pc: ProgramCounter,
    stack: Stack,
    delay_timer: u8,
    sound_timer: u8,
    rng: ThreadRng,
    pub keys: InputBuffer,
    key_register: u8,
    waiting_for_key: bool,
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Self {
            instruction: 0,
            ram: Memory::new(),
            vram: VideoMemory::new(),
            vram_changed: false,
            v: Registers::new(),
            i: 0,
            pc: ProgramCounter::new(),
            stack: Stack::new(),
            rng: rand::thread_rng(),
            delay_timer: 0,
            sound_timer: 0,
            keys: InputBuffer::new(),
            key_register: 0,
            waiting_for_key: false,
        };

        let mut i = constants::FONT_START_ADDR;
        for font_char in &CHIP8_FONTS {
            cpu.ram.write(i, *font_char);
            i += 1;
        }

        cpu
    }

    pub fn reset(&mut self) {
        self.ram.clear();
        self.vram.clear();
        self.v.clear();
        self.i = 0;
        self.pc.reset();
        self.stack.clear();
        self.rng = rand::thread_rng();
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.keys.clear();
    }

    #[inline]
    pub fn load_program(&mut self, data: Vec<u8>) {
        let mut address = constants::PROGRAM_START_ADDR;
        for v in &data {
            self.ram.write(address, *v);
            address += 1;
        }
    }

    #[inline]
    pub fn tick(&mut self) {
        self.vram_changed = false;

        if self.waiting_for_key {
            for key in 0..15 as usize {
                if *self.keys.get(key) {
                    self.waiting_for_key = false;
                    self.v.write(self.key_register, key as u8);
                    self.key_register = 0;
                }
            }
            if !self.waiting_for_key {
                return;
            }
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }

        // CHIP-8 processes two instructions per clock cycle
        for _ in 0..1 {
            let opcode = self.read_instruction();
            self.execute_instruction(opcode);
        }
    }

    #[inline]
    fn read_instruction(&mut self) -> u16 {
        (*self.ram.read(self.pc.address) as u16) << 8 | (*self.ram.read(self.pc.address + 1) as u16)
    }

    fn execute_instruction(&mut self, instruction: u16) {
        self.pc.next();

        let opcode = (
            ((instruction & 0xF000) >> 12) as u8,
            ((instruction & 0x0F00) >> 8) as u8,
            ((instruction & 0x00F0) >> 4) as u8,
            (instruction & 0x000F) as u8,
        );

        let x = opcode.1.try_into().unwrap();
        let y = opcode.2.try_into().unwrap();

        let n: u8 = opcode.3.try_into().unwrap();
        let kk = (instruction & 0x00FF) as u8;
        let nnn = instruction & 0x0FFF;

        match opcode {
            (0x0, 0x0, 0xe, 0x0) => self.cls(),
            (0x0, 0x0, 0xe, 0xe) => self.ret(),
            (0x1, _, _, _) => self.jp(nnn),
            (0x2, _, _, _) => self.call(nnn),
            (0x3, _, _, _) => self.se_vx(x, kk),
            (0x4, _, _, _) => self.sne_vx(x, kk),
            (0x5, _, _, 0x0) => self.se_vx_vy(x, y),
            (0x6, _, _, _) => self.ld_vx(x, kk),
            (0x7, _, _, _) => self.add_vx(x, kk),
            (0x8, _, _, 0x0) => self.ld_vx_vy(x, y),
            (0x8, _, _, 0x1) => self.or_vx_vy(x, y),
            (0x8, _, _, 0x2) => self.and_vx_vy(x, y),
            (0x8, _, _, 0x3) => self.xor_vx_vy(x, y),
            (0x8, _, _, 0x4) => self.add_vx_vy(x, y),
            (0x8, _, _, 0x5) => self.sub_vx_vy(x, y),
            (0x8, _, _, 0x6) => self.shr_vx_vy(x, y),
            (0x8, _, _, 0x7) => self.subn_vx_vy(x, y),
            (0x8, _, _, 0xe) => self.shl_vx_vy(x, y),
            (0x9, _, _, 0x0) => self.sne_vx_vy(x, y),
            (0xa, _, _, _) => self.ld_i(nnn),
            (0xb, _, _, _) => self.jp_v0(nnn),
            (0xc, _, _, _) => self.rnd_vx(x, kk),
            (0xd, _, _, _) => self.drw_vx_vy(x, y, n),
            (0xe, _, 0x9, 0xe) => self.skp_vx(x),
            (0xe, _, 0xa, 0x1) => self.sknp_vx(x),
            (0xf, _, 0x0, 0x7) => self.ld_vx_dt(x),
            (0xf, _, 0x0, 0xa) => self.ld_vx_k(x),
            (0xf, _, 0x1, 0x5) => self.ld_dt_vx(x),
            (0xf, _, 0x1, 0x8) => self.ld_st_vx(x),
            (0xf, _, 0x1, 0xe) => self.add_i_vx(x),
            (0xf, _, 0x2, 0x9) => self.ld_f_vx(x),
            (0xf, _, 0x3, 0x3) => self.ld_b_vx(x),
            (0xf, _, 0x5, 0x5) => self.ld_i_vx(x),
            (0xf, _, 0x6, 0x5) => self.ld_vx_i(x),
            _ => debug!("Invalid opcode: {:?}", opcode),
        }
    }

    ///
    /// 0x00E0 - CLS
    ///
    /// Clear the display.
    pub fn cls(&mut self) {
        debug!("00E0 - CLS");
        self.vram.clear();
        self.vram_changed = true;
    }

    ///
    /// 0x00EE - RET
    ///
    /// The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
    ///
    pub fn ret(&mut self) {
        debug!("00EE - RET");
        self.pc.jump(self.stack.pop());
    }

    ///
    /// 0x1nnn - JP
    ///
    /// The interpreter sets the program counter to nnn.
    ///
    pub fn jp(&mut self, address: u16) {
        debug!("1nnn - JP address: {}", address);
        self.pc.jump(address);
    }

    ///
    /// 0x2nnn - CALL
    ///
    /// The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
    ///
    pub fn call(&mut self, address: u16) {
        debug!("2nnn - CALL address: {}", address);
        // self.pc.next();
        self.stack.push(self.pc.address);
        self.pc.jump(address);
    }

    ///
    /// 0x3xkk - SE Vx
    ///
    /// The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    ///
    pub fn se_vx(&mut self, vx: u8, kk: u8) {
        debug!("3xkk - SE V{} byte: {}", vx, (kk));
        if self.v.read(vx) == &kk {
            self.pc.next();
        }
    }

    ///
    /// 4xkk - SNE Vx
    ///
    /// // The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    ///
    pub fn sne_vx(&mut self, vx: u8, kk: u8) {
        debug!("4xkk - SNE V{} byte: {}", vx, (kk));
        if self.v.read(vx) != &kk {
            self.pc.next();
        }
    }

    ///
    /// 5xy0 - SE Vx, Vy
    ///
    /// The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    ///
    pub fn se_vx_vy(&mut self, vx: u8, vy: u8) {
        debug!("5xy0 - SE V{}, V{}", vx, vy);
        if self.v.read(vx) == self.v.read(vy) {
            self.pc.next();
        }
    }

    ///
    /// 6xkk - LD Vx
    ///
    /// The interpreter puts the value kk into register Vx.
    ///
    pub fn ld_vx(&mut self, vx: u8, kk: u8) {
        debug!("6xkk - LD V{} byte: {}", vx, (kk));
        self.v.write(vx, kk);
    }

    ///
    /// 7xkk - ADD Vx
    ///
    /// Adds the value kk to the value of register Vx, then stores the result in Vx.
    ///
    pub fn add_vx(&mut self, vx: u8, kk: u8) {
        debug!("7xkk - ADD V{} byte: {}", vx, (kk));
        // TODO: does this wrap?
        self.v.write(vx, self.v.read(vx).wrapping_add(kk));
    }

    ///
    /// 8xy0 - LD Vx, Vy
    ///
    /// Stores the value of register Vy in register Vx.
    ///
    pub fn ld_vx_vy(&mut self, vx: u8, vy: u8) {
        debug!("8xy0 - LD V{}, V{}", vx, vy);
        self.v.write(vx, *self.v.read(vy));
    }

    ///
    /// 8xy1 - OR Vx, Vy
    ///
    /// Set Vx = Vx OR Vy.
    ///
    pub fn or_vx_vy(&mut self, vx: u8, vy: u8) {
        debug!("8xy1 - OR V{}, V{}", vx, vy);
        self.v.write(vx, self.v.read(vx) | self.v.read(vy));
    }

    ///
    /// 8xy2 - AND Vx, Vy
    ///
    /// Set Vx = Vx AND Vy.
    ///
    pub fn and_vx_vy(&mut self, vx: u8, vy: u8) {
        debug!("8xy2 - AND V{}, V{}", vx, vy);
        self.v.write(vx, self.v.read(vx) & self.v.read(vy));
    }

    ///
    /// 8xy3 - XOR Vx, Vy
    ///
    /// Set Vx = Vx XOR Vy.
    ///
    pub fn xor_vx_vy(&mut self, vx: u8, vy: u8) {
        debug!("8xy3 - XOR V{}, V{}", vx, vy);
        self.v.write(vx, self.v.read(vx) ^ self.v.read(vy));
    }

    ///
    /// 8xy4 - ADD Vx, Vy
    ///
    /// Set Vx = Vx + Vy, set VF = carry
    ///
    pub fn add_vx_vy(&mut self, vx: u8, vy: u8) {
        debug!("8xy4 - ADD V{}, V{}", vx, vy);
        let sum = (self.v.read(vx).wrapping_add(*self.v.read(vy))) as u16;
        self.v.write(vx, sum.try_into().unwrap());
        self.v.write(0xF, (sum > 0xFF) as u8);
    }

    ///
    /// 8xy5 - SUB Vx, Vy
    ///
    /// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
    pub fn sub_vx_vy(&mut self, vx: u8, vy: u8) {
        debug!("8xy5 - SUB V{}, V{}", vx, vy);
        self.v.write(0xF, (self.v.read(vx) > self.v.read(vy)) as u8);
        self.v
            .write(vx, self.v.read(vx).wrapping_sub(*self.v.read(vy)));
    }

    ///
    /// 8xy6 - SHR Vx, Vy
    ///
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
    ///
    pub fn shr_vx_vy(&mut self, vx: u8, vy: u8) {
        debug!("8xy6 - SHR V{}, V{}", vx, vy);
        //--- OLD: If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2. ---

        // Store the value of register VY shifted right one bit in register VX
        // Set register VF to the least significant bit prior to the shift
        // VY is unchanged
        if self.v.read(vx) & 0b0000000000000001 == 1 {
            self.v.write(0xF, 1);
        } else {
            self.v.write(0xF, 0);
        }
        self.v.write(vx, self.v.read(vy) >> 1);
        // self.v.write(vx, self.v.read(vx) / 2);
    }

    ///
    /// 8xy7 - SUBN Vx, Vy
    ///
    /// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
    ///
    pub fn subn_vx_vy(&mut self, vx: u8, vy: u8) {
        debug!("8xy7 - SUBN V{}, V{}", vx, vy);
        if self.v.read(vy) > self.v.read(vy) {
            self.v.write(vx, self.v.read(vy) - self.v.read(vx));
            self.v.write(0xF, 1);
        } else {
            self.v.write(vx, 0);
            self.v.write(0xF, 0);
        }
    }

    ///
    /// 8xyE - SHL Vx, Vy
    ///
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
    ///
    pub fn shl_vx_vy(&mut self, vx: u8, vy: u8) {
        debug!("8xyE - SHL V{}, V{}", vx, vy);
        //--- OLD: If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.---

        // Store the value of register VY shifted left one bit in register VX
        // Set register VF to the most significant bit prior to the shift
        // VY is unchanged
        if self.v.read(vx) >> 7 == 0b1 {
            self.v.write(0xF, 1);
        } else {
            self.v.write(0xF, 0)
        }
        // self.v.write(vx, self.v.read(vx) << 1);
        self.v.write(vx, self.v.read(vy) << 1);
    }

    ///
    /// 9xy0 - SNE Vx, Vy
    ///
    /// The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
    ///
    pub fn sne_vx_vy(&mut self, vx: u8, vy: u8) {
        debug!("9xy0 - SNE V{}, V{}", vx, vy);
        if self.v.read(vx) != self.v.read(vy) {
            // self.next_instruction();
            self.pc.next();
        }
    }

    ///
    /// Annn - LD I
    ///
    /// I is set to nnn.
    ///
    pub fn ld_i(&mut self, address: u16) {
        debug!("Annn - LD I address: {}", address);
        self.i = address;
    }

    ///
    /// Bnnn - JP V0
    ///
    /// The program counter is set to nnn plus the value of V0.
    ///
    pub fn jp_v0(&mut self, address: u16) {
        debug!("Bnnn - JP V0 address: {}", address);
        self.pc.jump(*self.v.read(0) as u16 + address);
    }

    ///
    /// Cxkk - RND Vx
    ///
    /// The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx.
    ///
    pub fn rnd_vx(&mut self, vx: u8, value: u8) {
        debug!("Cxkk - RND V{}, byte: {}", vx, value);
        let rnum: u8 = self.rng.gen();
        self.v.write(vx, rnum & value);
    }

    ///
    /// Dxyn - DRW Vx, Vy
    ///
    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    ///
    pub fn drw_vx_vy(&mut self, vx: u8, vy: u8, sprite_len: u8) {
        debug!(
            "D{}{}{} - DRW V{}, V{}, {}",
            vx, vy, sprite_len, vx, vy, sprite_len
        );
        // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.

        let mut x = *self.v.read(vx) as usize;
        let mut y = *self.v.read(vy) as usize;

        if x > constants::SCREEN_WIDTH - 1 {
            x = x % constants::SCREEN_WIDTH;
        }
        if y > constants::SCREEN_HEIGHT - 1 {
            y = y % constants::SCREEN_HEIGHT;
        }

        // let len = self.instruction & 0x000F;
        let sprite_data = self.ram.read_slice(self.i, sprite_len.try_into().unwrap());

        debug!(
            "Draw the following Sprite with length {} at ({},{}): {:?}",
            sprite_len, x, y, sprite_data
        );

        if self.i >= 0x200 {
            let sprite = self.vram.read_sprite((self.i, vx, vy));

            match sprite {
                Some(coords) => {
                    debug!(
                        "Stored sprite position for address {} and V{},V{}: {:?}",
                        self.i, vx, vy, coords
                    );
                    if coords.0 == x && coords.1 == y {
                        return;
                    } else {
                        let mut sy: usize = 0;
                        let mut sx: usize = 0;
                        while sy < sprite_len.try_into().unwrap() {
                            while sx < 8 {
                                let bit =
                                    read_bit_from_byte(&sprite_data[sy as usize], 7 - sx as u8);

                                let vx = coords.0 + sx;
                                let vy = coords.1 + sy;

                                if vx <= constants::SCREEN_WIDTH - 1
                                    && vy <= constants::SCREEN_HEIGHT - 1
                                    && *bit == 1
                                {
                                    // Need to keep track of fonts drawn to the screen and re-draw them when erased in this loop
                                    // Or we keep track of each pixel's history?

                                    // Create two arrays for pixels being turned on and off
                                    //  instead of overwriting the current VRAM w/both
                                    // Then make the "off" pixels fade out in the Display component

                                    // How do we know what this pixel was BEFORE we drew the sprite we're erasing?
                                    self.vram.write(vx, vy, self.vram.read(vx, vy) ^ bit);
                                }

                                sx += 1;
                            }
                            sx = 0;
                            sy += 1;
                        }
                        // while sy < len.try_into().unwrap() {
                        //     let vy = coords.1 + sy;
                        //     if vy <= constants::SCREEN_HEIGHT - 1 {
                        //         while sx < 8 {
                        //             let vx = coords.0 + sx;
                        //             if vx <= constants::SCREEN_WIDTH - 1 {
                        //                 debug!(
                        //                     "Curr ({},{}): {}",
                        //                     vx,
                        //                     vy,
                        //                     self.vram.read(vx, vy),
                        //                 );
                        //                 // What was this space BEFORE the sprite we're about to erase?
                        //                 self.vram.write(vx, vy, 0);
                        //             }
                        //             sx += 1;
                        //         }
                        //     }
                        //     sx = 0;
                        //     sy += 1;
                        // }
                    }
                }
                None => {}
            }
        }

        debug!("Drawing sprite");

        let mut sy: usize = 0;
        let mut sx: usize = 0;

        self.v.write(0xF, 0);

        while sy < sprite_len.try_into().unwrap() {
            while sx < 8 {
                let bit = read_bit_from_byte(&sprite_data[sy as usize], 7 - sx as u8);

                let mut vx = x + sx;
                let mut vy = y + sy;

                if vx > constants::SCREEN_WIDTH - 1 {
                    vx = vx % constants::SCREEN_WIDTH;
                }
                if vy > constants::SCREEN_HEIGHT - 1 {
                    vy = vy % constants::SCREEN_HEIGHT;
                }

                if *bit == 1 {
                    let xor_res = self.vram.read(vx, vy) ^ bit;

                    if xor_res == 1 {
                        self.v.write(0xF, 1);
                        self.vram_changed = true;
                    }

                    self.vram.write(vx, vy, xor_res);
                }

                sx += 1;
            }
            sx = 0;
            sy += 1;
        }

        // if self.i >= 0x200 {
        self.vram.write_sprite((self.i, vx, vy), x, y);
        // }
    }

    ///
    /// Ex9E - SKP Vx
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
    ///
    pub fn skp_vx(&mut self, vx: u8) {
        debug!("Ex9E - SKP V{}", vx);
        let key = *self.v.read(vx) as usize;
        if *self.keys.get(key) {
            self.pc.next();
        }
    }

    ///
    /// ExA1 - SKNP Vx
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
    ///
    pub fn sknp_vx(&mut self, vx: u8) {
        debug!("ExA1 - SKNP V{}", vx);
        if !self.keys.get(*self.v.read(vx) as usize) {
            self.pc.next();
        }
    }

    ///
    /// Fx07 - LD Vx, DT
    ///
    /// The delay timer value is placed into Vx.
    ///
    pub fn ld_vx_dt(&mut self, vx: u8) {
        debug!("Fx07 - LD V{}, DT", vx);
        self.v.write(vx, self.delay_timer);
        // self.v.write(vx, *self.delay_timer.get_value());
    }

    ///
    /// Fx0A - LD Vx, K
    ///
    /// All execution stops until a key is pressed, then the value of that key is stored in Vx.
    ///
    pub fn ld_vx_k(&mut self, vx: u8) {
        debug!("Fx0A - LD V{}, K", vx);
        if !self.waiting_for_key {
            self.key_register = vx;
            self.waiting_for_key = true;
        }
    }

    ///
    /// Fx15 - LD DT Vx
    ///
    /// Delay timer is set equal to the value of Vx.
    ///
    pub fn ld_dt_vx(&mut self, vx: u8) {
        debug!("Fx15 - LD DT V{}", vx);
        self.delay_timer = *self.v.read(vx);
        // self.delay_timer.set_value(*self.v.read(vx));
    }

    ///
    /// Fx18 - LD ST Vx
    ///
    /// Sound timer is set equal to the value of Vx.
    ///
    pub fn ld_st_vx(&mut self, vx: u8) {
        debug!("Fx18 - LD ST V{}", vx);
        self.sound_timer = *self.v.read(vx);
        // self.sound_timer.set_value(*self.v.read(vx));
    }

    ///
    /// Fx1E - ADD I Vx
    ///
    /// The values of I and Vx are added, and the results are stored in I.
    ///
    pub fn add_i_vx(&mut self, vx: u8) {
        debug!("Fx1E - ADD I V{}", vx);
        self.i = self.i.wrapping_add(*self.v.read(vx) as u16);
    }

    ///
    /// Fx29 - LD F Vx
    ///
    /// The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx.
    ///
    pub fn ld_f_vx(&mut self, vx: u8) {
        debug!("Fx29 - LD F V{}", vx);
        self.i = constants::FONT_START_ADDR.wrapping_add((*self.v.read(vx) * 5) as u16);
    }

    ///
    /// Fx33 - LD B Vx
    ///
    /// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
    ///
    pub fn ld_b_vx(&mut self, vx: u8) {
        debug!("Fx33 - LD B V{}", vx);
        let i = self.i;
        let val = *self.v.read(vx);

        self.ram.write(i, val / 100);
        self.ram.write(i + 1, (val % 100) / 10);
        self.ram.write(i + 2, val % 10);
    }

    ///
    /// Fx55 - LD I Vx
    ///
    /// CHIP-8:
    ///     The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I. I is then incremented.
    /// SCHIP 1.1:
    ///     The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
    ///
    pub fn ld_i_vx(&mut self, vx: u8) {
        debug!("Fx55 - LD [I] V{}", vx);
        // if self.quirks_mode {
        for i in 0..(vx + 1) as u16 {
            self.ram.write(self.i + i, *self.v.read(i as u8));
        }
        // } else {
        //     // TODO: implement the original command
        //     for i in 0..(vx + 1) as u16 {
        //         self.ram.write(self.i + i, *self.v.read(i as u8));
        //     }
        // }
    }

    ///
    /// Fx65 - LD Vx, I
    ///
    /// The interpreter reads values from memory starting at location I into registers V0 through Vx.
    ///
    pub fn ld_vx_i(&mut self, vx: u8) {
        debug!("Fx65 - LD V{}, [I]", vx);
        for i in 0..(vx + 1) as u16 {
            self.v.write(i as u8, *self.ram.read(self.i + i));
        }
    }
}
